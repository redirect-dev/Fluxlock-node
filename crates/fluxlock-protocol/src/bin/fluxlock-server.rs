use tiny_http::{Server, Response, Header, Method};


fn validate(identity: &str, epoch: i32) -> (bool, &'static str) {
    match identity {
        "ID-1000" => {
            if epoch == 0 {
                (true, "identity is current")
            } else {
                (false, "identity expired")
            }
        }
        "ID-1001" => {
            if epoch == 1 {
                (true, "identity is current")
            } else {
                (false, "identity expired")
            }
        }
        _ => (false, "unknown identity"),
    }
}

fn extract(body: &str, key: &str) -> String {
    body.split(&format!("\"{}\":", key))
        .nth(1)
        .and_then(|s| s.split(&[',', '}'][..]).next())
        .map(|s| s.trim().trim_matches('"').to_string())
        .unwrap_or_default()
}

fn json_response(valid: bool, identity: &str, epoch: i32, reason: &str) -> Response<std::io::Cursor<Vec<u8>>> {
    let json = format!(
        r#"{{
  "valid": {},
  "identity": "{}",
  "epoch": {},
  "reason": "{}"
}}"#,
        valid, identity, epoch, reason
    );

    Response::from_string(json)
        .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
}

fn main() {
    let server = Server::http("0.0.0.0:8080").unwrap();

    println!("🚀 Fluxlock API + UI running at http://localhost:8080");

    for mut request in server.incoming_requests() {
        let url = request.url().to_string();

        // =========================
        // POST /validate (REAL API)
        // =========================
        if request.method() == &Method::Post && url.starts_with("/validate") {
            let mut body = String::new();
            request.as_reader().read_to_string(&mut body).unwrap();

            let identity = extract(&body, "identity");
            let epoch = extract(&body, "epoch").parse::<i32>().unwrap_or(-1);

            let (valid, reason) = validate(&identity, epoch);

            let response = json_response(valid, &identity, epoch, reason);
            request.respond(response).unwrap();
        }

        // =========================
        // GET /validate (browser)
        // =========================
        else if url.starts_with("/validate") {
            let query = url.split('?').nth(1).unwrap_or("");
            let mut identity = "";
            let mut epoch = -1;

            for pair in query.split('&') {
                let mut parts = pair.split('=');
                let key = parts.next().unwrap_or("");
                let value = parts.next().unwrap_or("");

                if key == "identity" {
                    identity = value;
                } else if key == "epoch" {
                    epoch = value.parse().unwrap_or(-1);
                }
            }

            let (valid, reason) = validate(identity, epoch);

            let response = json_response(valid, identity, epoch, reason);
            request.respond(response).unwrap();
        }

        // =========================
        // UI
        // =========================
        else {
            let html = r###"
            <html>
            <head>
                <title>Fluxlock UI</title>
                <style>
                    body {
                        font-family: Arial, sans-serif;
                        margin: 40px;
                        background-color: #0f172a;
                        color: #e2e8f0;
                    }
                    input, button {
                        padding: 10px;
                        margin-top: 5px;
                        margin-bottom: 15px;
                        font-size: 16px;
                    }
                    button {
                        background-color: #3b82f6;
                        color: white;
                        border: none;
                        cursor: pointer;
                    }
                    button:hover {
                        background-color: #2563eb;
                    }
                    #result {
                        margin-top: 20px;
                        font-size: 18px;
                        font-family: monospace;
                        white-space: pre;
                    }
                </style>
            </head>
            <body>
                <h1>Fluxlock Identity Validator</h1>

                <p>Validate identity against epoch (time-bound enforcement)</p>

                <label>Identity:</label><br/>
                <input id="identity" value="ID-1000"/><br/>

                <label>Epoch:</label><br/>
                <input id="epoch" value="1"/><br/>

                <button onclick="validate()">Validate</button>

                <pre id="result"></pre>

                <script>
                    function validate() {
                        const id = document.getElementById('identity').value;
                        const epoch = parseInt(document.getElementById('epoch').value);

                        fetch('/validate', {
                            method: 'POST',
                            headers: {
                                'Content-Type': 'application/json'
                            },
                            body: JSON.stringify({
                                identity: id,
                                epoch: epoch
                            })
                        })
                        .then(res => res.json())
                        .then(data => {
                            document.getElementById('result').innerText =
                                JSON.stringify(data, null, 2);
                        });
                    }
                </script>
            </body>
            </html>
            "###;

            request.respond(Response::from_string(html)).unwrap();
        }
    }
}
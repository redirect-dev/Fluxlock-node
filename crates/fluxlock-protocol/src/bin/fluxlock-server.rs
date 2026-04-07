use tiny_http::{Server, Response, Header};
use std::str;

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

fn main() {
    let server = Server::http("0.0.0.0:8080").unwrap();

    println!("🚀 Fluxlock API + UI running at http://localhost:8080");

    for request in server.incoming_requests() {
        let url = request.url().to_string();

        if url.starts_with("/validate") {
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

            let json = format!(
                r#"{{
    "valid": {},
    "identity": "{}",
    "epoch": {},
    "reason": "{}"
}}"#,
                valid, identity, epoch, reason
            );

            let response = Response::from_string(json)
                .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());

            request.respond(response).unwrap();
        } else {
            let html = r###"
            <html>
            <head>
                <title>Fluxlock UI</title>
            </head>
            <body>
                <h1>Fluxlock Identity Validator</h1>

                <input id="identity" value="ID-1000"/>
                <input id="epoch" value="1"/>

                <button onclick="validate()">Validate</button>

                <pre id="result"></pre>

                <script>
                    function validate() {
                        const id = document.getElementById('identity').value;
                        const epoch = document.getElementById('epoch').value;

                        fetch(`/validate?identity=${id}&epoch=${epoch}`)
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
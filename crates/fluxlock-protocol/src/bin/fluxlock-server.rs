use tiny_http::{Server, Response};
use std::str;

fn validate(identity: &str, epoch: i32) -> bool {
    match identity {
        "ID-1000" => epoch == 0,
        "ID-1001" => epoch == 1,
        _ => false,
    }
}

fn main() {
    let server = Server::http("0.0.0.0:8080").unwrap();

    println!("🚀 Fluxlock UI running at http://localhost:8080");

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

            let result = validate(identity, epoch);

            let response = if result {
                "VALID"
            } else {
                "INVALID"
            };

            let res = Response::from_string(response);
            request.respond(res).unwrap();
        } else {
            let html = r#"
            <html>
            <head>
                <title>Fluxlock UI</title>
            </head>
            <body>
                <h1>🔐 Fluxlock Identity Validator</h1>

                <label>Identity:</label><br/>
                <input id="identity" value="ID-1000"/><br/><br/>

                <label>Epoch:</label><br/>
                <input id="epoch" value="1"/><br/><br/>

                <button onclick="validate()">Validate</button>

                <h2 id="result"></h2>

                <script>
                    function validate() {
                        const id = document.getElementById('identity').value;
                        const epoch = document.getElementById('epoch').value;

                        fetch(`/validate?identity=${id}&epoch=${epoch}`)
                            .then(res => res.text())
                            .then(data => {
                                const result = document.getElementById('result');
                                if (data === "VALID") {
                                    result.innerHTML = "✅ VALID";
                                } else {
                                    result.innerHTML = "❌ INVALID";
                                }
                            });
                    }
                </script>
            </body>
            </html>
            "#;

            let res = Response::from_string(html);
            request.respond(res).unwrap();
        }
    }
}
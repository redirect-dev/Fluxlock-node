use tiny_http::{Server, Response, Header};
use serde::Serialize;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Serialize, Clone)]
struct Validator {
    id: usize,
    trust: f64,
    influence: f64,
    status: String,
    behavior_score: f64,
    drift_score: f64,
}

fn create_network() -> Vec<Validator> {
    let mut rng = rand::thread_rng();

    (0..20)
        .map(|i| Validator {
            id: i,
            trust: 90.0 + rng.gen_range(-3.0..3.0),
            influence: 65.0 + rng.gen_range(-5.0..5.0),
            status: "normal".to_string(),
            behavior_score: 92.0 + rng.gen_range(-2.0..2.0),
            drift_score: 0.0,
        })
        .collect()
}

fn update_network(validators: &mut Vec<Validator>) {
    let mut rng = rand::thread_rng();

    println!("🔥 ENGINE LOOP RUNNING");

    for v in validators.iter_mut() {
        let is_attacker = v.id == 19;

        // 🔥 EXPECTED BEHAVIOR BASELINE
        let expected_behavior = 90.0;

        // 🔥 ACTUAL BEHAVIOR
        let actual_behavior = if is_attacker {
            // STEALTH ATTACK CURVE
            if v.drift_score < 30.0 {
                rng.gen_range(85.0..95.0) // looks normal
            } else if v.drift_score < 70.0 {
                rng.gen_range(60.0..80.0) // subtle drift
            } else {
                rng.gen_range(10.0..40.0) // collapse phase
            }
        } else {
            rng.gen_range(88.0..98.0)
        };

        // Smooth update
        v.behavior_score = (v.behavior_score * 0.85) + (actual_behavior * 0.15);

        // 🔥 DEVIATION-BASED DRIFT (THIS IS THE FIX)
        let deviation = expected_behavior - v.behavior_score;

        if is_attacker {
            v.drift_score += deviation.max(0.0) * 0.8;
        } else {
            v.drift_score += deviation.max(0.0) * 0.2;
            v.drift_score *= 0.95; // decay for honest nodes
        }

        // Clamp drift
        if v.drift_score > 150.0 {
            v.drift_score = 150.0;
        }

        if is_attacker {
            println!(
                "🚨 ATTACKER → drift: {:.2}, behavior: {:.2}, trust: {:.2}",
                v.drift_score, v.behavior_score, v.trust
            );
        }

        // 🔥 TRUST RESPONDS TO DRIFT (NOT JUST EVENTS)
        if v.drift_score > 20.0 {
            v.trust -= 4.0;
        } else {
            v.trust += 0.2;
        }

        v.trust = v.trust.clamp(0.0, 100.0);

        // STATUS
        if v.drift_score > 100.0 && is_attacker {
            v.status = "attacked".to_string();
            v.trust = 5.0;
        } else if v.drift_score > 40.0 {
            v.status = "drifting".to_string();
        } else {
            v.status = "normal".to_string();
        }

        // INFLUENCE
        if v.trust < 30.0 {
            v.influence = 5.0;
        } else {
            v.influence = v.influence * 0.7 + v.trust * 0.3;
        }
    }
}

fn main() {
    let server = Server::http("0.0.0.0:8080").unwrap();

    println!("🚀 Fluxlock Engine ACTIVE on http://localhost:8080");

    let network = Arc::new(Mutex::new(create_network()));

    let sim = Arc::clone(&network);
    thread::spawn(move || loop {
        {
            let mut net = sim.lock().unwrap();
            update_network(&mut net);
        }
        thread::sleep(Duration::from_millis(1000));
    });

    for request in server.incoming_requests() {
        if request.url() == "/simulation" {
            let net = network.lock().unwrap();
            let json = serde_json::to_string(&*net).unwrap();

            let response = Response::from_string(json)
                .with_header(
                    Header::from_bytes(
                        &b"Content-Type"[..],
                        &b"application/json"[..],
                    ).unwrap(),
                )
                .with_header(
                    Header::from_bytes(
                        &b"Access-Control-Allow-Origin"[..],
                        &b"*"[..],
                    ).unwrap(),
                );

            let _ = request.respond(response);
        } else {
            let _ = request.respond(Response::from_string("Not Found"));
        }
    }
}
use std::env;
use std::fs::OpenOptions;
use std::io::Write;

// --------------------
// CONFIGURATION
// --------------------
const MAX_TICKS: u32 = 10;
const MAX_KEY_AGE: u32 = 2;

// Node that will intentionally skip key rotation
const ADVERSARIAL_NODE: &str = "node3";

// --------------------
// MAIN
// --------------------
fn main() {
    let args: Vec<String> = env::args().collect();
    let node_id = args.get(1).expect("Node ID required").clone();

    let log_file = format!("{}_log.csv", node_id);
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&log_file)
        .expect("Unable to create log file");

    // CSV header
    writeln!(
        file,
        "tick,node,decision,weighted_decision,trust,key_age"
    )
    .unwrap();

    // --------------------
    // INITIAL STATE
    // --------------------
    let mut trust: f64 = 0.75;
    let mut key_age: u32 = 0;

    // --------------------
    // SIMULATION LOOP
    // --------------------
    for tick in 0..MAX_TICKS {
        // Simulated decision (deterministic)
        let decision = (tick + node_id.len() as u32) % 2;

        // --------------------
        // KEY ROTATION LOGIC
        // --------------------
        key_age += 1;

        if key_age >= MAX_KEY_AGE {
            if node_id != ADVERSARIAL_NODE {
                // Honest node rotates key
                key_age = 0;
            } else {
                // Adversarial node skips rotation
                key_age += 1;
            }
        }

        // --------------------
        // TRUST UPDATE
        // --------------------
        if key_age > MAX_KEY_AGE {
            // Penalize stale keys
            trust -= 0.10;
        } else {
            // Reward healthy behavior slightly
            trust += 0.05;
        }

        // Clamp trust
        if trust > 1.0 {
            trust = 1.0;
        }
        if trust < 0.0 {
            trust = 0.0;
        }

        let weighted_decision = (decision as f64 * trust).round() as u32;

        // --------------------
        // LOG OUTPUT
        // --------------------
        writeln!(
            file,
            "{},{},{},{},{:.3},{}",
            tick, node_id, decision, weighted_decision, trust, key_age
        )
        .unwrap();

        println!(
            "{} | tick {} | decision {} | trust {:.3} | key_age {}",
            node_id, tick, decision, trust, key_age
        );
    }

    println!("{} FINISHED", node_id);
}

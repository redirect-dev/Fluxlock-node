use std::env;
use std::fs::{OpenOptions};
use std::io::Write;
use std::thread;
use std::time::Duration;

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

// ===============================
// Phase 6 Reputation Parameters
// ===============================
const REP_MIN: f64 = 0.10;
const REP_MAX: f64 = 1.00;

const PENALTY_MULTIPLIER: f64 = 0.85;
const RECOVERY_RATE: f64 = 0.01;

const CONSENSUS_TOLERANCE: i32 = 10;
const PENALTY_THRESHOLD: i32 = 20;

const MAX_TICKS: u32 = 10;

// ===============================

fn main() {
    let args: Vec<String> = env::args().collect();
    let node_name = args.get(1).expect("Node name required");

    let adversarial_nodes = vec!["node3"];

    let mut rng = StdRng::from_entropy();
    let mut reputation: f64 = 1.0;

    let log_file = format!("{}_log.csv", node_name);
    let mut log = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&log_file)
        .unwrap();

    writeln!(
        log,
        "tick,node,entropy,consensus,delta,reputation"
    ).unwrap();

    for tick in 0..MAX_TICKS {
        // -------------------------------
        // Entropy generation
        // -------------------------------
        let mut entropy: i32 = rng.gen_range(60..100);

        if adversarial_nodes.contains(&node_name.as_str()) {
            entropy += rng.gen_range(20..80); // spike
        }

        // -------------------------------
        // Read network consensus
        // -------------------------------
        let consensus: i32 = read_consensus();

        let delta = entropy - consensus;

        // ===============================
        // Phase 6 Reputation Update
        // ===============================
        if delta.abs() > PENALTY_THRESHOLD {
            reputation *= PENALTY_MULTIPLIER;
        } else if delta.abs() <= CONSENSUS_TOLERANCE {
            reputation += RECOVERY_RATE;
        }

        // Clamp bounds
        if reputation > REP_MAX {
            reputation = REP_MAX;
        }
        if reputation < REP_MIN {
            reputation = REP_MIN;
        }

        // -------------------------------
        // Write network state
        // -------------------------------
        append_network_state(tick, node_name, entropy);

        // -------------------------------
        // Log output
        // -------------------------------
        writeln!(
            log,
            "{},{},{},{},{},{}",
            tick,
            node_name,
            entropy,
            consensus,
            delta,
            reputation
        ).unwrap();

        println!(
            "{} | tick {} | entropy {} | consensus {} | rep {:.3}",
            node_name, tick, entropy, consensus, reputation
        );

        thread::sleep(Duration::from_millis(300));
    }

    println!("{} FINISHED", node_name);
}

// ===============================
// Helper Functions
// ===============================

fn append_network_state(tick: u32, node: &str, entropy: i32) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("network_state.txt")
        .unwrap();

    writeln!(file, "{}:{}:{}", tick, node, entropy).unwrap();
}

fn read_consensus() -> i32 {
    let content = std::fs::read_to_string("network_state.txt").unwrap_or_default();
    let mut values = vec![];

    for line in content.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 3 {
            if let Ok(v) = parts[2].parse::<i32>() {
                values.push(v);
            }
        }
    }

    if values.is_empty() {
        80
    } else {
        values.iter().sum::<i32>() / values.len() as i32
    }
}

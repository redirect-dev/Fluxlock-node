use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::env;
use std::fs::{OpenOptions, read_to_string};
use std::io::Write;
use std::thread;
use std::time::Duration;

const TICKS: usize = 10;
const PENALTY_THRESHOLD: i32 = 20;
const RECOVERY_THRESHOLD: i32 = 10;
const PENALTY_MULTIPLIER: f64 = 0.85;
const RECOVERY_RATE: f64 = 0.01;
const MIN_REP: f64 = 0.10;
const MAX_REP: f64 = 1.00;

fn main() {
    let args: Vec<String> = env::args().collect();
    let node_id = args.get(1).expect("node id required").clone();

    let mut rng = StdRng::from_entropy();
    let mut reputation: f64 = 1.0;

    let adversarial = node_id == "node3";

    for tick in 0..TICKS {
        // Generate entropy
        let mut entropy: i32 = rng.gen_range(70..100);
        if adversarial && tick % 2 == 0 {
            entropy += rng.gen_range(40..80);
        }

        // Write entropy + rep to shared state
        {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("network_state.txt")
                .unwrap();
            writeln!(file, "{},{},{:.3}", tick, entropy, reputation).unwrap();
        }

        thread::sleep(Duration::from_millis(50));

        // Read network state
        let state = read_to_string("network_state.txt").unwrap_or_default();

        let mut weighted_sum = 0.0;
        let mut rep_sum = 0.0;

        for line in state.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() != 3 {
                continue;
            }
            let t: usize = parts[0].parse().unwrap_or(999);
            if t != tick {
                continue;
            }
            let ent: f64 = parts[1].parse().unwrap_or(0.0);
            let rep: f64 = parts[2].parse().unwrap_or(0.0);

            weighted_sum += ent * rep;
            rep_sum += rep;
        }

        let consensus = if rep_sum > 0.0 {
            (weighted_sum / rep_sum) as i32
        } else {
            entropy
        };

        let delta = (entropy - consensus).abs();

        // Update reputation
        if delta > PENALTY_THRESHOLD {
            reputation *= PENALTY_MULTIPLIER;
        } else if delta <= RECOVERY_THRESHOLD {
            reputation += RECOVERY_RATE;
        }

        reputation = reputation.clamp(MIN_REP, MAX_REP);

        println!(
            "{} | tick {} | entropy {} | weighted_consensus {} | rep {:.3}",
            node_id, tick, entropy, consensus, reputation
        );

        thread::sleep(Duration::from_millis(200));
    }

    println!("{} FINISHED", node_id);
}

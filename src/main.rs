use std::env;
use std::fs::{OpenOptions, read_to_string};
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

const TICKS: i64 = 10;
const CONSENSUS_WEIGHT: f64 = 0.25;
const MIN_ENTROPY: i64 = 0;
const MAX_ENTROPY: i64 = 255;

fn clamp(v: i64) -> i64 {
    if v < MIN_ENTROPY {
        MIN_ENTROPY
    } else if v > MAX_ENTROPY {
        MAX_ENTROPY
    } else {
        v
    }
}

fn median(values: &mut Vec<i64>) -> i64 {
    if values.is_empty() {
        return 0;
    }
    values.sort();
    values[values.len() / 2]
}

fn parse_network_state() -> Vec<i64> {
    let mut values = Vec::new();

    if let Ok(contents) = read_to_string("network_state.txt") {
        for line in contents.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 3 {
                if let Ok(v) = parts[2].parse::<i64>() {
                    values.push(v);
                }
            }
        }
    }

    values
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let node_id = args.get(1).cloned().unwrap_or("nodeX".to_string());

    // adversarial nodes (for later fault-tolerance tests)
    let adversarial_nodes = vec!["node3"];

    // initial entropy seeded by node id
    let mut entropy: i64 =
        64 + (node_id.bytes().map(|b| b as i64).sum::<i64>() % 64);

    let log_name = format!("{}_log.csv", node_id);
    let mut log = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_name)
        .unwrap();

    writeln!(log, "tick,node,entropy,delta_from_median").unwrap();

    for tick in 0..TICKS {
        let mut network_values = parse_network_state();
        let consensus = median(&mut network_values);

        // natural drift
        let mut drift = (tick * 7 + entropy) % 11 - 5;

        // adversarial spike
        if adversarial_nodes.contains(&node_id.as_str()) && tick % 3 == 0 {
            drift += 50;
        }

        // consensus pull
        let pull = ((consensus - entropy) as f64 * CONSENSUS_WEIGHT) as i64;

        entropy = clamp(entropy + drift + pull);

        // write to shared network state
        let mut net = OpenOptions::new()
            .create(true)
            .append(true)
            .open("network_state.txt")
            .unwrap();

        writeln!(net, "{}:{}:{}", tick, node_id, entropy).unwrap();

        let delta = entropy - consensus;
        writeln!(log, "{},{},{},{}", tick, node_id, entropy, delta).unwrap();

        sleep(Duration::from_millis(300));
    }

    writeln!(log, "FINISHED").unwrap();
}

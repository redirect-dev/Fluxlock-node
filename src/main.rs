use std::env;
use std::fs::{OpenOptions, read_to_string};
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

const MIN_ENTROPY: i64 = 32;
const MAX_ENTROPY: i64 = 255;
const CONSENSUS_WEIGHT: f64 = 0.15; // soft influence
const TICKS: usize = 10;

fn clamp(value: i64) -> i64 {
    if value < MIN_ENTROPY { MIN_ENTROPY }
    else if value > MAX_ENTROPY { MAX_ENTROPY }
    else { value }
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

// compute median
fn median(values: &mut Vec<i64>) -> i64 {
    if values.is_empty() { return 0; }
    values.sort_unstable();
    let mid = values.len() / 2;
    if values.len() % 2 == 0 {
        (values[mid - 1] + values[mid]) / 2
    } else {
        values[mid]
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let node_id = args.get(1).cloned().unwrap_or("nodeX".to_string());

    // initial local entropy
    let mut entropy: i64 = 64 + (node_id.bytes().map(|b| b as i64).sum::<i64>() % 64);

    // node log
    let log_name = format!("{}_log.txt", node_id);
    let mut log = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_name)
        .unwrap();

    for tick in 0..TICKS {
        let mut network_values = parse_network_state();
        let cons = median(&mut network_values);

        // local drift
        let drift = (tick as i64 * 7 + entropy) % 11 - 5;

        // soft consensus pull
        let consensus_pull = ((cons - entropy) as f64 * CONSENSUS_WEIGHT) as i64;

        entropy = clamp(entropy + drift + consensus_pull);

        // log to network_state.txt
        let line = format!("{}:{}:{}\n", tick, node_id, entropy);
        let mut net = OpenOptions::new()
            .create(true)
            .append(true)
            .open("network_state.txt")
            .unwrap();
        net.write_all(line.as_bytes()).unwrap();

        // log with consensus info
        let delta = entropy - cons;
        writeln!(log, "{},{},{},{}", tick, node_id, entropy, delta).unwrap();

        sleep(Duration::from_millis(300));
    }

    writeln!(log, "FINISHED").unwrap();
}

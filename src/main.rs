use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    // Get the node ID from command line arguments
    let args: Vec<String> = env::args().collect();
    let node_id = if args.len() > 1 {
        &args[1]
    } else {
        "node0"
    };

    // Use node ID to generate a unique seed
    let seed: u64 = node_id.bytes().map(|b| b as u64).sum();
    let mut rng = StdRng::seed_from_u64(seed);

    // Open a log file per node
    let log_filename = format!("{}_log.txt", node_id);
    let mut log_file = File::create(&log_filename)
        .expect("Failed to create log file");

    println!("Starting FluxLock node: {}", node_id);
    writeln!(log_file, "Starting FluxLock node: {}", node_id).unwrap();

    // Simulate 10 ticks of key evolution
    let mut key = rng.gen_range(1..100);
    for tick in 0..10 {
        key = key ^ rng.gen_range(1..100); // simple evolving key
        println!("Tick {}: key = {}", tick, key);
        writeln!(log_file, "Tick {}: key = {}", tick, key).unwrap();
    }

    println!("Node {} finished.", node_id);
    writeln!(log_file, "Node {} finished.", node_id).unwrap();
}

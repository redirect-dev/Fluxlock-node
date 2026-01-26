use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufRead, BufReader};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Get the node ID from command line arguments
    let args: Vec<String> = env::args().collect();
    let node_id = if args.len() > 1 { &args[1] } else { "node0" };

    // Use node ID to generate a unique seed
    let seed: u64 = node_id.bytes().map(|b| b as u64).sum();
    let mut rng = StdRng::seed_from_u64(seed);

    // Open a log file per node
    let log_filename = format!("{}_log.txt", node_id);
    let mut log_file = File::create(&log_filename).expect("Failed to create log file");

    println!("Starting FluxLock network node: {}", node_id);
    writeln!(log_file, "Starting FluxLock network node: {}", node_id).unwrap();

    let network_file = "network_state.txt";

    let mut key = rng.gen_range(1..100);
    for tick in 0..10 {
        // Evolve key locally
        key = key ^ rng.gen_range(1..100);

        // Append current key to network state file
        {
            let mut nf = OpenOptions::new()
                .create(true)
                .append(true)
                .open(network_file)
                .expect("Failed to open network file");
            writeln!(nf, "{}:{}:{}", tick, node_id, key).unwrap();
        }

        // Read all keys from network file
        let file = File::open(network_file).expect("Failed to read network file");
        let reader = BufReader::new(file);
        let mut network_snapshot = vec![];
        for line in reader.lines() {
            if let Ok(l) = line {
                network_snapshot.push(l);
            }
        }

        // Log local and network snapshot
        println!("Tick {}: node {} key = {}, network state: {:?}", tick, node_id, key, network_snapshot);
        writeln!(log_file, "Tick {}: node {} key = {}, network state: {:?}", tick, node_id, key, network_snapshot).unwrap();

        // Small delay to simulate network ticks
        sleep(Duration::from_millis(200));
    }

    println!("Node {} finished.", node_id);
    writeln!(log_file, "Node {} finished.", node_id).unwrap();
}

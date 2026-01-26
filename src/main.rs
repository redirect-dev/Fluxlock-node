use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufRead, BufReader};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Get node ID from command line arguments
    let args: Vec<String> = env::args().collect();
    let node_id = if args.len() > 1 { &args[1] } else { "node0" };

    // Unique seed per node
    let seed: u64 = node_id.bytes().map(|b| b as u64).sum();
    let mut rng = StdRng::seed_from_u64(seed);

    // Log file per node
    let log_filename = format!("{}_log.txt", node_id);
    let mut log_file = File::create(&log_filename).expect("Failed to create log file");

    println!("Starting FluxLock consensus node: {}", node_id);
    writeln!(log_file, "Starting FluxLock consensus node: {}", node_id).unwrap();

    let network_file = "network_state.txt";

    let mut key = rng.gen_range(1..100);
    for tick in 0..10 {
        // Evolve key locally
        key = key ^ rng.gen_range(1..100);

        // Read current network state
        let mut network_snapshot = vec![];
        if let Ok(file) = File::open(network_file) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(l) = line {
                    let parts: Vec<&str> = l.split(':').collect();
                    if parts.len() == 3 && parts[1] != node_id {
                        if let Ok(remote_key) = parts[2].parse::<u64>() {
                            network_snapshot.push(remote_key);
                        }
                    }
                }
            }
        }

        // Simple consensus adjustment: average of other nodes
        if !network_snapshot.is_empty() {
            let avg: u64 = network_snapshot.iter().sum::<u64>() / network_snapshot.len() as u64;
            key = (key + avg) / 2; // move slightly toward network average
        }

        // Append current key to network file
        {
            let mut nf = OpenOptions::new()
                .create(true)
                .append(true)
                .open(network_file)
                .expect("Failed to open network file");
            writeln!(nf, "{}:{}:{}", tick, node_id, key).unwrap();
        }

        // Log local key and network snapshot
        println!("Tick {}: node {} key = {}, network snapshot: {:?}", tick, node_id, key, network_snapshot);
        writeln!(log_file, "Tick {}: node {} key = {}, network snapshot: {:?}", tick, node_id, key, network_snapshot).unwrap();

        // Small delay to simulate network tick
        sleep(Duration::from_millis(200));
    }

    println!("Node {} finished.", node_id);
    writeln!(log_file, "Node {} finished.", node_id).unwrap();
}

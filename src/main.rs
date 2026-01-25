use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;

// Simple evolving lock/key simulation
fn main() {
    let node_id = std::env::args().nth(1).unwrap_or("node0".to_string());
    let mut key: u64 = 0;

    // Open log file for this node
    let log_file_path = format!("{}_log.txt", node_id);
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)
        .expect("Unable to open log file");

    println!("Starting FluxLock node: {}", node_id);
    writeln!(log_file, "Starting FluxLock node: {}", node_id).unwrap();

    // Tick loop: evolve key every second
    for tick in 0..20 {
        key = key.wrapping_add(tick * 7 + 13); // simple evolving function
        let log_line = format!("Tick {}: key = {}", tick, key);
        println!("{}", log_line);
        writeln!(log_file, "{}", log_line).unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    println!("Node {} finished.", node_id);
    writeln!(log_file, "Node {} finished.", node_id).unwrap();
}


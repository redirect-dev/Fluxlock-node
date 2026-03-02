use std::fs::File;
use std::io::Read;

use fluxlock_core::TickLog;
use fluxlock_replay::ReplayEngine;

fn main() {
    println!("Fluxlock Replay Starting...");

    // Load tick log
    let mut file = File::open("tick_log.json")
        .expect("Failed to open tick_log.json");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read tick_log.json");

    let tick_log: TickLog =
        serde_json::from_str(&contents)
            .expect("Failed to deserialize TickLog");

    println!(
        "Loaded {} tick records. Beginning replay...",
        tick_log.records.len()
    );

    // Replay + verify
    ReplayEngine::replay_and_verify(&tick_log);

    println!("Replay completed successfully. No divergence detected.");
}

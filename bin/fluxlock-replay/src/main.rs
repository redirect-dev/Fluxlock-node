use fluxlock_core::TickLog;
use fluxlock_replay::replay;
use std::fs;

fn main() {
    println!("Fluxlock Replay Starting...");

    // Load tick log produced by fluxlock-node
    let data = fs::read_to_string("tick_log.json")
        .expect("Failed to read tick_log.json");

    let tick_log: TickLog =
        serde_json::from_str(&data).expect("Failed to parse tick log");

    println!(
        "Loaded {} tick records. Beginning replay...",
        tick_log.records.len()
    );

    match replay(&tick_log) {
        Ok(_) => {
            println!("Replay completed successfully. No divergence detected.");
        }
        Err(err) => {
            println!("REPLAY FAILURE:");
            println!("{}", err);
        }
    }
}

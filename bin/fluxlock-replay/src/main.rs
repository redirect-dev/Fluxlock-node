use fluxlock_core::{TickLog, EngineCompositeState};
use fluxlock_engine::{apply_tick, hash_state};
use std::fs;

fn main() {
    println!("Fluxlock Replay Starting...");

    let data = fs::read_to_string("tick_log.json")
        .expect("Unable to read tick_log.json");

    let tick_log: TickLog =
        serde_json::from_str(&data).expect("Failed to parse tick log");

    println!(
        "Loaded {} tick records. Beginning replay...",
        tick_log.records.len()
    );

    let mut state = EngineCompositeState::new();
    let mut expected_parent = String::from("GENESIS");

    for record in &tick_log.records {

        if record.parent_hash != expected_parent {
            panic!("Seal chain break");
        }

        apply_tick(&mut state, &record.input, record.tick_index)
            .expect("Detinistic transition failure");

        let observed_hash = hash_state(&state);

        if observed_hash != record.state_hash {
            panic!(
                "STATE DIVERGENCE at tick {}\nExpected: {}\nObserved: {}",
                record.tick_index,
                record.state_hash,
                observed_hash
            );
        }

        expected_parent = record.state_hash.clone();
    }

    println!("Replay completed successfully. Seal-chain verified.");
}

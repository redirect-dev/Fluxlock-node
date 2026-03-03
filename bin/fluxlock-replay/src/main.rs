use std::fs;

use fluxlock_core::{EngineCompositeState, TickLog};
use fluxlock_engine::{apply_tick, hash_state};

fn main() {
    println!("Fluxlock Consensus Replay Starting...");

    let data = fs::read_to_string("tick_log.json")
        .expect("Unable to read tick_log.json");

    let tick_log: TickLog =
        serde_json::from_str(&data)
            .expect("Invalid JSON");

    println!(
        "Loaded {} tick records. Beginning dual-node replay...",
        tick_log.records.len()
    );

    // Two independent nodes
    let mut node_a = EngineCompositeState::new();
    let mut node_b = EngineCompositeState::new();

    for record in &tick_log.records {

        apply_tick(&mut node_a, &record.input, record.tick_index)
            .expect("Node A transition failure");

        apply_tick(&mut node_b, &record.input, record.tick_index)
            .expect("Node B transition failure");

        let hash_a = hash_state(&node_a);
        let hash_b = hash_state(&node_b);

        if hash_a != hash_b {
            panic!(
                "CONSENSUS DIVERGENCE at tick {}\nNode A: {}\nNode B: {}",
                record.tick_index,
                hash_a,
                hash_b
            );
        }

        if hash_a != record.state_hash {
            panic!(
                "STATE HASH MISMATCH at tick {}\nExpected: {}\nObserved: {}",
                record.tick_index,
                record.state_hash,
                hash_a
            );
        }
    }

    println!("Consensus replay completed successfully.");
    println!("All nodes converged. Determinism confirmed.");
}

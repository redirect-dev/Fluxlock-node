use fluxlock_core::{EngineCompositeState, TickLog};
use fluxlock_engine::FluxlockEngine;

use blake3;
use serde_json;
use hex;

fn main() {
    println!("Fluxlock Replay Starting...");

    let json = std::fs::read_to_string("tick_log.json")
        .expect("tick_log.json not found");

    let tick_log: TickLog =
        serde_json::from_str(&json).expect("Failed to parse tick log");

    println!(
        "Loaded {} tick records. Beginning replay...",
        tick_log.records.len()
    );

    let mut engine = FluxlockEngine;
    let mut state = EngineCompositeState::new();
    let mut parent_hash = String::from("GENESIS");

    for record in tick_log.records.iter() {
        let prev_state = state.clone();

        engine
            .execute_tick(&mut state, &prev_state)
            .expect("INVARIANT VIOLATION DURING REPLAY");

        // --- State hash verification (always enforced) ---
        let state_bytes =
            serde_json::to_vec(&state).expect("State serialization failed");

        let observed_state_hash =
            hex::encode(blake3::hash(&state_bytes).as_bytes());

        if observed_state_hash != record.state_hash {
            panic!(
                "STATE DIVERGENCE at tick {}\nExpected: {}\nObserved: {}",
                record.tick_index,
                record.state_hash,
                observed_state_hash
            );
        }

        // --- Chain hash verification (skip GENESIS tick) ---
        if record.tick_index > 1 {
            let mut hasher = blake3::Hasher::new();
            hasher.update(parent_hash.as_bytes());
            hasher.update(blake3::hash(&state_bytes).as_bytes());

            let observed_chain_hash =
                hex::encode(hasher.finalize().as_bytes());

            if observed_chain_hash != record.parent_hash {
                panic!(
                    "CHAIN DIVERGENCE at tick {}\nExpected parent: {}\nObserved: {}",
                    record.tick_index,
                    record.parent_hash,
                    observed_chain_hash
                );
            }

            parent_hash = observed_chain_hash;
        } else {
            // Genesis tick initializes the chain
            parent_hash = record.parent_hash.clone();
        }
    }

    println!("Replay completed successfully. No divergence detected.");
}

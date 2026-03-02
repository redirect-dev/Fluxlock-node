use fluxlock_core::{EngineCompositeState, TickLog, TickRecord};
use fluxlock_engine::FluxlockEngine;

use blake3;
use serde_json;
use hex;

fn main() {
    println!("Fluxlock Node Starting...");

    // ⬅️ MUST be mutable because execute_tick mutates engine internals
    let mut engine = FluxlockEngine;
    let mut state = EngineCompositeState::new();
    let mut tick_log = TickLog::new();

    // Genesis parent hash
    let mut parent_hash = String::from("GENESIS");

    for tick in 1..=25 {
        let prev_state = state.clone();

        engine
            .execute_tick(&mut state, &prev_state)
            .expect("INVARIANT VIOLATION");

        // --- Hash state deterministically ---
        let state_bytes =
            serde_json::to_vec(&state).expect("State serialization failed");

        let state_hash = blake3::hash(&state_bytes);
        let state_hash_hex = hex::encode(state_hash.as_bytes());

        // --- Chain hash = H(parent || state_hash) ---
        let mut hasher = blake3::Hasher::new();
        hasher.update(parent_hash.as_bytes());
        hasher.update(state_hash.as_bytes());
        let chain_hash = hasher.finalize();
        let chain_hash_hex = hex::encode(chain_hash.as_bytes());

        println!(
            "Tick {} | trust={:.3} | lock={} | stage={} | seal={}",
            tick,
            state.trust.trust_score,
            state.lock.level,
            state.lifecycle.stage,
            &chain_hash_hex[..8]
        );

        tick_log.records.push(TickRecord {
            tick_index: tick,
            state: state.clone(),
            parent_hash: parent_hash.clone(),
            state_hash: state_hash_hex,
            signature: None, // Phase 2: unsigned hash chain
        });

        parent_hash = chain_hash_hex;
    }

    std::fs::write(
        "tick_log.json",
        serde_json::to_string_pretty(&tick_log).unwrap(),
    )
    .expect("Failed to write tick_log.json");

    println!("Fluxlock Node Finished.");
}

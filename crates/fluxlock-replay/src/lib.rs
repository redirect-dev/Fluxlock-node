use fluxlock_core::{EngineCompositeState, TickLog};
use fluxlock_engine::FluxlockEngine;
use blake3;

/// Canonical state hash (must match node)
fn hash_state(state: &EngineCompositeState) -> String {
    let bytes = serde_json::to_vec(state).expect("state serialization failed");
    blake3::hash(&bytes).to_hex()[..8].to_string()
}

pub fn replay_and_verify(tick_log: &TickLog) {
    println!("Loaded {} tick records. Beginning replay...", tick_log.records.len());

    let mut engine = FluxlockEngine;
    let mut state = EngineCompositeState::new();

    let mut expected_parent = String::from("GENESIS");

    for record in &tick_log.records {
        // 1️⃣ Parent hash continuity
        if record.parent_hash != expected_parent {
            panic!(
                "SEAL CHAIN BREAK at tick {}:\nExpected parent {}\nFound parent {}",
                record.tick_index,
                expected_parent,
                record.parent_hash
            );
        }

        // 2️⃣ Deterministic re-execution
        let prev = state.clone();
        engine
            .execute_tick(&prev, &mut state)
            .expect("Engine invariant violation during replay");

        // Recovery reset must match node rules
        if state.trust.trust_score <= 0.0 {
            let preserved_lock = state.lock.level;
            state = EngineCompositeState::new();
            state.lock.level = preserved_lock;
            state.trust.trust_score = 25.0;
            state.recovery.grace_ticks_remaining = 5;
        }

        // 3️⃣ State hash integrity
        let observed_hash = hash_state(&state);
        if observed_hash != record.state_hash {
            panic!(
                "STATE HASH DIVERGENCE at tick {}:\nExpected {}\nObserved {}",
                record.tick_index,
                record.state_hash,
                observed_hash
            );
        }

        expected_parent = record.state_hash.clone();
    }

    println!("Replay completed successfully. Seal-chain verified.");
}

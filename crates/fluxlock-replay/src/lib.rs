use fluxlock_core::{EngineCompositeState, TickLog};
use fluxlock_engine::FluxlockEngine;
use blake3;

/// Canonical deterministic state hash (consensus critical)
fn hash_state(state: &EngineCompositeState) -> String {
    let mut hasher = blake3::Hasher::new();

    // --- TrustState
    hasher.update(&state.trust.trust_score.to_le_bytes());

    // --- LifecycleState
    hasher.update(&[state.lifecycle.stage]);

    // --- LockState
    hasher.update(&[state.lock.level]);

    // --- RecoveryState
    hasher.update(&[state.recovery.is_recovering as u8]);
    hasher.update(&state.recovery.recovery_ticks.to_le_bytes());
    hasher.update(&state.recovery.grace_ticks_remaining.to_le_bytes());

    // --- KeyState (use deterministic internal hash)
    let key_hash = state.key_state.hash();
    hasher.update(&key_hash);

    hasher.finalize().to_hex()[..8].to_string()
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

        // Recovery reset logic
        if state.trust.trust_score <= 0.0 {
            let preserved_lock = state.lock.level;
            state = EngineCompositeState::new();
            state.lock.level = preserved_lock;
            state.trust.trust_score = 25.0;
            state.recovery.grace_ticks_remaining = 5;
        }

        // 3️⃣ Deterministic state hash integrity
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

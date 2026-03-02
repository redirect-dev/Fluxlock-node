use fluxlock_core::{EngineCompositeState, TickLog};
use fluxlock_engine::FluxlockEngine;

/// Replays a TickLog and verifies deterministic execution
pub fn replay(log: &TickLog) -> Result<(), String> {
    let mut engine = FluxlockEngine::new();

    // 🔥 Enable divergence for testing
    engine.divergence_mode = true;

    // IMPORTANT: use canonical constructor
    let mut state = EngineCompositeState::new();

    for (i, record) in log.records.iter().enumerate() {
        engine.execute_tick(&mut state);

        if state != record.state {
            return Err(format!(
                "DIVERGENCE at tick {}:\nExpected: {:?}\nObserved: {:?}",
                i + 1,
                record.state,
                state
            ));
        }
    }

    Ok(())
}

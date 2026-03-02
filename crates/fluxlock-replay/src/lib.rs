use fluxlock_core::{
    EngineCompositeState,
    TickLog,
};
use fluxlock_engine::FluxlockEngine;

/// ============================
/// Replay Engine
/// ============================
pub struct ReplayEngine;

impl ReplayEngine {
    /// Replay a tick log and verify deterministic state evolution
    pub fn replay_and_verify(log: &TickLog) {
        let mut engine = FluxlockEngine::new();
        let mut state = EngineCompositeState::new();

        for record in &log.records {
            engine.execute_tick(&mut state);

            if state != record.state {
                panic!(
                    "Replay divergence at tick {}.\nExpected: {:?}\nObserved: {:?}",
                    record.tick,
                    record.state,
                    state
                );
            }
        }
    }
}

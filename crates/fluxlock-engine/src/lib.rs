use fluxlock_core::{
    EngineCompositeState,
};

/// ============================
/// Fluxlock Engine
/// ============================
pub struct FluxlockEngine;

impl FluxlockEngine {
    /// Create a new engine instance
    pub fn new() -> Self {
        FluxlockEngine
    }

    /// Execute one deterministic protocol tick
    pub fn execute_tick(&mut self, state: &mut EngineCompositeState) {
        // ----------------------------
        // Trust decay (baseline pressure)
        // ----------------------------
        if !state.recovery.is_recovering {
            state.trust.decay(5.0);
        }

        // ----------------------------
        // Lifecycle transitions
        // ----------------------------
        if state.trust.trust_score < 50.0 {
            state.lifecycle.stage = 1; // Degraded
        }

        // ----------------------------
        // Lock escalation
        // ----------------------------
        if state.trust.trust_score < 25.0 {
            state.lock.level = 1; // Restricted
        }

        // ----------------------------
        // Enter recovery
        // ----------------------------
        if state.trust.trust_score < 20.0 && !state.recovery.is_recovering {
            state.recovery.is_recovering = true;
            state.recovery.recovery_ticks = 0;
        }

        // ----------------------------
        // Recovery execution
        // ----------------------------
        if state.recovery.is_recovering {
            state.recovery.recovery_ticks += 1;

            // Recovery completes after 4 ticks
            if state.recovery.recovery_ticks >= 4 {
                state.recovery.is_recovering = false;
                state.recovery.recovery_ticks = 0;
                state.recovery.grace_ticks_remaining = 5;

                // Partial trust restoration
                state.trust.trust_score = 25.0;

                // Reset defensive posture
                state.lifecycle.stage = 0;
                state.lock.level = 0;
            }
        }

        // ----------------------------
        // Grace window (slow trust recovery)
        // ----------------------------
        if state.recovery.grace_ticks_remaining > 0 {
            state.recovery.grace_ticks_remaining -= 1;
            state.trust.recover(3.0);
        }
    }
}

use fluxlock_core::EngineCompositeState;

#[derive(Default)]
pub struct FluxlockEngine {
    /// Used ONLY for divergence testing
    pub divergence_mode: bool,
}

impl FluxlockEngine {
    pub fn new() -> Self {
        Self {
            divergence_mode: false,
        }
    }

    pub fn execute_tick(&mut self, state: &mut EngineCompositeState) {
        // Base trust decay
        state.trust.trust_score -= 5.0;

        // 🔥 INTENTIONAL DIVERGENCE
        // This changes behavior ONLY when enabled
        if self.divergence_mode && state.trust.trust_score == 75.0 {
            state.trust.trust_score -= 1.0; // subtle, deterministic fault
        }

        // Lifecycle transition
        if state.trust.trust_score <= 50.0 {
            state.lifecycle.stage = 1;
        }

        // Lock escalation
        if state.trust.trust_score <= 20.0 {
            state.lock.level = 1;
        }

        // Recovery trigger
        if state.trust.trust_score <= 15.0 {
            state.recovery.is_recovering = true;
            state.recovery.recovery_ticks += 1;
        }

        // Recovery completion
        if state.recovery.recovery_ticks >= 3 {
            state.trust.trust_score = 25.0;
            state.lifecycle.stage = 0;
            state.lock.level = 0;
            state.recovery.is_recovering = false;
            state.recovery.recovery_ticks = 0;
            state.recovery.grace_ticks_remaining = 4;
        }

        // Grace window logic
        if state.recovery.grace_ticks_remaining > 0 {
            state.recovery.grace_ticks_remaining -= 1;
            state.trust.trust_score += 2.0;
        }
    }
}

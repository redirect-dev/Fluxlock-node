use fluxlock_core::EngineCompositeState;

pub struct FluxlockEngine;

impl FluxlockEngine {

    pub fn execute_tick(state: &mut EngineCompositeState) {

        // ======================================================
        // RECOVERY GRACE WINDOW (ACTIVE)
        // ======================================================
        if state.recovery.grace_ticks_remaining > 0 {

            // Non-linear trust rebuild:
            // Faster early, slower as trust improves
            let rebuild_amount = if state.trust.trust_score < 35.0 {
                4.0
            } else {
                2.0
            };

            if state.trust.trust_score < 50.0 {
                state.trust.trust_score += rebuild_amount;

                if state.trust.trust_score > 50.0 {
                    state.trust.trust_score = 50.0;
                }
            }

            state.recovery.grace_ticks_remaining -= 1;

            // During grace:
            // - No decay
            // - No lock escalation
            // - No lifecycle downgrade
            return;
        }

        // ======================================================
        // TRUST DECAY
        // ======================================================
        state.trust.decay(5.0);

        // ======================================================
        // LIFECYCLE DEGRADATION
        // ======================================================
        if state.trust.trust_score < 50.0 {
            state.lifecycle.stage = 1;
        }

        // ======================================================
        // LOCK ESCALATION
        // ======================================================
        if state.trust.trust_score < 30.0 {
            state.lock.level = 1;
        }

        // ======================================================
        // RECOVERY ENTRY
        // ======================================================
        if state.lock.level > 0 && state.trust.trust_score < 20.0 {
            state.recovery.is_recovering = true;
            state.recovery.recovery_ticks += 1;
        }

        // ======================================================
        // RECOVERY COMPLETION
        // ======================================================
        if state.recovery.is_recovering && state.recovery.recovery_ticks >= 5 {

            // Unlock node
            state.lock.level = 0;
            state.lifecycle.stage = 0;

            // Exit recovery mode
            state.recovery.is_recovering = false;
            state.recovery.recovery_ticks = 0;

            // Restore minimum trust floor
            if state.trust.trust_score < 25.0 {
                state.trust.trust_score = 25.0;
            }

            // Adaptive grace window based on recovery severity
            state.recovery.grace_ticks_remaining = if state.trust.trust_score <= 10.0 {
                8
            } else if state.trust.trust_score <= 20.0 {
                6
            } else {
                4
            };
        }
    }
}

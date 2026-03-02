use fluxlock_core::{
    EngineCompositeState,
    InvariantViolation,
};

pub struct FluxlockEngine;

impl FluxlockEngine {
    pub fn execute_tick(
        &mut self,
        previous: &EngineCompositeState,
        next: &EngineCompositeState,
    ) -> Result<(), InvariantViolation> {

        // 🔒 Invariant 1 — Trust only increases during recovery grace
        if next.trust.trust_score > previous.trust.trust_score
            && !previous.recovery.is_recovering
            && previous.recovery.grace_ticks_remaining == 0
        {
            return Err(InvariantViolation::TrustIncreasedOutsideRecovery);
        }

        // 🔒 Invariant 2 — Lock never decreases
        if next.lock.level < previous.lock.level {
            return Err(InvariantViolation::LockDecreased);
        }

        // 🔒 Invariant 3 — Lifecycle never regresses
        if next.lifecycle.stage < previous.lifecycle.stage {
            return Err(InvariantViolation::LifecycleRegression);
        }

        Ok(())
    }
}

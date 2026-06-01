use fluxlock_core::types::{
    Validator,
};

use fluxlock_core::types::ContinuityState;

// =========================
// 🧠 STATE ENGINE
// =========================
pub fn evaluate_continuity_state(
    validator: &mut Validator,
) {

    // =========================
    // ☠ EXILED
    // =========================
    if validator.fracture_severity > 95.0
    && validator.quarantine_level > 90.0 {

        validator.continuity_state =
            ContinuityState::Exiled;

        return;
    }

    // =========================
    // 🔴 FRACTURED
    // =========================
    if !validator.chain_valid
    || validator.drift > 120.0 {

        validator.continuity_state =
            ContinuityState::Fractured;

        return;
    }

    // =========================
    // ⚠ QUARANTINED
    // =========================
    if validator.quarantine_level > 40.0 {

        validator.continuity_state =
            ContinuityState::Quarantined;

        return;
    }

    // =========================
    // 🧬 REHABILITATING
    // =========================
    if validator.recovery_timer > 0
    && validator.rehabilitation_score > 20.0 {

        validator.continuity_state =
            ContinuityState::Rehabilitating;

        return;
    }

    // =========================
    // 🟠 RECOVERING
    // =========================
    if validator.recovery_timer > 0
    || validator.drift > 40.0 {

        validator.continuity_state =
            ContinuityState::Recovering;

        return;
    }

    // =========================
    // 🔄 EVOLVING
    // =========================
    if validator.epoch_age < 120 {

        validator.continuity_state =
            ContinuityState::Evolving;

        return;
    }

    // =========================
    // 🟢 HEALTHY
    // =========================
    validator.continuity_state =
        ContinuityState::Healthy;
}
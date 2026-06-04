use fluxlock_core::types::{
    Validator,
};

use fluxlock_core::types::ContinuityState;

// =========================
// 🧠 AUTHORITY SCORE
// =========================
fn authority_score(
    validator: &Validator,
) -> f64 {

    let score =

        validator.trust * 0.20

        + validator.continuity_reputation * 0.25

        + validator.adaptive_reputation * 0.20

        + validator.continuity_memory_score * 0.15

        + validator.evolutionary_authenticity * 0.10

        + (
            validator.historical_consensus_accuracy
            * 100.0
        ) * 0.10;

    score.clamp(0.0, 100.0)
}

// =========================
// 🧠 STATE ENGINE
// =========================
pub fn evaluate_continuity_state(
    validator: &mut Validator,
) {

    let authority =
        authority_score(
            validator
        );

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
    // 🔴 HARD FRACTURE
    // =========================
    if !validator.chain_valid
    || validator.drift > 120.0 {

        validator.continuity_state =
            ContinuityState::Fractured;

        return;
    }

    // =========================
    // ⚠ HARD QUARANTINE
    // =========================
    if validator.quarantine_level > 75.0 {

        validator.continuity_state =
            ContinuityState::Quarantined;

        return;
    }

    // =========================
    // 🧠 AUTHORITY GOVERNANCE
    // =========================

    // Healthy
    if authority >= 85.0 {

        validator.continuity_state =
            ContinuityState::Healthy;

        return;
    }

    // Evolving
    if authority >= 70.0 {

        validator.continuity_state =
            ContinuityState::Evolving;

        return;
    }

    // Recovering
    if authority >= 50.0 {

        validator.continuity_state =
            ContinuityState::Recovering;

        return;
    }

    // Rehabilitating
    if authority >= 30.0 {

        validator.continuity_state =
            ContinuityState::Rehabilitating;

        return;
    }

    // Quarantined
    if authority >= 10.0 {

        validator.continuity_state =
            ContinuityState::Quarantined;

        return;
    }

    // =========================
    // 🔴 AUTHORITY COLLAPSE
    // =========================
    validator.continuity_state =
        ContinuityState::Fractured;
}
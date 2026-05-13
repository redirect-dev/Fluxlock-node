use fluxlock_core::types::{
    Validator,
};

// =========================
// 🧠 GOVERNANCE RESPONSE
// =========================
pub struct GovernanceResult {

    pub allow_rotation: bool,

    pub stabilization_delta: f64,

    pub quarantine_reduction: f64,

    pub trust_bonus: f64,
}

// =========================
// 🧠 GOVERNANCE ENGINE
// =========================
pub fn evaluate_governance(
    validator: &Validator,
) -> GovernanceResult {

    // =========================
    // 🔒 STABILIZATION WINDOW
    // =========================
    let epochs_since_rotation =
        validator.current_epoch
        .saturating_sub(
            validator.last_epoch_transition
        );

    // =========================
    // 🟡 ROTATION COOL DOWN
    // =========================
    if epochs_since_rotation < 120 {

        return GovernanceResult {

            allow_rotation: false,

            stabilization_delta: 0.15,

            quarantine_reduction: 0.05,

            trust_bonus: 0.02,
        };
    }

    // =========================
    // 🔴 QUARANTINE LOCK
    // =========================
    if validator.quarantine_level > 15.0 {

        return GovernanceResult {

            allow_rotation: false,

            stabilization_delta: 0.08,

            quarantine_reduction: 0.12,

            trust_bonus: 0.01,
        };
    }

    // =========================
    // 🟢 HEALTHY GOVERNANCE
    // =========================
    GovernanceResult {

        allow_rotation: true,

        stabilization_delta: 0.02,

        quarantine_reduction: 0.03,

        trust_bonus: 0.05,
    }
}
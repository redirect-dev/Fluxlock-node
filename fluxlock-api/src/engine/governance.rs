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

    // 🧬 NEW
    pub rehabilitation_boost: f64,

    pub scar_reduction: f64,

    pub immune_response_boost: f64,

    pub network_reacceptance: bool,
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
    // 🔴 FRACTURED STATE
    // =========================
    if !validator.chain_valid {

        return GovernanceResult {

            allow_rotation: false,

            stabilization_delta: 0.50,

            quarantine_reduction: 0.0,

            trust_bonus: -0.10,

            rehabilitation_boost: -1.0,

            scar_reduction: 0.0,

            immune_response_boost: 0.0,

            network_reacceptance: false,
        };
    }

    // =========================
    // 🟠 QUARANTINE RECOVERY
    // =========================
    if validator.quarantine_level > 25.0 {

        return GovernanceResult {

            allow_rotation: false,

            stabilization_delta: 0.35,

            quarantine_reduction: 0.40,

            trust_bonus: 0.03,

            rehabilitation_boost: 1.2,

            scar_reduction: 0.02,

            immune_response_boost: 0.05,

            network_reacceptance: false,
        };
    }

    // =========================
    // 🟡 RECOVERING STATE
    // =========================
    if validator.status == "recovering" {

        return GovernanceResult {

            allow_rotation: false,

            stabilization_delta: 0.18,

            quarantine_reduction: 0.25,

            trust_bonus: 0.05,

            rehabilitation_boost: 1.8,

            scar_reduction: 0.04,

            immune_response_boost: 0.10,

            network_reacceptance: true,
        };
    }

    // =========================
    // 🔄 ROTATION COOL DOWN
    // =========================
    if epochs_since_rotation < 120 {

        return GovernanceResult {

            allow_rotation: false,

            stabilization_delta: 0.10,

            quarantine_reduction: 0.05,

            trust_bonus: 0.02,

            rehabilitation_boost: 0.4,

            scar_reduction: 0.01,

            immune_response_boost: 0.03,

            network_reacceptance: true,
        };
    }

    // =========================
    // 🟢 HEALTHY GOVERNANCE
    // =========================
    GovernanceResult {

        allow_rotation: true,

        stabilization_delta: 0.04,

        quarantine_reduction: 0.10,

        trust_bonus: 0.08,

        rehabilitation_boost: 2.5,

        scar_reduction: 0.08,

        immune_response_boost: 0.15,

        network_reacceptance: true,
    }
}
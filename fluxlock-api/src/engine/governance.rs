use fluxlock_core::types::{
    Validator,
};

use fluxlock_core::types::ContinuityState;

// =========================
// 🧠 GOVERNANCE RESPONSE
// =========================
pub struct GovernanceResult {

    pub allow_rotation: bool,

    pub stabilization_delta: f64,

    pub quarantine_reduction: f64,

    pub trust_bonus: f64,

    // 🧬 REHABILITATION
    pub rehabilitation_boost: f64,

    pub scar_reduction: f64,

    pub immune_response_boost: f64,

    pub network_reacceptance: bool,

    // 🌐 TEMPORAL MEMORY
    pub adaptive_reputation_delta: f64,

    pub continuity_memory_delta: f64,

    pub historical_stability_bonus: f64,

    pub maturity_bonus: f64,

    pub fracture_penalty: f64,
}

// =========================
// 🧠 GOVERNANCE ENGINE
// =========================
pub fn evaluate_governance(
    validator: &Validator,
) -> GovernanceResult {

    // =========================
    // ⏳ ROTATION WINDOW
    // =========================
    let epochs_since_rotation =
        validator.current_epoch
        .saturating_sub(
            validator.last_epoch_transition
        );

    // =========================
    // 🧠 TEMPORAL MEMORY
    // =========================
    let maturity_factor =
        (
            validator.continuity_age
            as f64
            / 1000.0
        )
        .min(5.0);

    let recovery_strength =
        (
            validator.recovery_history
            as f64
            * 0.03
        )
        .min(3.0);

    let fracture_penalty =
        (
            validator.fracture_history
            as f64
            * 0.05
        )
        .min(4.0);

    let governance_memory =
        (
            validator.governance_history
            as f64
            * 0.01
        )
        .min(2.5);

        // =========================
// 🏛 AUTHORITY GOVERNANCE
// =========================

let authority_factor =

    (
        validator.leadership_score
        * 0.35
    )

    +

    (
        validator.network_influence_score
        * 0.35
    )

    +

    (
        validator.governance_weight
        * 10.0
    )

    +

    (
        validator.peer_reputation
        * 0.20
    );

let authority_bonus =
    (
        authority_factor / 100.0
    )
    .min(5.0);

    let stability_memory =
        (
            validator
                .historical_consensus_accuracy
            * 0.05
        )
        .min(2.0);

    // =========================
    // 🔴 FRACTURED STATE
    // =========================
    if !validator.chain_valid {

        return GovernanceResult {

            allow_rotation: false,

            stabilization_delta:
                0.50
                + fracture_penalty,

            quarantine_reduction: 0.0,

            trust_bonus:
                -0.10
                - fracture_penalty,

            rehabilitation_boost:
                -1.0,

            scar_reduction: 0.0,

            immune_response_boost: 0.0,

            network_reacceptance: false,

            adaptive_reputation_delta:
                -0.50,

            continuity_memory_delta:
                -0.25,

            historical_stability_bonus:
                0.0,

            maturity_bonus: 0.0,

            fracture_penalty,
        };
    }

    // =========================
    // 🟠 QUARANTINED
    // =========================
    if validator.quarantine_level > 25.0 {

        return GovernanceResult {

            allow_rotation: false,

            stabilization_delta:
                0.35
                + fracture_penalty,

            quarantine_reduction:
                0.40
                + recovery_strength,

            trust_bonus:
                0.03
                + governance_memory,

            rehabilitation_boost:
                1.2
                + recovery_strength,

            scar_reduction:
                0.02,

            immune_response_boost:
                0.05,

            network_reacceptance: false,

            adaptive_reputation_delta:
                0.05,

            continuity_memory_delta:
                0.04,

            historical_stability_bonus:
                stability_memory,

            maturity_bonus:
                maturity_factor,

            fracture_penalty,
        };
    }

    // =========================
    // 🟡 RECOVERING
    // =========================
    if validator.continuity_state
    == ContinuityState::Recovering {

        return GovernanceResult {

            allow_rotation: false,

            stabilization_delta:
                0.18,

            quarantine_reduction:
                0.25
                + recovery_strength,

            trust_bonus:
                0.05
                + governance_memory,

            rehabilitation_boost:
                1.8
                + recovery_strength,

            scar_reduction:
                0.04,

            immune_response_boost:
                0.10,

            network_reacceptance: true,

            adaptive_reputation_delta:
                0.10,

            continuity_memory_delta:
                0.08,

            historical_stability_bonus:
                stability_memory,

            maturity_bonus:
                maturity_factor,

            fracture_penalty,
        };
    }

    // =========================
    // 🔄 ROTATION COOL DOWN
    // =========================
    if epochs_since_rotation < 120 {

        return GovernanceResult {

            allow_rotation: false,

            stabilization_delta:
                0.10,

            quarantine_reduction:
                0.05,

            trust_bonus:
                0.02
                + governance_memory,

            rehabilitation_boost:
                0.4,

            scar_reduction:
                0.01,

            immune_response_boost:
                0.03,

            network_reacceptance: true,

            adaptive_reputation_delta:
                0.03,

            continuity_memory_delta:
                0.02,

            historical_stability_bonus:
                stability_memory,

            maturity_bonus:
                maturity_factor,

            fracture_penalty,
        };
    }

    // =========================
    // 🟢 HEALTHY GOVERNANCE
    // =========================
    GovernanceResult {

        allow_rotation: true,

        stabilization_delta:
            (
                0.04
                - (
                    maturity_factor
                    * 0.002
                )
            )
            .max(0.01),

        quarantine_reduction:
            0.10
            + recovery_strength,

        trust_bonus:
            0.08
             + governance_memory
             + stability_memory
             + authority_bonus * 0.05,

        rehabilitation_boost:
            2.5
            + recovery_strength
            + authority_bonus * 0.20,

        scar_reduction:
            0.08,

        immune_response_boost:
            0.15
            + maturity_factor * 0.02
            + authority_bonus * 0.03,

        network_reacceptance: true,

        adaptive_reputation_delta:
            0.20
             + maturity_factor * 0.05
             + authority_bonus * 0.05,

        continuity_memory_delta:
            0.15
            + governance_memory,

        historical_stability_bonus:
            stability_memory,

        maturity_bonus:
            maturity_factor,

        fracture_penalty,
    }
}
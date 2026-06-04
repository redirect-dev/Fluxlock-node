use serde::Serialize;

use fluxlock_core::types::{
    Validator,
};

use fluxlock_core::types::ContinuityState;

// =========================
// 🧠 DECISION
// =========================
#[derive(Serialize)]
pub struct Decision {

    pub decision: String,

    pub weight: f64,

    pub state: String,

    pub reason: String,
}

// =========================
// 🧬 AUTHORITY SCORE
// =========================
fn authority_score(
    v: &Validator,
) -> f64 {

    let score =

        // continuity history
        v.continuity_reputation * 0.30

        // adaptive behavior
        + v.adaptive_reputation * 0.25

        // memory persistence
        + v.continuity_memory_score * 0.15

        // authenticity engine
        + v.evolutionary_authenticity * 0.15

        // governance success
        + (
            v.historical_consensus_accuracy
            * 100.0
        ) * 0.10

        // peer agreement
        + (
            v.peer_agreement_ratio
            * 100.0
        ) * 0.05;

    score.clamp(0.0, 100.0)
}

// =========================
// 🧠 EVALUATE
// =========================
pub fn evaluate_validator(
    v: &Validator,
) -> Decision {

    let authority =
        authority_score(v);

    // =========================
    // ☠ EXILED
    // =========================
    if v.continuity_state
        == ContinuityState::Exiled
    {

        return Decision {

            decision:
                "REJECT".into(),

            weight: 0.0,

            state:
                "exiled".into(),

            reason:
                "continuity permanently rejected"
                    .into(),
        };
    }

    // =========================
    // 🔴 FRACTURED
    // =========================
    if v.continuity_state
        == ContinuityState::Fractured
    {

        return Decision {

            decision:
                "REJECT".into(),

            weight: 0.0,

            state:
                "fractured".into(),

            reason:
                "continuity fractured"
                    .into(),
        };
    }

    // =========================
    // ⚠ QUARANTINED
    // =========================
    if v.continuity_state
        == ContinuityState::Quarantined
    {

        return Decision {

            decision:
                "WEIGHTED".into(),

            weight:
                (
                    authority / 100.0
                )
                .clamp(
                    0.05,
                    0.25
                ),

            state:
                "quarantined".into(),

            reason:
                format!(
                    "authority score {:.2}",
                    authority
                ),
        };
    }

    // =========================
    // 🧬 REHABILITATING
    // =========================
    if v.continuity_state
        == ContinuityState::Rehabilitating
    {

        return Decision {

            decision:
                "WEIGHTED".into(),

            weight:
                (
                    authority / 100.0
                )
                .clamp(
                    0.20,
                    0.50
                ),

            state:
                "rehabilitating".into(),

            reason:
                format!(
                    "authority score {:.2}",
                    authority
                ),
        };
    }

    // =========================
    // 🟠 RECOVERING
    // =========================
    if v.continuity_state
        == ContinuityState::Recovering
    {

        return Decision {

            decision:
                "WEIGHTED".into(),

            weight:
                (
                    authority / 100.0
                )
                .clamp(
                    0.20,
                    0.65
                ),

            state:
                "recovering".into(),

            reason:
                format!(
                    "authority score {:.2}",
                    authority
                ),
        };
    }

    // =========================
    // 🔄 EVOLVING
    // =========================
    if v.continuity_state
        == ContinuityState::Evolving
    {

        return Decision {

            decision:
                "WEIGHTED".into(),

            weight:
                (
                    authority / 100.0
                )
                .clamp(
                    0.50,
                    0.85
                ),

            state:
                "evolving".into(),

            reason:
                format!(
                    "authority score {:.2}",
                    authority
                ),
        };
    }

    // =========================
    // 🟢 HEALTHY
    // =========================
    let weight =
        (
            authority / 100.0
        )
        .clamp(
            0.50,
            1.0
        );

    let decision =
        if authority >= 90.0 {

            "ACCEPT"

        } else if authority >= 75.0 {

            "WEIGHTED"

        } else {

            "REJECT"
        };

    Decision {

        decision:
            decision.into(),

        weight,

        state:
            "healthy".into(),

        reason:
            format!(
                "authority score {:.2}",
                authority
            ),
    }
}
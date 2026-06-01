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
// 🧠 EVALUATE
// =========================
pub fn evaluate_validator(
    v: &Validator,
) -> Decision {

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

            weight: 0.10,

            state:
                "quarantined".into(),

            reason:
                "network quarantine active"
                    .into(),
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

            weight: 0.35,

            state:
                "rehabilitating".into(),

            reason:
                "continuity recovery in progress"
                    .into(),
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
                    v.trust / 100.0
                )
                .clamp(0.2, 0.6),

            state:
                "recovering".into(),

            reason:
                "identity instability detected"
                    .into(),
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

            weight: 0.75,

            state:
                "evolving".into(),

            reason:
                "identity mutation stabilizing"
                    .into(),
        };
    }

    // =========================
    // 🟢 HEALTHY
    // =========================
    Decision {

        decision:
            "ACCEPT".into(),

        weight:
            (
                v.trust / 100.0
            )
            .clamp(0.6, 1.0),

        state:
            "healthy".into(),

        reason:
            "continuity verified"
                .into(),
    }
}
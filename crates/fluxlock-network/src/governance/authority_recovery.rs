use fluxlock_core::types::{
    Validator,
    ContinuityState,
};

// =========================
// 🔄 AUTHORITY RECOVERY
// =========================
pub fn recover_authority(
    validator: &mut Validator,
) {

    let mut recovery = 0.0;

    // =========================
    // 🌱 STATE RECOVERY
    // =========================
    match validator.continuity_state {

        ContinuityState::Healthy => {
            recovery += 1.0;
        }

        ContinuityState::Evolving => {
            recovery += 1.5;
        }

        ContinuityState::Recovering => {
            recovery += 2.0;
        }

        ContinuityState::Rehabilitating => {
            recovery += 3.0;
        }

        _ => {}
    }

    // =========================
    // 🧠 MEMORY BONUS
    // =========================
    recovery +=
        validator.continuity_memory_score
        * 0.0025;

    // =========================
    // 🌐 CONSENSUS BONUS
    // =========================
    recovery +=
        validator.peer_agreement_ratio;

    // =========================
    // 🛡 STABILITY BONUS
    // =========================
    recovery +=
        validator.validator_stability_index
        * 0.005;

    // =========================
    // 👑 APPLY RECOVERY
    // =========================
    validator.authority_points +=
        recovery;

    // =========================
    // ⚖ POLITICAL DECAY
    // =========================
    let authority_decay =

        validator.authority_points
        * 0.0005;

    validator.authority_points -=
        authority_decay;

    // =========================
    // 👑 TERM FATIGUE
    // =========================
    if validator.elected_authority {

        validator.authority_points -=
            validator.governance_term
            as f64
            * 0.01;
    }

    // =========================
    // 🔒 LIMITS
    // =========================
    validator.authority_points =
        validator
            .authority_points
            .clamp(
                0.0,
                100000.0
            );
}
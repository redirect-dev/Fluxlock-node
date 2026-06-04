use fluxlock_core::types::{
    Validator,
    ContinuityState,
};

// =========================
// 👑 AUTHORITY LIFECYCLE
// =========================
pub fn promote_authority(
    validator: &mut Validator,
) {

    // =========================
    // 🧠 AUTHORITY SCORE
    // =========================
    let authority =

        validator.trust * 0.20

        + validator.continuity_reputation * 0.25

        + validator.adaptive_reputation * 0.20

        + validator.continuity_memory_score * 0.15

        + validator.evolutionary_authenticity * 0.10

        + (
            validator.historical_consensus_accuracy
            * 100.0
        ) * 0.10;

    // =========================
    // 📈 AUTHORITY GROWTH
    // =========================
    validator.authority_points +=
        authority * 0.01;

    // =========================
    // 📉 AUTHORITY DECAY
    // =========================
    let mut decay = 0.0;

    if validator.trust < 70.0 {

        decay += 2.0;
    }

    if validator.continuity_reputation < 60.0 {

        decay += 2.0;
    }

    if validator.fracture_history > 0 {

        decay +=
            validator.fracture_history
            as f64
            * 0.10;
    }

    match validator.continuity_state {

        ContinuityState::Recovering => {

            decay += 1.0;
        }

        ContinuityState::Rehabilitating => {

            decay += 0.5;
        }

        ContinuityState::Quarantined => {

            decay += 3.0;
        }

        ContinuityState::Fractured => {

            decay += 5.0;
        }

        ContinuityState::Exiled => {

            decay += 10.0;
        }

        _ => {}
    }

    validator.authority_points -= decay;

    if validator.authority_points < 0.0 {

        validator.authority_points = 0.0;
    }

    // =========================
    // 👑 DETERMINE RANK
    // =========================
    let new_rank =

        if validator.authority_points >= 5000.0 {

            "Archon"

        } else if validator.authority_points >= 2500.0 {

            "Governor"

        } else if validator.authority_points >= 1000.0 {

            "Steward"

        } else if validator.authority_points >= 500.0 {

            "Senior Validator"

        } else if validator.authority_points >= 100.0 {

            "Validator"

        } else {

            "Observer"
        };

    // =========================
    // ⬆ PROMOTION
    // =========================
    let old_rank =
        validator.authority_rank.clone();

    if old_rank != new_rank {

        let old_value =
            rank_value(
                &old_rank
            );

        let new_value =
            rank_value(
                new_rank
            );

        if new_value > old_value {

            validator.authority_promotions += 1;

            validator.last_promotion_epoch =
                validator.current_epoch;
        }

        // =========================
        // ⬇ DEMOTION
        // =========================
        if new_value < old_value {

            validator.authority_demotions += 1;
        }

        validator.authority_rank =
            new_rank.to_string();
    }
}

// =========================
// 👑 RANK VALUE
// =========================
fn rank_value(
    rank: &str,
) -> u32 {

    match rank {

        "Archon" => 6,

        "Governor" => 5,

        "Steward" => 4,

        "Senior Validator" => 3,

        "Validator" => 2,

        _ => 1,
    }
}
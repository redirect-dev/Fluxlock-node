use fluxlock_storage::authority_history_store::{
    save_authority_event,
};

use fluxlock_core::types::{
    Validator,
    ContinuityState,
    AuthorityEvent,
    AuthorityEventType,
};

// =========================
// 👑 AUTHORITY LIFECYCLE
// =========================
pub fn promote_authority(
    validator: &mut Validator,
) {

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

    validator.authority_points +=
        authority * 0.01;

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

            let event =
                AuthorityEvent {

                    validator_id:
                        validator.id,

                    epoch:
                        validator.current_epoch,

                    event_type:
                        AuthorityEventType::Promotion,

                    authority_before:
                        old_value as f64,

                    authority_after:
                        new_value as f64,

                    description:
                        format!(
                            "{} promoted to {}",
                            old_rank,
                            new_rank
                        ),
                };

            validator.authority_history.push(
                event.clone()
            );

            save_authority_event(
                &event
            ).ok();

            println!(
                "👑 AUTHORITY EVENT | Validator {} | PROMOTION {} -> {}",
                validator.id,
                old_rank,
                new_rank
            );
        }

        if new_value < old_value {

            validator.authority_demotions += 1;

            let event =
                AuthorityEvent {

                    validator_id:
                        validator.id,

                    epoch:
                        validator.current_epoch,

                    event_type:
                        AuthorityEventType::Demotion,

                    authority_before:
                        old_value as f64,

                    authority_after:
                        new_value as f64,

                    description:
                        format!(
                            "{} demoted to {}",
                            old_rank,
                            new_rank
                        ),
                };

            validator.authority_history.push(
                event.clone()
            );

            save_authority_event(
                &event
            ).ok();

            println!(
                "⚠ AUTHORITY EVENT | Validator {} | DEMOTION {} -> {}",
                validator.id,
                old_rank,
                new_rank
            );
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
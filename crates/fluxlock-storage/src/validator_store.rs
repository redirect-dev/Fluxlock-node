use rusqlite::{
    params,
    Result,
};

use crate::db::DB;

use fluxlock_core::types::{
    Validator,
};

// =========================
// 💾 SAVE VALIDATOR
// =========================
pub fn save_validator(
    validator: &Validator,
) -> Result<()> {

    let conn =
        DB.lock().unwrap();

    conn.execute(
        "
        INSERT OR REPLACE INTO validators (

            id,

            confidence,
            trust,
            drift,

            epoch_age,

            chain_valid,
            network_accepted,

            recovery_timer,

            rehabilitation_score,
            rehabilitation_epochs,

            peer_votes_valid,
            peer_votes_invalid,

            local_valid,
            global_valid,

            attack_history,
            successful_recoveries,

            resilience_score,
            scar_level,
            immune_response,

            consensus_pressure,
            instability_radius,

            stabilization_power,
            rehabilitation_votes,

            fracture_severity,

            continuity_anchor_strength,

            current_epoch,

            inherited_trust,
            lineage_stability,

            epoch_rotations,
            rebirth_count,

            last_epoch_transition,

            status

        )
        VALUES (

            ?1,

            ?2,
            ?3,
            ?4,

            ?5,

            ?6,
            ?7,

            ?8,

            ?9,
            ?10,

            ?11,
            ?12,

            ?13,
            ?14,

            ?15,
            ?16,

            ?17,
            ?18,
            ?19,

            ?20,
            ?21,

            ?22,
            ?23,

            ?24,

            ?25,

            ?26,

            ?27,
            ?28,

            ?29,
            ?30,

            ?31,

            ?32
        )
        ",
        params![

            validator.id,

            validator.confidence,
            validator.trust,
            validator.drift,

            validator.epoch_age,

            validator.chain_valid,
            validator.network_accepted,

            validator.recovery_timer,

            validator.rehabilitation_score,
            validator.rehabilitation_epochs,

            validator.peer_votes_valid,
            validator.peer_votes_invalid,

            validator.local_valid,
            validator.global_valid,

            validator.attack_history,
            validator.successful_recoveries,

            validator.resilience_score,
            validator.scar_level,
            validator.immune_response,

            validator.consensus_pressure,
            validator.instability_radius,

            validator.stabilization_power,
            validator.rehabilitation_votes,

            validator.fracture_severity,

            validator.continuity_anchor_strength,

            validator.current_epoch,

            validator.inherited_trust,
            validator.lineage_stability,

            validator.epoch_rotations,
            validator.rebirth_count,

            validator.last_epoch_transition,

            validator.status
        ]
    )?;

    Ok(())
}

// =========================
// 📦 LOAD VALIDATORS
// =========================
pub fn validator_count() -> usize {

    let conn =
        DB.lock().unwrap();

    let mut stmt =
        conn.prepare(
            "
            SELECT COUNT(*)
            FROM validators
            "
        )
        .unwrap();

    let count: i64 =
        stmt.query_row(
            [],
            |row| row.get(0)
        )
        .unwrap();

    count as usize
}
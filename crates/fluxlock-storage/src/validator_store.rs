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

            continuity_memory_score,
            historical_consensus_accuracy,
            recovery_consistency,
            adaptive_reputation,
            continuity_survival_score,

            fracture_history,
            recovery_history,
            governance_history,
            continuity_age,

            attack_history,
            successful_recoveries,

            resilience_score,
            scar_level,
            scar_severity,
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

            quorum_score,
            peer_agreement_ratio,

            malicious_reports,
            consensus_failures,

            last_quorum_epoch,

            governance_weight,
            governance_participation,
            autonomous_trust_bias,

            quarantine_level,

            peer_reputation,
            leadership_score,

            recovery_votes_received,
            recovery_votes_given,

            network_influence_score,
            isolation_events,

            validator_stability_index,

            influence_radius,
            entropy_output,
            immune_strength,
            healing_wave,

            topology_cluster,

            resonance_score,
            regional_pressure,
            trust_gravity,

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

            ?32,
            ?33,

            ?34,

            ?35,

            ?36,

            ?37,
            ?38,

            ?39,
            ?40,

            ?41,

            ?42,
            ?43,
            ?44,

            ?45,

            ?46,
            ?47,

            ?48,
            ?49,

            ?50,
            ?51,

            ?52,

            ?53,
            ?54,
            ?55,
            ?56,

            ?57,

            ?58,
            ?59,
            ?60,

            ?61
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

            validator.continuity_memory_score,
            validator.historical_consensus_accuracy,
            validator.recovery_consistency,
            validator.adaptive_reputation,
            validator.continuity_survival_score,

            validator.fracture_history,
            validator.recovery_history,
            validator.governance_history,
            validator.continuity_age,

            validator.attack_history,
            validator.successful_recoveries,

            validator.resilience_score,
            validator.scar_level,
            validator.scar_severity,
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

            validator.quorum_score,
            validator.peer_agreement_ratio,

            validator.malicious_reports,
            validator.consensus_failures,

            validator.last_quorum_epoch,

            validator.governance_weight,
            validator.governance_participation,
            validator.autonomous_trust_bias,

            validator.quarantine_level,

            validator.peer_reputation,
            validator.leadership_score,

            validator.recovery_votes_received,
            validator.recovery_votes_given,

            validator.network_influence_score,
            validator.isolation_events,

            validator.validator_stability_index,

            validator.influence_radius,
            validator.entropy_output,
            validator.immune_strength,
            validator.healing_wave,

            validator.topology_cluster,

            validator.resonance_score,
            validator.regional_pressure,
            validator.trust_gravity,

            format!(
                "{:?}",
                validator.continuity_state
)
        ]
    )?;

    Ok(())
}

// =========================
// 📦 VALIDATOR COUNT
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
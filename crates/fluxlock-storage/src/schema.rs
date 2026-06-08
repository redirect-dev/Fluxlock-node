use crate::db::DB;

// =========================
// 🚀 INIT SCHEMA
// =========================
pub fn init_schema() {

    let conn =
        DB.lock().unwrap();

    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS validators (

            id INTEGER PRIMARY KEY,

            confidence REAL NOT NULL,
            trust REAL NOT NULL,
            drift REAL NOT NULL,

            epoch_age INTEGER NOT NULL,

            chain_valid INTEGER NOT NULL,
            network_accepted INTEGER NOT NULL,

            recovery_timer INTEGER NOT NULL,

            rehabilitation_score REAL NOT NULL,
            rehabilitation_epochs INTEGER NOT NULL,

            peer_votes_valid INTEGER NOT NULL,
            peer_votes_invalid INTEGER NOT NULL,

            local_valid INTEGER NOT NULL,
            global_valid INTEGER NOT NULL,

            -- =========================
            -- 🧠 CONTINUITY MEMORY
            -- =========================
            continuity_memory_score REAL NOT NULL,

            historical_consensus_accuracy REAL NOT NULL,

            recovery_consistency REAL NOT NULL,

            adaptive_reputation REAL NOT NULL,

            continuity_survival_score REAL NOT NULL,

            fracture_history INTEGER NOT NULL,

            recovery_history INTEGER NOT NULL,

            governance_history INTEGER NOT NULL,

            continuity_age INTEGER NOT NULL,

            -- =========================
            -- 🌐 RESILIENCE
            -- =========================
            attack_history INTEGER NOT NULL,

            successful_recoveries INTEGER NOT NULL,

            resilience_score REAL NOT NULL,

            scar_level REAL NOT NULL,

            scar_severity REAL NOT NULL,

            immune_response REAL NOT NULL,

            consensus_pressure REAL NOT NULL,

            instability_radius REAL NOT NULL,

            stabilization_power REAL NOT NULL,

            rehabilitation_votes INTEGER NOT NULL,

            fracture_severity REAL NOT NULL,

            continuity_anchor_strength REAL NOT NULL,

            current_epoch INTEGER NOT NULL,

            inherited_trust REAL NOT NULL,

            lineage_stability REAL NOT NULL,

            epoch_rotations INTEGER NOT NULL,

            rebirth_count INTEGER NOT NULL,

            last_epoch_transition INTEGER NOT NULL,

            quorum_score REAL NOT NULL,

            peer_agreement_ratio REAL NOT NULL,

            malicious_reports INTEGER NOT NULL,

            consensus_failures INTEGER NOT NULL,

            last_quorum_epoch INTEGER NOT NULL,

            governance_weight REAL NOT NULL,

            governance_participation REAL NOT NULL,

            autonomous_trust_bias REAL NOT NULL,

            quarantine_level REAL NOT NULL,

            peer_reputation REAL NOT NULL,

            leadership_score REAL NOT NULL,

            recovery_votes_received INTEGER NOT NULL,

            recovery_votes_given INTEGER NOT NULL,

            network_influence_score REAL NOT NULL,

            isolation_events INTEGER NOT NULL,

            validator_stability_index REAL NOT NULL,

            -- =========================
            -- 🌐 ECOLOGY
            -- =========================
            influence_radius REAL NOT NULL,

            entropy_output REAL NOT NULL,

            immune_strength REAL NOT NULL,

            healing_wave REAL NOT NULL,

            topology_cluster INTEGER NOT NULL,

            resonance_score REAL NOT NULL,

            regional_pressure REAL NOT NULL,

            trust_gravity REAL NOT NULL,

            status TEXT NOT NULL
        );

        -- =========================
        -- 🔑 IDENTITY LINKS
        -- =========================
        CREATE TABLE IF NOT EXISTS identity_links (

            id INTEGER PRIMARY KEY AUTOINCREMENT,

            validator_id INTEGER NOT NULL,

            chain_index INTEGER NOT NULL,

            public_key BLOB NOT NULL,

            signature BLOB,

            continuity_hash TEXT NOT NULL,

            parent_hash TEXT NOT NULL,

            state_hash TEXT NOT NULL,

            lineage_signature BLOB,

            transition_signature BLOB,

            epoch INTEGER NOT NULL,

            continuity_epoch INTEGER NOT NULL,

            governance_weight REAL NOT NULL,

            governance_score REAL NOT NULL,

            governance_votes INTEGER NOT NULL,

            network_alignment REAL NOT NULL,

            continuity_confidence REAL NOT NULL,

            peer_agreement_ratio REAL NOT NULL,

            entropy_score REAL NOT NULL,

            lineage_stability REAL NOT NULL,

            fracture_severity REAL NOT NULL,

            rehabilitation_factor REAL NOT NULL,

            quarantine_level REAL NOT NULL,

            malicious_reports INTEGER NOT NULL,

            fork_conflicts INTEGER NOT NULL,

            continuity_verified INTEGER NOT NULL
        );

        -- =========================
        -- 🔐 IDENTITIES
        -- =========================
        CREATE TABLE IF NOT EXISTS identities (

            identity_id TEXT PRIMARY KEY,

            created_epoch INTEGER NOT NULL,

            last_active_epoch INTEGER NOT NULL,

            session_count INTEGER NOT NULL,

            trust_score REAL NOT NULL,

            continuity_score REAL NOT NULL,

            bound_validator INTEGER NOT NULL,

            successful_auths INTEGER NOT NULL,

            failed_auths INTEGER NOT NULL,

            recovery_events INTEGER NOT NULL,

            drift_score REAL NOT NULL,

            status TEXT NOT NULL,

            credential_depth INTEGER NOT NULL
        );

        -- =========================
        -- 📜 IDENTITY PROOFS
        -- =========================
        CREATE TABLE IF NOT EXISTS identity_proofs (

            id INTEGER PRIMARY KEY AUTOINCREMENT,

            identity_id TEXT NOT NULL,

            epoch INTEGER NOT NULL,

            validator_id INTEGER NOT NULL,

            trust REAL NOT NULL,

            continuity REAL NOT NULL,

            previous_hash TEXT NOT NULL,

            proof_hash TEXT NOT NULL
        );

        -- =========================
        -- 🧠 CONTINUITY EVENTS
        -- =========================
        CREATE TABLE IF NOT EXISTS continuity_events (

            id INTEGER PRIMARY KEY AUTOINCREMENT,

            validator_id INTEGER NOT NULL,

            epoch INTEGER NOT NULL,

            event_type TEXT NOT NULL,

            severity REAL NOT NULL,

            trust_delta REAL NOT NULL,

            continuity_delta REAL NOT NULL,

            recovery_delta REAL NOT NULL,

            description TEXT NOT NULL
        );

        -- =========================
        -- 🌐 PROTOCOL EVENTS
        -- =========================
        CREATE TABLE IF NOT EXISTS protocol_events (

            id INTEGER PRIMARY KEY AUTOINCREMENT,

            validator_id INTEGER NOT NULL,

            event_type TEXT NOT NULL,

            severity REAL NOT NULL,

            timestamp INTEGER NOT NULL,

            details TEXT NOT NULL
        );

                -- =========================
        -- AUTHORITY EVENTS
        -- =========================
        CREATE TABLE IF NOT EXISTS authority_events (

            id INTEGER PRIMARY KEY AUTOINCREMENT,

            validator_id INTEGER NOT NULL,

            epoch INTEGER NOT NULL,

            event_type TEXT NOT NULL,

            authority_before REAL NOT NULL,

            authority_after REAL NOT NULL,

            description TEXT NOT NULL
        );
        "
    )
    .expect(
        "failed to initialize schema"
    );
}
 
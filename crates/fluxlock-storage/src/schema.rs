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

            attack_history INTEGER NOT NULL,
            successful_recoveries INTEGER NOT NULL,

            resilience_score REAL NOT NULL,
            scar_level REAL NOT NULL,
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

            status TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS identity_links (

            id INTEGER PRIMARY KEY AUTOINCREMENT,

            validator_id INTEGER NOT NULL,

            chain_index INTEGER NOT NULL,

            public_key BLOB NOT NULL,

            signature BLOB
        );

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

        CREATE TABLE IF NOT EXISTS protocol_events (

            id INTEGER PRIMARY KEY AUTOINCREMENT,

            validator_id INTEGER NOT NULL,

            event_type TEXT NOT NULL,

            severity REAL NOT NULL,

            timestamp INTEGER NOT NULL,

            details TEXT NOT NULL
        );
        "
    )
    .expect(
        "failed to initialize schema"
    );
}
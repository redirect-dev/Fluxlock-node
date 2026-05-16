use rusqlite::{
    params,
    Connection,
};

use fluxlock_core::types::{
    IdentityLink,
};

// =========================
// 💾 SAVE IDENTITY CHAIN
// =========================
pub fn save_identity_chain(

    validator_id: u32,

    chain: &Vec<IdentityLink>,

) -> rusqlite::Result<()> {

    let conn =
        Connection::open(
            "fluxlock.db"
        )?;

    create_lineage_table(
        &conn
    )?;

    conn.execute(

        "
        DELETE FROM lineage
        WHERE validator_id = ?1
        ",

        params![
            validator_id
        ],
    )?;

    for (index, link) in
        chain.iter().enumerate()
    {

        conn.execute(

            "
            INSERT INTO lineage (

                validator_id,
                link_index,

                continuity_hash,
                parent_hash,

                epoch,

                governance_weight,
                entropy_score,

                public_key

            )
            VALUES (

                ?1,
                ?2,
                ?3,
                ?4,
                ?5,
                ?6,
                ?7,
                ?8
            )
            ",

            params![

                validator_id,

                index as u64,

                link.continuity_hash,

                link.parent_hash,

                link.epoch,

                link.governance_weight,

                link.entropy_score,

                link.public_key,
            ],
        )?;
    }

    Ok(())
}

// =========================
// 📥 LOAD IDENTITY CHAIN
// =========================
pub fn load_identity_chain(

    validator_id: u32,

) -> rusqlite::Result<Vec<IdentityLink>> {

    let conn =
        Connection::open(
            "fluxlock.db"
        )?;

    create_lineage_table(
        &conn
    )?;

    let mut stmt =
        conn.prepare(

            "
            SELECT

                continuity_hash,
                parent_hash,
                epoch,
                governance_weight,
                entropy_score,
                public_key

            FROM lineage

            WHERE validator_id = ?1

            ORDER BY link_index ASC
            "
        )?;

    let rows =
        stmt.query_map(

            params![
                validator_id
            ],

            |row| {

                let continuity_hash:
                    String =
                    row.get(0)?;

                let parent_hash:
                    String =
                    row.get(1)?;

                let epoch:
                    u64 =
                    row.get(2)?;

                let governance_weight:
                    f64 =
                    row.get(3)?;

                let entropy_score:
                    f64 =
                    row.get(4)?;

                let public_key:
                    Vec<u8> =
                    row.get(5)?;

                Ok(
                    IdentityLink {

                        // =========================
                        // 🔐 CRYPTO
                        // =========================
                        public_key,

                        signature:
                            None,

                        // =========================
                        // 🔗 CONTINUITY
                        // =========================
                        continuity_hash:
                            continuity_hash
                                .clone(),

                        parent_hash,

                        state_hash:
                            continuity_hash
                                .clone(),

                        lineage_signature:
                            None,

                        transition_signature:
                            None,

                        // =========================
                        // 🌐 EPOCH
                        // =========================
                        epoch,

                        continuity_epoch:
                            epoch,

                        validator_id,

                        // =========================
                        // 🧠 GOVERNANCE
                        // =========================
                        governance_weight,

                        governance_score:
                            governance_weight
                                * 100.0,

                        governance_votes: 0,

                        // =========================
                        // 🌐 CONSENSUS
                        // =========================
                        network_alignment: 1.0,

                        continuity_confidence:
                            100.0,

                        peer_agreement_ratio:
                            1.0,

                        // =========================
                        // 🧬 STABILITY
                        // =========================
                        entropy_score,

                        lineage_stability:
                            100.0,

                        fracture_severity:
                            0.0,

                        rehabilitation_factor:
                            1.0,

                        // =========================
                        // ⚠ SECURITY
                        // =========================
                        quarantine_level:
                            0.0,

                        malicious_reports: 0,

                        fork_conflicts: 0,

                        continuity_verified:
                            true,
                    }
                )
            }
        )?;

    let mut chain =
        Vec::new();

    for row in rows {

        chain.push(
            row?
        );
    }

    Ok(chain)
}

// =========================
// 💾 CREATE TABLE
// =========================
fn create_lineage_table(

    conn: &Connection

) -> rusqlite::Result<()> {

    conn.execute(

        "
        CREATE TABLE IF NOT EXISTS lineage (

            validator_id INTEGER,

            link_index INTEGER,

            continuity_hash TEXT,

            parent_hash TEXT,

            epoch INTEGER,

            governance_weight REAL,

            entropy_score REAL,

            public_key BLOB
        )
        ",

        [],
    )?;

    Ok(())
}
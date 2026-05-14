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

                Ok(
                    IdentityLink {

                        public_key:
                            row.get(5)?,

                        signature:
                            None,

                        continuity_hash:
                            row.get(0)?,

                        parent_hash:
                            row.get(1)?,

                        epoch:
                            row.get(2)?,

                        validator_id,

                        governance_weight:
                            row.get(3)?,

                        entropy_score:
                            row.get(4)?,
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
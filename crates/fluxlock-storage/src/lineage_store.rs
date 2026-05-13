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
                entropy_score

            )
            VALUES (

                ?1,
                ?2,
                ?3,
                ?4,
                ?5,
                ?6,
                ?7
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
            ],
        )?;
    }

    Ok(())
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

            entropy_score REAL
        )
        ",

        [],
    )?;

    Ok(())
}
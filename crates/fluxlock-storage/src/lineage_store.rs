use rusqlite::{
    params,
    Result,
};

use crate::db::DB;

use fluxlock_core::types::{
    IdentityLink,
    IdentityProof,
};

// =========================
// 💾 SAVE VALIDATOR LINEAGE
// =========================
pub fn save_identity_chain(
    validator_id: u32,
    chain: &Vec<IdentityLink>,
) -> Result<()> {

    let conn =
        DB.lock().unwrap();

    // =========================
    // 🧹 CLEAR OLD CHAIN
    // =========================
    conn.execute(
        "
        DELETE FROM identity_links
        WHERE validator_id = ?1
        ",
        params![
            validator_id
        ]
    )?;

    // =========================
    // 🔗 STORE CHAIN
    // =========================
    for (index, link)
        in chain.iter().enumerate()
    {

        conn.execute(
            "
            INSERT INTO identity_links (

                validator_id,

                chain_index,

                public_key,

                signature

            )
            VALUES (

                ?1,

                ?2,

                ?3,

                ?4
            )
            ",
            params![

                validator_id,

                index as u32,

                link.public_key,

                link.signature
            ]
        )?;
    }

    Ok(())
}

// =========================
// 💾 SAVE IDENTITY PROOFS
// =========================
pub fn save_identity_proofs(
    identity_id: &str,
    proofs: &Vec<IdentityProof>,
) -> Result<()> {

    let conn =
        DB.lock().unwrap();

    // =========================
    // 🧹 CLEAR OLD PROOFS
    // =========================
    conn.execute(
        "
        DELETE FROM identity_proofs
        WHERE identity_id = ?1
        ",
        params![
            identity_id
        ]
    )?;

    // =========================
    // 🔗 STORE PROOFS
    // =========================
    for proof in proofs {

        conn.execute(
            "
            INSERT INTO identity_proofs (

                identity_id,

                epoch,

                validator_id,

                trust,

                continuity,

                previous_hash,

                proof_hash

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

                identity_id,

                proof.epoch,

                proof.validator_id,

                proof.trust,

                proof.continuity,

                proof.previous_hash,

                proof.proof_hash
            ]
        )?;
    }

    Ok(())
}
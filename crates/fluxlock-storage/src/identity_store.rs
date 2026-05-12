use rusqlite::{
    params,
    Result,
};

use crate::db::DB;

use fluxlock_core::types::{
    FluxIdentity,
};

// =========================
// 💾 SAVE IDENTITY
// =========================
pub fn save_identity(
    identity: &FluxIdentity,
) -> Result<()> {

    let conn =
        DB.lock().unwrap();

    conn.execute(
        "
        INSERT OR REPLACE INTO identities (

            identity_id,

            created_epoch,

            last_active_epoch,

            session_count,

            trust_score,

            continuity_score,

            bound_validator,

            successful_auths,

            failed_auths,

            recovery_events,

            drift_score,

            status,

            credential_depth

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

            ?13
        )
        ",
        params![

            identity.identity_id,

            identity.created_epoch,

            identity.last_active_epoch,

            identity.session_count,

            identity.trust_score,

            identity.continuity_score,

            identity.bound_validator,

            identity.successful_auths,

            identity.failed_auths,

            identity.recovery_events,

            identity.drift_score,

            identity.status,

            identity.credential_depth
        ]
    )?;

    Ok(())
}

// =========================
// 📦 IDENTITY EXISTS
// =========================
pub fn identity_exists(
    identity_id: &str,
) -> bool {

    let conn =
        DB.lock().unwrap();

    let mut stmt =
        conn.prepare(
            "
            SELECT COUNT(*)
            FROM identities
            WHERE identity_id = ?1
            "
        )
        .unwrap();

    let count: i64 =
        stmt.query_row(
            [identity_id],
            |row| row.get(0)
        )
        .unwrap();

    count > 0
}

// =========================
// 📊 IDENTITY COUNT
// =========================
pub fn identity_count() -> usize {

    let conn =
        DB.lock().unwrap();

    let mut stmt =
        conn.prepare(
            "
            SELECT COUNT(*)
            FROM identities
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
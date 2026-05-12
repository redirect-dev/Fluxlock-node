use rusqlite::{
    params,
    Result,
};

use crate::db::DB;

use std::time::{
    SystemTime,
    UNIX_EPOCH,
};

// =========================
// 💾 STORE EVENT
// =========================
pub fn store_event(
    validator_id: u32,

    event_type: &str,

    severity: f64,

    details: &str,
) -> Result<()> {

    let conn =
        DB.lock().unwrap();

    let timestamp =
        SystemTime::now()
            .duration_since(
                UNIX_EPOCH
            )
            .unwrap()
            .as_secs();

    conn.execute(
        "
        INSERT INTO protocol_events (

            validator_id,

            event_type,

            severity,

            timestamp,

            details

        )
        VALUES (

            ?1,

            ?2,

            ?3,

            ?4,

            ?5
        )
        ",
        params![

            validator_id,

            event_type,

            severity,

            timestamp,

            details
        ]
    )?;

    Ok(())
}
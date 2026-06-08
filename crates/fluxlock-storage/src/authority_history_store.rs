use rusqlite::{
    params,
    Connection,
};

use fluxlock_core::types::{
    AuthorityEvent,
    AuthorityEventType,
};

// =========================
// 💾 SAVE AUTHORITY EVENT
// =========================
pub fn save_authority_event(
    event: &AuthorityEvent,
) -> rusqlite::Result<()> {

    let conn =
        Connection::open(
            "fluxlock.db"
        )?;

    create_authority_table(
        &conn
    )?;

    conn.execute(
        "
        INSERT INTO authority_events (

            validator_id,
            epoch,
            event_type,
            authority_before,
            authority_after,
            description

        )
        VALUES (

            ?1,
            ?2,
            ?3,
            ?4,
            ?5,
            ?6
        )
        ",
        params![

            event.validator_id,

            event.epoch,

            format!(
                "{:?}",
                event.event_type
            ),

            event.authority_before,

            event.authority_after,

            event.description,
        ],
    )?;

    Ok(())
}

// =========================
// 📥 LOAD AUTHORITY HISTORY
// =========================
pub fn load_authority_history(
    validator_id: u32,
) -> rusqlite::Result<Vec<AuthorityEvent>> {

    let conn =
        Connection::open(
            "fluxlock.db"
        )?;

    create_authority_table(
        &conn
    )?;

    let mut stmt =
        conn.prepare(
            "
            SELECT

                epoch,
                event_type,
                authority_before,
                authority_after,
                description

            FROM authority_events

            WHERE validator_id = ?1

            ORDER BY epoch ASC
            "
        )?;

    let rows =
        stmt.query_map(

            params![
                validator_id
            ],

            |row| {

                let event_type:
                    String =
                    row.get(1)?;

                Ok(
                    AuthorityEvent {

                        validator_id,

                        epoch:
                            row.get(0)?,

                        event_type:
                            match event_type.as_str() {

                                "Promotion" =>
                                    AuthorityEventType::Promotion,

                                "Demotion" =>
                                    AuthorityEventType::Demotion,

                                "ElectionWin" =>
                                    AuthorityEventType::ElectionWin,

                                "ElectionLoss" =>
                                    AuthorityEventType::ElectionLoss,

                                "ChallengeWon" =>
                                    AuthorityEventType::ChallengeWon,

                                "ChallengeLost" =>
                                    AuthorityEventType::ChallengeLost,

                                "LegitimacyIncrease" =>
                                    AuthorityEventType::LegitimacyIncrease,

                                "LegitimacyCollapse" =>
                                    AuthorityEventType::LegitimacyCollapse,

                                "SuccessionGranted" =>
                                    AuthorityEventType::SuccessionGranted,

                                _ =>
                                    AuthorityEventType::SuccessionLost,
                            },

                        authority_before:
                            row.get(2)?,

                        authority_after:
                            row.get(3)?,

                        description:
                            row.get(4)?,
                    }
                )
            }
        )?;

    let mut history =
        Vec::new();

    for row in rows {

        history.push(
            row?
        );
    }

    Ok(history)
}

// =========================
// 💾 CREATE TABLE
// =========================
fn create_authority_table(
    conn: &Connection,
) -> rusqlite::Result<()> {

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS authority_events (

            validator_id INTEGER,

            epoch INTEGER,

            event_type TEXT,

            authority_before REAL,

            authority_after REAL,

            description TEXT
        )
        ",
        [],
    )?;

    Ok(())
}
use once_cell::sync::Lazy;

use rusqlite::Connection;

use std::sync::Mutex;

// =========================
// 🌐 GLOBAL DB
// =========================
pub static DB: Lazy<Mutex<Connection>> =
    Lazy::new(|| {

        let conn =
            Connection::open(
                "fluxlock.db"
            )
            .expect(
                "failed to open fluxlock db"
            );

        Mutex::new(conn)
    });

// =========================
// 🚀 INIT DB
// =========================
pub fn init_db() {

    let conn =
        DB.lock().unwrap();

    conn.execute_batch(
        "
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        "
    )
    .expect(
        "failed to initialize database"
    );
}
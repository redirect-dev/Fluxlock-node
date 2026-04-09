use fluxlock_protocol::state::account::Account;
use fluxlock_protocol::state::reveal::apply_rotation_reveal;
use fluxlock_protocol::state::validator::Validator;
use fluxlock_protocol::state::event::Event;
use fluxlock_protocol::tx::transaction::RotationRevealTx;
use fluxlock_protocol::pq;

use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::File;
use std::io::Write;

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

// 🔥 NEW: Write events to JSON file
fn write_events_to_file(events: &Vec<Event>) {
    use std::fs;
    use std::path::PathBuf;

    // ✅ Get path to this crate (fluxlock-protocol)
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // ✅ Navigate to UI public folder
    let path = base
        .join("../../fluxlock-ui/public/events.json");

    let dir = path.parent().unwrap();

    // ✅ Ensure directory exists
    fs::create_dir_all(dir).expect("Failed to create UI public directory");

    let json = serde_json::to_string_pretty(events).unwrap();

    fs::write(&path, json).expect("Unable to write events.json");

    println!("📁 Writing events to: {:?}", path);
}

fn print_events(events: &Vec<Event>) {
    for event in events {
        println!("📡 EVENT: {:?}", event);
    }
}

pub fn run_simulation() {
    println!("🧪 Fluxlock Phase 4 Simulation (RUST → UI BRIDGE)\n");

    let mut all_events: Vec<Event> = vec![];

    let mut accounts: Vec<Account> = vec![];

    // -----------------------------
    // CREATE VALIDATOR
    // -----------------------------
    let mut validator = Validator::new(
        1000,
        b"validator_classical".to_vec(),
        b"validator_pq".to_vec(),
        0,
        100,
    );

    // -----------------------------
    // CREATE ACCOUNT
    // -----------------------------
    let (pq_pk, pq_sk) = pq::generate_keypair();

    let mut acc = Account::new(
        1000,
        b"initial_classical".to_vec(),
        pq_pk.clone(),
    );

    // -----------------------------
    // PREPARE ROTATION
    // -----------------------------
    let new_classical = b"new_classical".to_vec();
    let (new_pq, _) = pq::generate_keypair();

    acc.rotation_commitment = Some({
        let mut hasher = blake3::Hasher::new();
        hasher.update(&new_classical);
        hasher.update(&new_pq);
        hasher.finalize().as_bytes().to_vec()
    });

    accounts.push(acc);

    let timestamp = now();

    // -----------------------------
    // ❌ INVALID ROTATION
    // -----------------------------
    let bad_tx = RotationRevealTx {
        from: b"initial_classical".to_vec(),
        new_classical_key: new_classical.clone(),
        new_pq_key: new_pq.clone(),
        nonce: 0,
        epoch: 1,
        timestamp,
        link_signature: vec![0; 64],
        classical_signature: vec![],
        pq_signature: vec![],
    };

    println!("➡️ Invalid rotation\n");

    let (events, result) = apply_rotation_reveal(&mut accounts, &mut validator, &bad_tx);

    print_events(&events);
    all_events.extend(events);

    if let Err(e) = result {
        println!("❌ Error: {}\n", e);
    }

    // -----------------------------
    // ✅ VALID ROTATION
    // -----------------------------
    let link_sig = pq::sign(&new_pq, &pq_sk);

    let good_tx = RotationRevealTx {
        from: b"initial_classical".to_vec(),
        new_classical_key: new_classical.clone(),
        new_pq_key: new_pq.clone(),
        nonce: 1,
        epoch: 2,
        timestamp: timestamp + 1,
        link_signature: link_sig,
        classical_signature: vec![],
        pq_signature: vec![],
    };

    println!("➡️ Valid rotation\n");

    let (events, result) = apply_rotation_reveal(&mut accounts, &mut validator, &good_tx);

    print_events(&events);
    all_events.extend(events);

    if let Err(e) = result {
        println!("❌ Unexpected Error: {}\n", e);
    }

    // -----------------------------
    // 🔥 WRITE TO UI
    // -----------------------------
    write_events_to_file(&all_events);

    println!("📁 events.json written for UI\n");
    println!("🏁 Simulation complete\n");
}
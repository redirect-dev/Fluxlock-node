use fluxlock_protocol::state::account::Account;
use fluxlock_protocol::state::reveal::apply_rotation_reveal;
use fluxlock_protocol::state::validator::Validator;
use fluxlock_protocol::state::event::Event;
use fluxlock_protocol::tx::transaction::RotationRevealTx;
use fluxlock_protocol::pq;

use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
use std::path::PathBuf;

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn run_simulation() {
    println!("🧪 Fluxlock Phase 5 — STATEFUL MULTI VALIDATOR\n");

    let mut all_events: Vec<Event> = vec![];

    // -----------------------------
    // BASE ACCOUNT
    // -----------------------------
    let (pq_pk, pq_sk) = pq::generate_keypair();

    let base_account = Account::new(
        1000,
        b"initial_classical".to_vec(),
        pq_pk.clone(),
    );

    // -----------------------------
    // ROTATION SETUP
    // -----------------------------
    let new_classical = b"new_classical_v4".to_vec();
    let (new_pq, _) = pq::generate_keypair();

    let mut acc_template = base_account.clone();

    acc_template.rotation_commitment = Some({
        let mut hasher = blake3::Hasher::new();
        hasher.update(&new_classical);
        hasher.update(&new_pq);
        hasher.finalize().as_bytes().to_vec()
    });

    let timestamp = now();

    // -----------------------------
    // TRANSACTIONS
    // -----------------------------
    let bad_tx = RotationRevealTx {
        from: b"initial_classical".to_vec(),
        new_classical_key: new_classical.clone(),
        new_pq_key: new_pq.clone(),
        nonce: 0,
        epoch: 1,
        timestamp,
        link_signature: vec![0; 64], // invalid
        classical_signature: vec![],
        pq_signature: vec![],
    };

    let good_tx = RotationRevealTx {
        from: b"initial_classical".to_vec(),
        new_classical_key: new_classical.clone(),
        new_pq_key: new_pq.clone(),
        nonce: 1,
        epoch: 2,
        timestamp: timestamp + 1,
        link_signature: pq::sign(&new_pq, &pq_sk),
        classical_signature: vec![],
        pq_signature: vec![],
    };

    // -----------------------------
    // VALIDATORS WITH STATE
    // -----------------------------
    let mut validators = vec![
        (Validator::new(1000, b"A".to_vec(), b"A_pq".to_vec(), 0, 100), vec![acc_template.clone()]),
        (Validator::new(1000, b"B".to_vec(), b"B_pq".to_vec(), 0, 100), vec![acc_template.clone()]),
        (Validator::new(1000, b"C".to_vec(), b"C_pq".to_vec(), 0, 100), vec![acc_template.clone()]),
    ];

    println!("➡️ INVALID rotation phase\n");

    for (i, (validator, accounts)) in validators.iter_mut().enumerate() {
        let (events, result) = apply_rotation_reveal(accounts, validator, &bad_tx);

        println!("Validator {}: {:?}", i, result);

        for e in events {
            println!("📡 {:?}", e);
            all_events.push(e);
        }
    }

    println!("\n➡️ VALID rotation phase\n");

    for (i, (validator, accounts)) in validators.iter_mut().enumerate() {
        let (events, result) = apply_rotation_reveal(accounts, validator, &good_tx);

        println!("Validator {}: {:?}", i, result);

        for e in events {
            println!("📡 {:?}", e);
            all_events.push(e);
        }
    }

    // -----------------------------
    // WRITE EVENTS
    // -----------------------------
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../../fluxlock-ui/public/events.json");

    let json = serde_json::to_string_pretty(&all_events).unwrap();

    fs::write(&path, json).expect("Unable to write events.json");

    println!("\n📁 events.json updated");
}
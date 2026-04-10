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
    println!("🧪 Fluxlock Phase 8 — REPUTATION SYSTEM\n");

    let mut all_events: Vec<Event> = vec![];

    let (pq_pk, pq_sk) = pq::generate_keypair();

    let base_account = Account::new(
        1000,
        b"initial_classical".to_vec(),
        pq_pk.clone(),
    );

    let new_classical = b"new_classical_v7".to_vec();
    let (new_pq, _) = pq::generate_keypair();

    let mut acc_template = base_account.clone();

    acc_template.rotation_commitment = Some({
        let mut hasher = blake3::Hasher::new();
        hasher.update(&new_classical);
        hasher.update(&new_pq);
        hasher.finalize().as_bytes().to_vec()
    });

    let timestamp = now();

    let good_tx = RotationRevealTx {
        from: b"initial_classical".to_vec(),
        new_classical_key: new_classical.clone(),
        new_pq_key: new_pq.clone(),
        nonce: 0,
        epoch: 1,
        timestamp,
        link_signature: pq::sign(&new_pq, &pq_sk),
        classical_signature: vec![],
        pq_signature: vec![],
    };

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

    let mut validators = vec![
        ("Validator A", Validator::new(1000, b"A".to_vec(), b"A_pq".to_vec(), 0, 100), vec![acc_template.clone()]),
        ("Validator B", Validator::new(1000, b"B".to_vec(), b"B_pq".to_vec(), 0, 100), vec![acc_template.clone()]),
        ("Validator C", Validator::new(1000, b"C".to_vec(), b"C_pq".to_vec(), 0, 100), vec![acc_template.clone()]),
    ];

    let mut validator_states = Vec::new();

    for (name, validator, accounts) in validators.iter_mut() {
        let (events, _) = match *name {
            "Validator A" => apply_rotation_reveal(accounts, validator, &good_tx, name),
            "Validator B" => apply_rotation_reveal(accounts, validator, &bad_tx, name),
            "Validator C" => {
                let mut tampered = good_tx.clone();
                tampered.nonce = 999;
                apply_rotation_reveal(accounts, validator, &tampered, name)
            }
            _ => unreachable!(),
        };

        for e in events {
            all_events.push(e);
        }

        validator_states.push(serde_json::json!({
            "name": name,
            "stake": validator.stake,
            "reputation": validator.reputation,
        }));
    }

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../../fluxlock-ui/public/events.json");

    let json = serde_json::json!({
        "events": all_events,
        "validators": validator_states
    });

    fs::write(&path, serde_json::to_string_pretty(&json).unwrap())
        .expect("Unable to write events.json");

    println!("📁 events + validator state written");
}
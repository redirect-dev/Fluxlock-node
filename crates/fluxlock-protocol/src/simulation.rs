use fluxlock_protocol::state::account::Account;
use fluxlock_protocol::state::reveal::apply_rotation_reveal;
use fluxlock_protocol::state::validator::Validator;
use fluxlock_protocol::tx::transaction::RotationRevealTx;
use fluxlock_protocol::pq;

use std::time::{SystemTime, UNIX_EPOCH};

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn run_simulation() {
    println!("🧪 Fluxlock Phase 3B Simulation (WITH SLASHING)\n");

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
    // ❌ INVALID (continuity)
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

    println!("➡️ Invalid rotation (should SLASH)");

    let _ = apply_rotation_reveal(&mut accounts, &mut validator, &bad_tx);

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

    println!("➡️ Valid rotation");

    let _ = apply_rotation_reveal(&mut accounts, &mut validator, &good_tx);

    println!("\n🏁 Simulation complete\n");
}
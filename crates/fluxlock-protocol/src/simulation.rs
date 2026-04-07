use fluxlock_protocol::state::account::Account;
use fluxlock_protocol::state::reveal::apply_rotation_reveal;
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
    println!("🧪 Fluxlock Phase 3 Simulation\n");

    let mut accounts: Vec<Account> = vec![];

    // -----------------------------
    // CREATE INITIAL ACCOUNT
    // -----------------------------
    let (pq_pk, pq_sk) = pq::generate_keypair();

    let mut acc = Account::new(
        1000,
        b"initial_classical".to_vec(),
        pq_pk.clone(),
    );

    // commitment for new keys
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
    // ✅ VALID ROTATION
    // -----------------------------
    let link_sig = pq::sign(&new_pq, &pq_sk);

    let tx1 = RotationRevealTx {
        from: b"initial_classical".to_vec(),
        new_classical_key: new_classical.clone(),
        new_pq_key: new_pq.clone(),
        nonce: 0,
        epoch: 1,
        timestamp,

        link_signature: link_sig,
        classical_signature: vec![],
        pq_signature: vec![],
    };

    println!("➡️ Attempting VALID rotation...");

    let result = apply_rotation_reveal(&mut accounts, &tx1);

    match result {
        Ok(_) => println!("✅ Rotation succeeded\n"),
        Err(e) => println!("❌ Unexpected failure: {}\n", e),
    }

    // -----------------------------
    // ❌ FORK ATTEMPT
    // -----------------------------
    let tx2 = RotationRevealTx {
        from: b"new_classical".to_vec(), // identity updated
        new_classical_key: b"fork_classical".to_vec(),
        new_pq_key: b"fork_pq".to_vec(),
        nonce: 1,
        epoch: 1, // SAME epoch → fork
        timestamp: timestamp + 1,

        link_signature: vec![0; 64],
        classical_signature: vec![],
        pq_signature: vec![],
    };

    println!("➡️ Attempting fork (expected: FORK_DETECTED)");

    let result = apply_rotation_reveal(&mut accounts, &tx2);

    match result {
        Ok(_) => println!("⚠️ Fork succeeded (unexpected)\n"),
        Err(e) => println!("✅ Fork blocked: {}\n", e),
    }

    println!("🏁 Simulation complete\n");
}
use std::time::{SystemTime, UNIX_EPOCH};

use crate::tx::transaction::*;
use crate::tx::transaction::Tx;

/// helper
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn run_genesis_test() {
    println!("🧪 Running Genesis Test...\n");

    let timestamp = now();

    // -----------------------------
    // ROTATION COMMIT
    // -----------------------------
    let commit_tx = Tx::RotationCommit(RotationCommitTx {
        from: b"alice".to_vec(),
        new_key_commitment: vec![1, 2, 3],
        nonce: 0,
        epoch: 0,
        timestamp,

        classical_signature: vec![],
        pq_signature: vec![],
    });

    println!("✅ RotationCommitTx created");

    // -----------------------------
    // ROTATION REVEAL
    // -----------------------------
    let reveal_tx = Tx::RotationReveal(RotationRevealTx {
        from: b"alice".to_vec(),
        new_classical_key: vec![4, 5, 6],
        new_pq_key: vec![7, 8, 9],
        nonce: 1,
        epoch: 0,
        timestamp,

        link_signature: vec![],
        classical_signature: vec![],
        pq_signature: vec![7, 8, 9],
    });

    println!("✅ RotationRevealTx created");

    // -----------------------------
    // TRANSFER
    // -----------------------------
    let transfer_tx = Tx::Transfer(TransferTx {
        from: b"alice".to_vec(),
        to: b"bob".to_vec(),
        amount: 100,
        nonce: 2,
        epoch: 0,
        timestamp,

        classical_signature: vec![],
        pq_signature: vec![],
    });

    println!("✅ TransferTx created");

    println!("\n🎉 Genesis Test Complete\n");

    let _ = vec![commit_tx, reveal_tx, transfer_tx];
}
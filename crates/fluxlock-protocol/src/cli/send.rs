use std::time::{SystemTime, UNIX_EPOCH};

use crate::tx::transaction::{Tx, TransferTx};

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn send_tx() {
    println!("📤 Sending transaction...");

    let timestamp = now();

    let tx = Tx::Transfer(TransferTx {
        from: b"alice".to_vec(),
        to: b"bob".to_vec(),
        amount: 50,
        nonce: 0,
        epoch: 0,
        timestamp,

        classical_signature: vec![],
        pq_signature: vec![],
    });

    println!("✅ Transaction created: {:?}", tx);
}
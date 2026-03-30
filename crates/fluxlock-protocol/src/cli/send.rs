use ed25519_dalek::{SigningKey, Signer};

use crate::cli::wallet::load_wallet;
use crate::cli::mempool::add_tx;

use crate::tx::transaction::{Tx, TransferTx};

pub fn send_tx() {
    println!("💸 Preparing transaction...\n");

    let wallet = match load_wallet() {
        Some(w) => w,
        None => return,
    };

    let signing_key = SigningKey::from_bytes(
        &wallet.classical_secret.clone().try_into().expect("Invalid key"),
    );

    let from = wallet.classical_public.clone();
    let to = vec![9; 32];
    let amount = 100u128;

    // 🔥 TEMP FIX: use nonce 1 instead of 0
    let nonce = 1u64;

    let mut msg = vec![];
    msg.extend(&from);
    msg.extend(&to);
    msg.extend(&amount.to_le_bytes());
    msg.extend(&nonce.to_le_bytes());

    let classical_sig = signing_key.sign(&msg).to_bytes().to_vec();
    let pq_sig = crate::pq::sign(&msg, &wallet.pq_secret);

    let tx = Tx::Transfer(TransferTx {
        from,
        to,
        amount,
        nonce,
        classical_signature: classical_sig,
        pq_signature: pq_sig,
    });

    add_tx(tx);

    println!("✅ Transaction submitted\n");
}
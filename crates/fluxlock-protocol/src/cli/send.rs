use ed25519_dalek::{SigningKey, Signer};

use crate::cli::wallet::{load_wallet, save_wallet};
use crate::cli::mempool::add_tx;

use crate::tx::transaction::{Tx, TransferTx};
use crate::tx::message::build_transfer_message;

pub fn send_tx() {
    println!("💸 Preparing transaction...\n");

    let mut wallet = match load_wallet() {
        Some(w) => w,
        None => return,
    };

    let signing_key = SigningKey::from_bytes(
        &wallet.classical_secret.clone().try_into().expect("Invalid key"),
    );

    let from = wallet.classical_public.clone();
    let to = vec![9; 32];
    let amount = 100u128;

    // ✅ REAL NONCE
    let nonce = wallet.nonce;

    // 🔥 NEW: EPOCH (for now derived from nonce or simple counter)
    // later this will be network-driven
    let epoch = wallet.nonce as u64;

    // Build base message
    let msg = build_transfer_message(
        &from,
        &to,
        amount,
        nonce,
    );

    // 🔐 Bind epoch into signature payload
    let mut data = Vec::new();
    data.extend_from_slice(&epoch.to_be_bytes());
    data.extend_from_slice(&msg);

    // Classical signature (epoch-bound)
    let classical_sig = signing_key.sign(&data).to_bytes().to_vec();

    // PQ signature (also bound to epoch for consistency)
    let pq_sig = crate::pq::sign(&data, &wallet.pq_secret);

    let tx = Tx::Transfer(TransferTx {
        from,
        to,
        amount,
        nonce,
        epoch, // 🔥 NEW FIELD (you will need to add this to struct)
        classical_signature: classical_sig,
        pq_signature: pq_sig,
    });

    add_tx(tx);

    // ✅ increment nonce locally
    wallet.nonce += 1;
    save_wallet(&wallet);

    println!("✅ Transaction submitted (epoch-bound)\n");
}
use ed25519_dalek::{SigningKey, Signer};

use crate::cli::wallet::load_wallet;
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
    let to = vec![9; 32]; // demo recipient
    let amount = 100u128;
    let nonce = 0u64;

    // Build message
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

    println!("✅ Transaction created successfully\n");

    println!("--- TX DETAILS ---");
    println!("Amount: {}", amount);
    println!("Nonce: {}", nonce);
    println!("------------------\n");

    // For now, just show success (next step will inject into chain)
    println!("🚀 Ready to submit (integration coming next step)\n");
}
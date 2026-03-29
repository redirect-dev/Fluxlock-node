use ed25519_dalek::{Verifier, Signature, VerifyingKey};

use crate::tx::transaction::TransferTx;

/// Verify Ed25519 signature for a transfer
pub fn verify_transfer(tx: &TransferTx) -> bool {
    // Build deterministic message
    let mut message = vec![];

    message.extend(&tx.from);
    message.extend(&tx.to);
    message.extend(&tx.amount.to_le_bytes());
    message.extend(&tx.nonce.to_le_bytes());

    // Ensure correct key length
    if tx.from.len() != 32 {
        return false;
    }

    let pubkey_bytes: [u8; 32] = match tx.from.as_slice().try_into() {
        Ok(b) => b,
        Err(_) => return false,
    };

    let pubkey = match VerifyingKey::from_bytes(&pubkey_bytes) {
        Ok(pk) => pk,
        Err(_) => return false,
    };

    let sig = match Signature::from_slice(&tx.signature) {
        Ok(s) => s,
        Err(_) => return false,
    };

    pubkey.verify(&message, &sig).is_ok()
}
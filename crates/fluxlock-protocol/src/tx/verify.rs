use ed25519_dalek::{Signature, VerifyingKey, Verifier};

use crate::pq;

use crate::tx::transaction::{
    TransferTx,
    RotationCommitTx,
    RotationRevealTx,
};
use crate::tx::message::build_transfer_message;

// -----------------------------
// TRANSFER
// -----------------------------
pub fn verify_transfer(tx: &TransferTx, pq_pubkey: &Vec<u8>) -> bool {
    let msg = build_transfer_message(
        &tx.from,
        &tx.to,
        tx.amount,
        tx.nonce,
    );

    // --- Public key ---
    let pubkey_bytes: [u8; 32] = match tx.from.clone().try_into() {
        Ok(b) => b,
        Err(_) => return false,
    };

    let pubkey = match VerifyingKey::from_bytes(&pubkey_bytes) {
        Ok(k) => k,
        Err(_) => return false,
    };

    // --- Signature ---
    let sig_bytes: [u8; 64] = match tx.classical_signature.clone().try_into() {
        Ok(b) => b,
        Err(_) => return false,
    };

    let sig = Signature::from_bytes(&sig_bytes);

    if pubkey.verify(&msg, &sig).is_err() {
        return false;
    }

    // --- PQ verify ---
    if !pq::verify(&msg, &tx.pq_signature, pq_pubkey) {
        return false;
    }

    true
}

// -----------------------------
// ROTATION COMMIT
// -----------------------------
pub fn verify_rotation_commit(tx: &RotationCommitTx) -> bool {
    let mut msg = vec![];
    msg.extend(&tx.from);
    msg.extend(&tx.new_key_commitment);
    msg.extend(&tx.nonce.to_le_bytes());

    let pubkey_bytes: [u8; 32] = match tx.from.clone().try_into() {
        Ok(b) => b,
        Err(_) => return false,
    };

    let pubkey = match VerifyingKey::from_bytes(&pubkey_bytes) {
        Ok(k) => k,
        Err(_) => return false,
    };

    let sig_bytes: [u8; 64] = match tx.classical_signature.clone().try_into() {
        Ok(b) => b,
        Err(_) => return false,
    };

    let sig = Signature::from_bytes(&sig_bytes);

    pubkey.verify(&msg, &sig).is_ok()
}

// -----------------------------
// ROTATION REVEAL
// -----------------------------
pub fn verify_rotation_reveal(tx: &RotationRevealTx) -> bool {
    let mut msg = vec![];
    msg.extend(&tx.from);
    msg.extend(&tx.new_classical_key);
    msg.extend(&tx.new_pq_key);
    msg.extend(&tx.nonce.to_le_bytes());

    let pubkey_bytes: [u8; 32] = match tx.from.clone().try_into() {
        Ok(b) => b,
        Err(_) => return false,
    };

    let pubkey = match VerifyingKey::from_bytes(&pubkey_bytes) {
        Ok(k) => k,
        Err(_) => return false,
    };

    let sig_bytes: [u8; 64] = match tx.classical_signature.clone().try_into() {
        Ok(b) => b,
        Err(_) => return false,
    };

    let sig = Signature::from_bytes(&sig_bytes);

    pubkey.verify(&msg, &sig).is_ok()
}
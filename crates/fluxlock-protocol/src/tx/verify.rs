use ed25519_dalek::{Verifier, Signature, VerifyingKey};

use crate::pq;
use crate::tx::transaction::{
    TransferTx,
    RotationCommitTx,
    RotationRevealTx,
};

/// 🔐 Core hybrid verification
fn verify_hybrid(
    pubkey_bytes: &Vec<u8>,
    pq_pubkey: &Vec<u8>,
    message: &Vec<u8>,
    classical_sig: &Vec<u8>,
    pq_sig: &Vec<u8>,
) -> bool {
    // --- Classical ---
    if pubkey_bytes.len() != 32 {
        return false;
    }

    let pubkey_arr: [u8; 32] = match pubkey_bytes.as_slice().try_into() {
        Ok(b) => b,
        Err(_) => return false,
    };

    let pubkey = match VerifyingKey::from_bytes(&pubkey_arr) {
        Ok(pk) => pk,
        Err(_) => return false,
    };

    let sig = match Signature::from_slice(classical_sig) {
        Ok(s) => s,
        Err(_) => return false,
    };

    if pubkey.verify(message, &sig).is_err() {
        return false;
    }

    // --- PQ ---
    if !pq::verify(message, pq_sig, pq_pubkey) {
        return false;
    }

    true
}

/// 🔐 Transfer verification
pub fn verify_transfer(tx: &TransferTx, pq_pubkey: &Vec<u8>) -> bool {
    let mut message = vec![];

    message.extend(&tx.from);
    message.extend(&tx.to);
    message.extend(&tx.amount.to_le_bytes());
    message.extend(&tx.nonce.to_le_bytes());

    verify_hybrid(
        &tx.from,
        pq_pubkey,
        &message,
        &tx.classical_signature,
        &tx.pq_signature,
    )
}

/// 🔐 Rotation Commit verification
pub fn verify_rotation_commit(tx: &RotationCommitTx, pq_pubkey: &Vec<u8>) -> bool {
    let mut message = vec![];

    message.extend(&tx.from);
    message.extend(&tx.new_key_commitment);
    message.extend(&tx.nonce.to_le_bytes());

    verify_hybrid(
        &tx.from,
        pq_pubkey,
        &message,
        &tx.classical_signature,
        &tx.pq_signature,
    )
}

/// 🔐 Rotation Reveal verification
pub fn verify_rotation_reveal(tx: &RotationRevealTx, pq_pubkey: &Vec<u8>) -> bool {
    let mut message = vec![];

    message.extend(&tx.from);
    message.extend(&tx.new_classical_key);
    message.extend(&tx.new_pq_key);
    message.extend(&tx.nonce.to_le_bytes());

    verify_hybrid(
        &tx.from,
        pq_pubkey,
        &message,
        &tx.classical_signature,
        &tx.pq_signature,
    )
}
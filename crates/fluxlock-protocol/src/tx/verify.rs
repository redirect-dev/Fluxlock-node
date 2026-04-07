use ed25519_dalek::{Signature, VerifyingKey, Verifier};

fn to_pubkey(bytes: &[u8]) -> Option<VerifyingKey> {
    let arr: [u8; 32] = bytes.try_into().ok()?;
    VerifyingKey::from_bytes(&arr).ok()
}

fn to_signature(bytes: &[u8]) -> Option<Signature> {
    let arr: [u8; 64] = bytes.try_into().ok()?;
    Some(Signature::from_bytes(&arr))
}

/// ✅ KEEP ORIGINAL NAMES (fixes producer.rs automatically)

pub fn verify_transfer(
    pubkey_bytes: &[u8],
    tx: &crate::tx::transaction::TransferTx,
) -> bool {
    let pubkey = match to_pubkey(pubkey_bytes) {
        Some(pk) => pk,
        None => return false,
    };

    let signature = match to_signature(&tx.classical_signature) {
        Some(sig) => sig,
        None => return false,
    };

    let msg = crate::tx::message::build_transfer_message(
        &tx.from,
        &tx.to,
        tx.amount,
        tx.nonce,
    );

    let mut data = Vec::new();
    data.extend_from_slice(&tx.epoch.to_be_bytes());
    data.extend_from_slice(&msg);

    pubkey.verify(&data, &signature).is_ok()
}

pub fn verify_rotation_commit(
    pubkey_bytes: &[u8],
    tx: &crate::tx::transaction::RotationCommitTx,
) -> bool {
    let pubkey = match to_pubkey(pubkey_bytes) {
        Some(pk) => pk,
        None => return false,
    };

    let signature = match to_signature(&tx.classical_signature) {
        Some(sig) => sig,
        None => return false,
    };

    let mut data = Vec::new();
    data.extend_from_slice(&tx.epoch.to_be_bytes());
    data.extend_from_slice(&tx.new_key_commitment);
    data.extend_from_slice(&tx.nonce.to_be_bytes());

    pubkey.verify(&data, &signature).is_ok()
}

pub fn verify_rotation_reveal(
    pubkey_bytes: &[u8],
    tx: &crate::tx::transaction::RotationRevealTx,
) -> bool {
    let pubkey = match to_pubkey(pubkey_bytes) {
        Some(pk) => pk,
        None => return false,
    };

    let signature = match to_signature(&tx.classical_signature) {
        Some(sig) => sig,
        None => return false,
    };

    let mut data = Vec::new();
    data.extend_from_slice(&tx.epoch.to_be_bytes());
    data.extend_from_slice(&tx.new_classical_key);
    data.extend_from_slice(&tx.new_pq_key);
    data.extend_from_slice(&tx.nonce.to_be_bytes());

    pubkey.verify(&data, &signature).is_ok()
}
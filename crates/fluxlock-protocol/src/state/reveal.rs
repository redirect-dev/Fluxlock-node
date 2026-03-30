use blake3;

use crate::state::account::{Account, FLAG_IDENTITY_EXPIRED};
use crate::tx::transaction::RotationRevealTx;

/// Apply rotation reveal (FULL HYBRID KEY REPLACEMENT)
pub fn apply_rotation_reveal(
    accounts: &mut Vec<Account>,
    tx: &RotationRevealTx,
) -> Result<(), String> {
    let acc = accounts
        .iter_mut()
        .find(|a| a.current_classical_pubkey == tx.from)
        .ok_or("Account not found")?;

    // 🔐 Enforce nonce
    if tx.nonce != acc.nonce {
        return Err("Invalid nonce".into());
    }

    acc.nonce += 1;

    // 🔐 Must have a commit
    let commitment = acc
        .rotation_commitment
        .clone()
        .ok_or("No commit found")?;

    // 🔐 Rebuild commitment
    let mut hasher = blake3::Hasher::new();
    hasher.update(&tx.new_classical_key);
    hasher.update(&tx.new_pq_key);

    let calculated = hasher.finalize().as_bytes().to_vec();

    if calculated != commitment {
        return Err("Commitment mismatch".into());
    }

    // 🔐 Replace BOTH keys
    acc.current_classical_pubkey = tx.new_classical_key.clone();
    acc.current_pq_pubkey = tx.new_pq_key.clone();

    // 🔄 Advance epoch
    acc.rotation_epoch += 1;

    // 🧹 Clear rotation state
    acc.rotation_commitment = None;
    acc.rotation_deadline_tick = None;

    // 🔥 Reset expiration
    acc.clear_flag(FLAG_IDENTITY_EXPIRED);

    Ok(())
}
use blake3;

use crate::state::account::{Account, FLAG_IDENTITY_EXPIRED};
use crate::tx::transaction::RotationRevealTx;
use crate::pq;

/// Apply rotation reveal (FULL HYBRID KEY REPLACEMENT + PHASE 3 RULES)
pub fn apply_rotation_reveal(
    accounts: &mut Vec<Account>,
    tx: &RotationRevealTx,
) -> Result<(), String> {
    let acc = accounts
        .iter_mut()
        .find(|a| a.current_classical_pubkey == tx.from)
        .ok_or("Account not found")?;

    // -----------------------------
    // 🔐 NONCE
    // -----------------------------
    if tx.nonce != acc.nonce {
        return Err("Invalid nonce".into());
    }

    acc.nonce += 1;

    // -----------------------------
    // 🔁 FORK PREVENTION (MOVE UP)
    // -----------------------------
    if tx.epoch <= acc.rotation_epoch {
        return Err("FORK_DETECTED".into());
    }

    // -----------------------------
    // ⏳ EXPIRATION
    // -----------------------------
    if acc.has_flag(FLAG_IDENTITY_EXPIRED) {
        return Err("IDENTITY_EXPIRED".into());
    }

    // -----------------------------
    // 🔁 CONTINUITY
    // -----------------------------
    let continuity_valid = pq::verify(
        &tx.new_pq_key,
        &tx.link_signature,
        &acc.current_pq_pubkey,
    );

    if !continuity_valid {
        return Err("INVALID_LINK_SIGNATURE".into());
    }

    // -----------------------------
    // 🔐 COMMIT CHECK
    // -----------------------------
    let commitment = acc
        .rotation_commitment
        .clone()
        .ok_or("No commit found")?;

    let mut hasher = blake3::Hasher::new();
    hasher.update(&tx.new_classical_key);
    hasher.update(&tx.new_pq_key);

    let calculated = hasher.finalize().as_bytes().to_vec();

    if calculated != commitment {
        return Err("Commitment mismatch".into());
    }

    // -----------------------------
    // 🔐 APPLY KEYS
    // -----------------------------
    acc.current_classical_pubkey = tx.new_classical_key.clone();
    acc.current_pq_pubkey = tx.new_pq_key.clone();

    acc.rotation_epoch = tx.epoch;

    acc.rotation_commitment = None;
    acc.rotation_deadline_tick = None;

    acc.clear_flag(FLAG_IDENTITY_EXPIRED);

    Ok(())
}
use blake3;

use crate::state::account::Account;
use crate::tx::transaction::RotationRevealTx;

/// Apply rotation reveal (commit → reveal → switch)
pub fn apply_rotation_reveal(
    accounts: &mut Vec<Account>,
    tx: &RotationRevealTx,
) -> Result<(), String> {
    let mut sender_index = None;

    for (i, acc) in accounts.iter().enumerate() {
        if acc.current_classical_pubkey == tx.from {
            sender_index = Some(i);
        }
    }

    let sender_i = sender_index.ok_or("Account not found")?;
    let sender = &mut accounts[sender_i];

    // nonce check
    if sender.nonce != tx.nonce {
        return Err("Invalid nonce".into());
    }

    // must have prior commitment
    let commitment = sender
        .rotation_commitment
        .clone()
        .ok_or("No rotation commitment")?;

    // hash new key
    let mut hasher = blake3::Hasher::new();
    hasher.update(&tx.new_classical_key);
    let computed = hasher.finalize();

    if commitment != computed.as_bytes() {
        return Err("Commitment mismatch".into());
    }

    // 🔥 IMMEDIATE KEY SWITCH
    sender.current_classical_pubkey = tx.new_classical_key.clone();
    sender.current_pq_pubkey = tx.new_pq_key.clone();

    // clear rotation state
    sender.rotation_commitment = None;
    sender.rotation_deadline_tick = None;

    sender.rotation_epoch += 1;
    sender.nonce += 1;

    Ok(())
}
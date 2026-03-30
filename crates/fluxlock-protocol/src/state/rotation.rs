use crate::state::account::Account;
use crate::tx::transaction::RotationCommitTx;

/// Apply rotation commit
pub fn apply_rotation_commit(
    accounts: &mut Vec<Account>,
    tx: &RotationCommitTx,
    current_tick: u64,
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

    // 🔐 Store commitment
    acc.rotation_commitment = Some(tx.new_key_commitment.clone());

    // ⏱ Set deadline (10 ticks window)
    acc.rotation_deadline_tick = Some(current_tick + 10);

    Ok(())
}
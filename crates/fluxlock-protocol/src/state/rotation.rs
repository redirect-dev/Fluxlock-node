use crate::state::account::Account;
use crate::tx::transaction::RotationCommitTx;

/// Apply rotation commit
pub fn apply_rotation_commit(
    accounts: &mut Vec<Account>,
    tx: &RotationCommitTx,
    current_tick: u64,
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

    // set commitment
    sender.rotation_commitment = Some(tx.new_key_commitment.clone());

    // set deadline (simple window)
    sender.rotation_deadline_tick = Some(current_tick + 10);

    sender.nonce += 1;

    Ok(())
}
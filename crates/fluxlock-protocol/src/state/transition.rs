use crate::state::account::Account;
use crate::tx::transaction::TransferTx;
use crate::tx::verify::verify_transfer;

/// Apply a transfer transaction to accounts
pub fn apply_transfer(
    accounts: &mut Vec<Account>,
    tx: &TransferTx,
    current_tick: u64,
) -> Result<(), String> {
    // 🔐 Verify signature
    if !verify_transfer(tx) {
        return Err("Invalid signature".into());
    }

    let mut sender_index = None;
    let mut receiver_index = None;

    for (i, acc) in accounts.iter().enumerate() {
        if acc.current_classical_pubkey == tx.from {
            sender_index = Some(i);
        }
        if acc.current_classical_pubkey == tx.to {
            receiver_index = Some(i);
        }
    }

    let sender_i = sender_index.ok_or("Sender not found")?;
    let receiver_i = receiver_index.ok_or("Receiver not found")?;

    let sender = &mut accounts[sender_i];

    // 🔥 ENFORCEMENT FIX (inclusive deadline)
    if let Some(deadline) = sender.rotation_deadline_tick {
        if current_tick >= deadline {
            return Err("Rotation required before transfer".into());
        }
    }

    // Nonce check
    if sender.nonce != tx.nonce {
        return Err("Invalid nonce".into());
    }

    // Balance check
    if sender.balance < tx.amount {
        return Err("Insufficient balance".into());
    }

    // Apply transfer
    sender.balance -= tx.amount;
    sender.nonce += 1;

    let receiver = &mut accounts[receiver_i];
    receiver.balance += tx.amount;

    Ok(())
}
use crate::state::account::Account;
use crate::tx::transaction::TransferTx;

/// Apply transfer (ASSUMES already verified)
pub fn apply_transfer(
    accounts: &mut Vec<Account>,
    tx: &TransferTx,
    _current_tick: u64,
) -> Result<(), String> {
    let sender = accounts
        .iter_mut()
        .find(|a| a.current_classical_pubkey == tx.from)
        .ok_or("Sender not found")?;

    // 🔐 Nonce check
    if tx.nonce != sender.nonce {
        return Err("Invalid nonce".into());
    }

    // 🔐 Balance check
    if sender.balance < tx.amount {
        return Err("Insufficient balance".into());
    }

    sender.balance -= tx.amount;
    sender.nonce += 1;

    let receiver = accounts
        .iter_mut()
        .find(|a| a.current_classical_pubkey == tx.to)
        .ok_or("Receiver not found")?;

    receiver.balance += tx.amount;

    Ok(())
}
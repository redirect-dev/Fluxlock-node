use crate::block::structure::{Block, HybridSignature};

use crate::state::account::Account;
use crate::state::validator::Validator;

use crate::tx::transaction::Tx;
use crate::tx::verify::{
    verify_transfer,
    verify_rotation_commit,
    verify_rotation_reveal,
};

pub fn produce_block(
    prev_block: &Block,
    validators: &mut Vec<Validator>,
    accounts: &mut Vec<Account>,
    txs: Vec<Tx>,
    counter: u64,
) -> Result<(Block, u64), String> {

    let mut new_counter = counter;

    for tx in txs.iter() {
        match tx {

            // -----------------------------
            // TRANSFER
            // -----------------------------
            Tx::Transfer(t) => {
                let sender = match accounts.iter_mut()
                    .find(|a| a.current_classical_pubkey == t.from)
                {
                    Some(acc) => acc,
                    None => {
                        slash(validators);
                        return Err("Transaction rejected: identity no longer valid".into());
                    }
                };

                // 🔐 FIX: correct verification (classical key + epoch-bound sig)
                if !verify_transfer(&sender.current_classical_pubkey, t) {
                    slash(validators);
                    return Err("Transaction rejected: invalid signature or epoch".into());
                }

                if t.nonce != sender.nonce {
                    slash(validators);
                    return Err("Transaction rejected: invalid nonce".into());
                }

                if sender.balance < t.amount {
                    slash(validators);
                    return Err("Transaction rejected: insufficient balance".into());
                }

                sender.balance -= t.amount;
                sender.nonce += 1;

                if let Some(receiver) = accounts.iter_mut()
                    .find(|a| a.current_classical_pubkey == t.to)
                {
                    receiver.balance += t.amount;
                }
            }

            // -----------------------------
            // ROTATION COMMIT
            // -----------------------------
            Tx::RotationCommit(r) => {
                let sender = match accounts.iter_mut()
                    .find(|a| a.current_classical_pubkey == r.from)
                {
                    Some(acc) => acc,
                    None => {
                        slash(validators);
                        return Err("Transaction rejected: identity no longer valid".into());
                    }
                };

                if !verify_rotation_commit(&sender.current_classical_pubkey, r) {
                    slash(validators);
                    return Err("Transaction rejected: invalid rotation commit".into());
                }

                if r.nonce != sender.nonce {
                    slash(validators);
                    return Err("Transaction rejected: invalid nonce".into());
                }

                sender.nonce += 1;
            }

            // -----------------------------
            // ROTATION REVEAL
            // -----------------------------
            Tx::RotationReveal(r) => {
                let sender = match accounts.iter_mut()
                    .find(|a| a.current_classical_pubkey == r.from)
                {
                    Some(acc) => acc,
                    None => {
                        slash(validators);
                        return Err("Transaction rejected: identity no longer valid".into());
                    }
                };

                if !verify_rotation_reveal(&sender.current_classical_pubkey, r) {
                    slash(validators);
                    return Err("Transaction rejected: invalid rotation reveal".into());
                }

                if r.nonce != sender.nonce {
                    slash(validators);
                    return Err("Transaction rejected: invalid nonce".into());
                }

                // 🔐 Identity rotation
                sender.current_classical_pubkey = r.new_classical_key.clone();
                sender.current_pq_pubkey = r.new_pq_key.clone();

                // 🔥 Time progression
                sender.rotation_epoch = r.epoch;

                sender.nonce += 1;
            }
        }
    }

    // -----------------------------
    // BUILD BLOCK
    // -----------------------------
    let next_block = Block {
        parent_hash: prev_block.parent_hash.clone(),
        tick: prev_block.tick + 1,

        state_root: [0u8; 32],
        tx_root: [0u8; 32],
        txs: vec![],

        validator_classical_key: vec![0; 32],

        signature: HybridSignature {
            ed25519_sig: vec![0; 64],
            dilithium_sig: vec![],
        },
    };

    new_counter += 1;

    Ok((next_block, new_counter))
}

// 🔥 Validator slashing
fn slash(validators: &mut Vec<Validator>) {
    if let Some(v) = validators.first_mut() {
        let penalty = (v.stake as f64 * 0.05) as u128;
        v.slash(penalty);
    }
}
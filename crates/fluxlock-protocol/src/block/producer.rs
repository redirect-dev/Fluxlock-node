use blake3;

use crate::block::structure::{Block, HybridSignature};
use crate::state::validator::Validator;
use crate::state::account::{Account, FLAG_IDENTITY_EXPIRED};
use crate::state::transition::apply_transfer;
use crate::state::rotation::apply_rotation_commit;
use crate::state::reveal::apply_rotation_reveal;
use crate::state::hasher::hash_accounts;

use crate::tx::transaction::Tx;
use crate::tx::verify::{
    verify_transfer,
    verify_rotation_commit,
    verify_rotation_reveal,
};

/// 🔥 Epoch config
const EPOCH_LENGTH: u64 = 5;

/// Select validator
pub fn select_validator(validators: &Vec<Validator>, tick: u64) -> (usize, Validator) {
    let index = (tick as usize) % validators.len();
    (index, validators[index].clone())
}

/// 🔥 Time effects (unchanged)
fn apply_time_effects(accounts: &mut Vec<Account>, current_tick: u64) {
    for acc in accounts.iter_mut() {
        if let Some(deadline) = acc.rotation_deadline_tick {
            if current_tick >= deadline {
                acc.set_flag(FLAG_IDENTITY_EXPIRED);
            }
        }

        if acc.has_flag(FLAG_IDENTITY_EXPIRED) {
            let decay = acc.balance / 100;
            acc.balance -= decay;
        }
    }
}

/// 🔥 Epoch enforcement (NEW)
fn enforce_epoch(account: &Account, current_tick: u64) -> Result<(), String> {
    let required_epoch = current_tick / EPOCH_LENGTH;

    if account.rotation_epoch < required_epoch {
        return Err("Account not rotated for current epoch".into());
    }

    Ok(())
}

/// 🔥 Slash validator
fn slash_validator(validators: &mut Vec<Validator>, index: usize) {
    let v = &mut validators[index];
    let slash = v.stake / 20;
    v.stake -= slash;

    println!("⚔️ Validator slashed! New stake: {}", v.stake);
}

/// Produce block
pub fn produce_block(
    previous_block: &Block,
    validators: &mut Vec<Validator>,
    accounts: &mut Vec<Account>,
    txs: Vec<Tx>,
    state_counter: u64,
) -> Result<(Block, u64), String> {
    let next_tick = previous_block.tick + 1;

    let (validator_index, validator) = select_validator(validators, next_tick);

    // 🔥 STEP 1 — time always moves
    apply_time_effects(accounts, next_tick);

    // 🔥 STEP 2 — simulate
    let mut temp_accounts = accounts.clone();

    for tx in &txs {
        let result = match tx {
            Tx::Transfer(t) => {
                let sender = temp_accounts.iter()
                    .find(|a| a.current_classical_pubkey == t.from)
                    .ok_or("Sender not found")?;

                // 🔥 NEW — epoch enforcement
                enforce_epoch(sender, next_tick)?;

                if !verify_transfer(t, &sender.current_pq_pubkey) {
                    slash_validator(validators, validator_index);
                    return Err("Invalid transfer signature".into());
                }

                apply_transfer(&mut temp_accounts, t, next_tick)
            }

            Tx::RotationCommit(r) => {
                let sender = temp_accounts.iter()
                    .find(|a| a.current_classical_pubkey == r.from)
                    .ok_or("Sender not found")?;

                if !verify_rotation_commit(r, &sender.current_pq_pubkey) {
                    slash_validator(validators, validator_index);
                    return Err("Invalid rotation commit signature".into());
                }

                apply_rotation_commit(&mut temp_accounts, r, next_tick)
            }

            Tx::RotationReveal(r) => {
                let sender = temp_accounts.iter()
                    .find(|a| a.current_classical_pubkey == r.from)
                    .ok_or("Sender not found")?;

                if !verify_rotation_reveal(r, &sender.current_pq_pubkey) {
                    slash_validator(validators, validator_index);
                    return Err("Invalid rotation reveal signature".into());
                }

                apply_rotation_reveal(&mut temp_accounts, r)
            }
        };

        if result.is_err() {
            slash_validator(validators, validator_index);
            return Err("Block rejected: invalid transaction".into());
        }
    }

    // 🔥 commit
    *accounts = temp_accounts;

    let new_counter = state_counter + 1;

    let new_state_root = hash_accounts(accounts);

    let new_block = Block {
        parent_hash: hash_block(previous_block),
        tick: next_tick,
        state_root: new_state_root,
        tx_root: [0u8; 32],
        txs: txs.clone(),
        validator_classical_key: validator.classical_pubkey.clone(),
        signature: HybridSignature {
            ed25519_sig: vec![],
            dilithium_sig: vec![],
        },
    };

    Ok((new_block, new_counter))
}

/// Hash block
fn hash_block(block: &Block) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();

    hasher.update(&block.tick.to_le_bytes());
    hasher.update(&block.state_root);

    let hash = hasher.finalize();

    *hash.as_bytes()
}
use blake3;

use crate::block::structure::{Block, HybridSignature};
use crate::state::validator::Validator;
use crate::state::account::{Account, FLAG_IDENTITY_EXPIRED};
use crate::state::transition::apply_transfer;
use crate::state::rotation::apply_rotation_commit;
use crate::state::reveal::apply_rotation_reveal;
use crate::state::hasher::hash_accounts;

use crate::tx::transaction::Tx;

/// Select validator using round-robin (returns index too)
pub fn select_validator(validators: &Vec<Validator>, tick: u64) -> (usize, Validator) {
    let index = (tick as usize) % validators.len();
    (index, validators[index].clone())
}

/// 🔥 APPLY TIME EFFECTS (ALWAYS RUNS)
fn apply_time_effects(accounts: &mut Vec<Account>, current_tick: u64) {
    for acc in accounts.iter_mut() {
        // expiration
        if let Some(deadline) = acc.rotation_deadline_tick {
            if current_tick >= deadline {
                acc.set_flag(FLAG_IDENTITY_EXPIRED);
            }
        }

        // decay
        if acc.has_flag(FLAG_IDENTITY_EXPIRED) {
            let decay = acc.balance / 100;
            acc.balance -= decay;
        }
    }
}

/// 🔥 SLASH VALIDATOR (5%)
fn slash_validator(validators: &mut Vec<Validator>, index: usize) {
    let v = &mut validators[index];
    let slash = v.stake / 20; // 5%
    v.stake -= slash;

    println!("⚔️ Validator slashed! New stake: {}", v.stake);
}

/// Produce next block with time + validation + slashing
pub fn produce_block(
    previous_block: &Block,
    validators: &mut Vec<Validator>, // 🔥 now mutable
    accounts: &mut Vec<Account>,
    txs: Vec<Tx>,
    state_counter: u64,
) -> Result<(Block, u64), String> {
    let next_tick = previous_block.tick + 1;

    let (validator_index, validator) = select_validator(validators, next_tick);

    // 🔥 STEP 1: TIME ALWAYS ADVANCES
    apply_time_effects(accounts, next_tick);

    // 🔥 STEP 2: SIMULATE TX EXECUTION
    let mut temp_accounts = accounts.clone();

    for tx in &txs {
        let result = match tx {
            Tx::Transfer(t) => apply_transfer(&mut temp_accounts, t, next_tick),
            Tx::RotationCommit(r) => apply_rotation_commit(&mut temp_accounts, r, next_tick),
            Tx::RotationReveal(r) => apply_rotation_reveal(&mut temp_accounts, r),
        };

        if result.is_err() {
            // 🔥 SLASH VALIDATOR
            slash_validator(validators, validator_index);

            return Err("Block rejected: invalid transaction".into());
        }
    }

    // 🔥 STEP 3: COMMIT STATE
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
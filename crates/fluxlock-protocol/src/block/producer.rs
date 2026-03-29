use blake3;

use crate::block::structure::{Block, HybridSignature};
use crate::state::validator::Validator;
use crate::state::account::Account;
use crate::state::transition::apply_transfer;
use crate::state::hasher::hash_accounts;
use crate::tx::transaction::TransferTx;

/// Select validator using round-robin
pub fn select_validator(validators: &Vec<Validator>, tick: u64) -> Validator {
    let index = (tick as usize) % validators.len();
    validators[index].clone()
}

/// Produce next block with full state hashing
pub fn produce_block(
    previous_block: &Block,
    validators: &Vec<Validator>,
    accounts: &mut Vec<Account>,
    txs: Vec<TransferTx>,
    state_counter: u64,
) -> (Block, u64) {
    let next_tick = previous_block.tick + 1;

    let validator = select_validator(validators, next_tick);

    // 🔥 Apply transactions
    for tx in &txs {
        let _ = apply_transfer(accounts, tx);
    }

    // Temporary counter (still exists but no longer used for hashing)
    let new_counter = state_counter + 1;

    // 🔐 REAL STATE ROOT
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

    (new_block, new_counter)
}

/// Hash the block (minimal version)
fn hash_block(block: &Block) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();

    hasher.update(&block.tick.to_le_bytes());
    hasher.update(&block.state_root);

    let hash = hasher.finalize();

    *hash.as_bytes()
}
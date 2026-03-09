use crate::block::structure::{Block, HybridSignature};
use crate::state::validator::Validator;

/// Select validator using round-robin
pub fn select_validator(validators: &Vec<Validator>, tick: u64) -> Validator {
    let index = (tick as usize) % validators.len();
    validators[index].clone()
}

/// Produce next block (minimal version)
pub fn produce_block(
    previous_block: &Block,
    validators: &Vec<Validator>,
) -> Block {
    let next_tick = previous_block.tick + 1;

    let validator = select_validator(validators, next_tick);

    // For now:
    // - No transactions
    // - State root stays the same
    // - Signatures are placeholders

    Block {
        parent_hash: hash_block(previous_block),

        tick: next_tick,

        state_root: previous_block.state_root,

        tx_root: [0u8; 32],

        validator_classical_key: validator.classical_pubkey.clone(),

        signature: HybridSignature {
            ed25519_sig: vec![],
            dilithium_sig: vec![],
        },
    }
}

/// Temporary block hash (placeholder)
fn hash_block(_block: &Block) -> [u8; 32] {
    // TODO: replace with real hashing (Blake3 later)
    [0u8; 32]
}
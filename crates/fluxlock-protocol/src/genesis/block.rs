use crate::block::structure::{Block, HybridSignature};
use crate::genesis::state::GenesisState;

pub fn build_genesis_block(genesis_state: &GenesisState) -> Block {
    Block {
        parent_hash: [0u8; 32],

        tick: 0,

        state_root: genesis_state.state_root,

        tx_root: [0u8; 32],

        validator_classical_key: vec![],

        signature: HybridSignature {
            ed25519_sig: vec![],
            dilithium_sig: vec![],
        },
    }
}

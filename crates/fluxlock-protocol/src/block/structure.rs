use crate::tx::transaction::Tx;

#[derive(Clone, Debug)]
pub struct HybridSignature {
    pub ed25519_sig: Vec<u8>,
    pub dilithium_sig: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct Block {
    pub parent_hash: [u8; 32],

    pub tick: u64,

    pub state_root: [u8; 32],
    pub tx_root: [u8; 32],

    pub txs: Vec<Tx>, // 🔥 UPDATED

    pub validator_classical_key: Vec<u8>,

    pub signature: HybridSignature,
}
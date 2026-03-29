#[derive(Clone, Debug)]
pub struct TransferTx {
    pub from: Vec<u8>,
    pub to: Vec<u8>,
    pub amount: u128,
    pub nonce: u64,
    pub signature: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct RotationCommitTx {
    pub from: Vec<u8>,
    pub new_key_commitment: Vec<u8>,
    pub nonce: u64,
    pub signature: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct RotationRevealTx {
    pub from: Vec<u8>,

    // actual new key
    pub new_classical_key: Vec<u8>,
    pub new_pq_key: Vec<u8>,

    pub nonce: u64,
    pub signature: Vec<u8>,
}

#[derive(Clone, Debug)]
pub enum Tx {
    Transfer(TransferTx),
    RotationCommit(RotationCommitTx),
    RotationReveal(RotationRevealTx),
}
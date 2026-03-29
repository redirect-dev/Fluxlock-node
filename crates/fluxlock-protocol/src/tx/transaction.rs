#[derive(Clone, Debug)]
pub struct TransferTx {
    pub from: Vec<u8>,
    pub to: Vec<u8>,
    pub amount: u128,
    pub nonce: u64,
}
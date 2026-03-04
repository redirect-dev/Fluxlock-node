#[derive(Clone, Debug)]
pub struct Account {
    pub balance: u128,
    pub nonce: u64,

    pub current_classical_pubkey: Vec<u8>,
    pub current_pq_pubkey: Vec<u8>,

    pub rotation_epoch: u64,
    pub rotation_commitment: Option<Vec<u8>>,
    pub rotation_deadline_tick: Option<u64>,

    pub flags: u32,
}

impl Account {
    pub fn new(
        balance: u128,
        classical_pubkey: Vec<u8>,
        pq_pubkey: Vec<u8>,
    ) -> Self {
        Self {
            balance,
            nonce: 0,
            current_classical_pubkey: classical_pubkey,
            current_pq_pubkey: pq_pubkey,
            rotation_epoch: 0,
            rotation_commitment: None,
            rotation_deadline_tick: None,
            flags: 0,
        }
    }
}

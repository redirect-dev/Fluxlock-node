use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyState {
    pub current_classical_pubkey: Option<Vec<u8>>,
    pub current_pq_pubkey: Option<Vec<u8>>,

    pub pending_classical_commitment: Option<Vec<u8>>,
    pub pending_pq_commitment: Option<Vec<u8>>,

    pub commitment_tick: Option<u64>,
}

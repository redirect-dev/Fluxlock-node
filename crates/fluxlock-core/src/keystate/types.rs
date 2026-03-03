use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyState {
    pub current_pubkey: Option<Vec<u8>>,
    pub pending_commitment: Option<Vec<u8>>,
    pub commitment_tick: Option<u64>,
}

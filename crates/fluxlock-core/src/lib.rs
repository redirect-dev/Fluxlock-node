pub mod keystate;

use serde::{Serialize, Deserialize};
use keystate::KeyState;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EngineCompositeState {
    pub key_state: KeyState,
}

impl EngineCompositeState {
    pub fn new() -> Self {
        Self {
            key_state: KeyState {
                current_classical_pubkey: None,
                current_pq_pubkey: None,

                pending_classical_commitment: None,
                pending_pq_commitment: None,

                commitment_tick: None,
            },
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TickInput {
    pub commit_classical: Option<Vec<u8>>,
    pub commit_pq: Option<Vec<u8>>,

    pub reveal_classical: Option<Vec<u8>>,
    pub reveal_pq: Option<Vec<u8>>,

    pub classical_signature: Option<Vec<u8>>,
    pub pq_signature: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TickRecord {
    pub tick_index: u64,
    pub input: TickInput,
    pub state: EngineCompositeState,
    pub parent_hash: String,
    pub state_hash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TickLog {
    pub records: Vec<TickRecord>,
}

impl TickLog {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }
}

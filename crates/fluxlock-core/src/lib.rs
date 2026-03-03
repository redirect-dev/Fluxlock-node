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
                current_pubkey: None,
                pending_commitment: None,
                commitment_tick: None,
            },
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TickInput {
    pub commit_pubkey: Option<Vec<u8>>,
    pub reveal_pubkey: Option<Vec<u8>>,
    pub payload: Option<Vec<u8>>,
    pub signature: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TickRecord {
    pub tick_index: u64,
    pub input: TickInput,
    pub state: EngineCompositeState,
    pub parent_hash: String,
    pub state_hash: String,
    pub signature: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TickLog {
    pub records: Vec<TickRecord>,
}

impl TickLog {
    pub fn new() -> Self {
        Self { records: Vec::new() }
    }
}

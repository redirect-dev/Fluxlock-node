use serde::{Serialize, Deserialize};

pub mod keystate;

use keystate::{KeyState, RotationPolicy, AlgorithmId};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrustState {
    pub trust_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LifecycleState {
    pub stage: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LockState {
    pub level: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecoveryState {
    pub is_recovering: bool,
    pub recovery_ticks: u32,
    pub grace_ticks_remaining: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EngineCompositeState {
    pub trust: TrustState,
    pub lifecycle: LifecycleState,
    pub lock: LockState,
    pub recovery: RecoveryState,
    pub key_state: KeyState,
}

impl EngineCompositeState {
    pub fn new() -> Self {
        Self {
            trust: TrustState { trust_score: 100.0 },
            lifecycle: LifecycleState { stage: 0 },
            lock: LockState { level: 0 },
            recovery: RecoveryState {
                is_recovering: false,
                recovery_ticks: 0,
                grace_ticks_remaining: 0,
            },
            key_state: KeyState {
                key_epoch: 0,
                activated_at_tick: 0,
                algorithm: AlgorithmId::Ed25519,
                current_pubkey: vec![],
                next_pubkey_commitment: None,
                rotation_policy: RotationPolicy { epoch_length: 100 },
                rotation_override: false,
                not_before_tick: 0,
                not_after_tick: None,
                parent_key_hash: None,
            },
        }
    }
}

#[derive(Debug)]
pub enum InvariantViolation {
    LockDecreased,
    LifecycleRegression,
    TrustWentNegative,
    TrustIncreasedOutsideRecovery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickInput {
    pub revealed_pubkey: Option<Vec<u8>>,

    // NEW: signature authorizing this input
    pub signature: Option<Vec<u8>>,

    pub payload: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickRecord {
    pub tick_index: u64,
    pub input: TickInput,
    pub state: EngineCompositeState,
    pub parent_hash: String,
    pub state_hash: String,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickLog {
    pub records: Vec<TickRecord>,
}

impl TickLog {
    pub fn new() -> Self {
        Self { records: Vec::new() }
    }
}

use serde::{Serialize, Deserialize};
pub mod keystate;

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
        }
    }
}

/* ================================
   Invariants (AUTHORITATIVE)
   ================================ */

#[derive(Debug)]
pub enum InvariantViolation {
    LockDecreased,
    LifecycleRegression,
    TrustWentNegative,
    TrustIncreasedOutsideRecovery,
}

/* ================================
   Tick Log
   ================================ */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickRecord {
    pub tick_index: u64,
    pub state: EngineCompositeState,
    pub parent_hash: String,
    pub state_hash: String,
    pub signature: Option<String>, // Phase 2.5
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

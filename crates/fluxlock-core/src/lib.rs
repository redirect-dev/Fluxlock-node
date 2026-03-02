use serde::{Serialize, Deserialize};

/// ============================
/// Trust State
/// ============================
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrustState {
    pub trust_score: f64,
}

impl TrustState {
    pub fn new() -> Self {
        Self { trust_score: 100.0 }
    }

    pub fn decay(&mut self, amount: f64) {
        self.trust_score = (self.trust_score - amount).max(0.0);
    }

    pub fn recover(&mut self, amount: f64) {
        self.trust_score = (self.trust_score + amount).min(100.0);
    }
}

/// ============================
/// Lifecycle State
/// ============================
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LifecycleState {
    pub stage: u8, // 0=Active, 1=Degraded, 2=Quarantined
}

impl LifecycleState {
    pub fn new() -> Self {
        Self { stage: 0 }
    }
}

/// ============================
/// Lock State
/// ============================
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LockState {
    pub level: u8, // 0=Unlocked, 1=Restricted, 2=Locked
}

impl LockState {
    pub fn new() -> Self {
        Self { level: 0 }
    }
}

/// ============================
/// Recovery State
/// ============================
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecoveryState {
    pub is_recovering: bool,
    pub recovery_ticks: u32,
    pub grace_ticks_remaining: u32,
}

impl RecoveryState {
    pub fn new() -> Self {
        Self {
            is_recovering: false,
            recovery_ticks: 0,
            grace_ticks_remaining: 0,
        }
    }
}

/// ============================
/// Composite Engine State
/// ============================
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
            trust: TrustState::new(),
            lifecycle: LifecycleState::new(),
            lock: LockState::new(),
            recovery: RecoveryState::new(),
        }
    }
}

/// ============================
/// Deterministic Tick Clock
/// ============================
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct TickClock {
    pub tick: u64,
}

impl TickClock {
    pub fn new() -> Self {
        Self { tick: 0 }
    }

    pub fn advance(&mut self) {
        self.tick += 1;
    }
}

/// ============================
/// Tick Record (Replay Primitive)
/// ============================
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TickRecord {
    pub tick: u64,
    pub state: EngineCompositeState,
}

/// ============================
/// Tick Log (Replay Journal)
/// ============================
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TickLog {
    pub records: Vec<TickRecord>,
}

impl TickLog {
    pub fn new() -> Self {
        Self { records: Vec::new() }
    }

    pub fn push(&mut self, tick: u64, state: &EngineCompositeState) {
        self.records.push(TickRecord {
            tick,
            state: state.clone(),
        });
    }
}

/// Fluxlock Core State Definitions

// ===============================
// TRUST STATE
// ===============================

#[derive(Debug, Clone)]
pub struct TrustState {
    pub trust_score: f64,
}

impl TrustState {
    pub fn new(initial_score: f64) -> Self {
        Self {
            trust_score: initial_score,
        }
    }

    pub fn decay(&mut self, amount: f64) {
        self.trust_score -= amount;

        if self.trust_score < 0.0 {
            self.trust_score = 0.0;
        }
    }
}

// ===============================
// LIFECYCLE STATE
// ===============================

#[derive(Debug, Clone)]
pub struct LifecycleState {
    pub stage: u8,
}

impl LifecycleState {
    pub fn new() -> Self {
        Self { stage: 0 }
    }
}

// ===============================
// LOCK STATE
// ===============================

#[derive(Debug, Clone)]
pub struct LockState {
    pub level: u8,
}

impl LockState {
    pub fn new() -> Self {
        Self { level: 0 }
    }
}

// ===============================
// RECOVERY STATE
// ===============================

#[derive(Debug, Clone)]
pub struct RecoveryState {
    pub is_recovering: bool,
    pub recovery_ticks: u64,
    pub grace_ticks_remaining: u64,
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

// ===============================
// TICK CLOCK
// ===============================

#[derive(Debug, Clone)]
pub struct TickClock {
    pub current_tick: u64,
}

impl TickClock {
    pub fn new() -> Self {
        Self { current_tick: 0 }
    }

    pub fn advance(&mut self) {
        self.current_tick += 1;
    }
}

// ===============================
// COMPOSITE ENGINE STATE
// ===============================

#[derive(Debug, Clone)]
pub struct EngineCompositeState {
    pub trust: TrustState,
    pub lifecycle: LifecycleState,
    pub lock: LockState,
    pub recovery: RecoveryState,
}

impl EngineCompositeState {
    pub fn new() -> Self {
        Self {
            trust: TrustState::new(100.0),
            lifecycle: LifecycleState::new(),
            lock: LockState::new(),
            recovery: RecoveryState::new(),
        }
    }
}

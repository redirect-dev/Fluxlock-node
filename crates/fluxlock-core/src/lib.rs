/// Fluxlock Core State Definitions

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

/// Lifecycle State (very early placeholder version)
#[derive(Debug, Clone)]
pub struct LifecycleState {
    pub stage: u8,
}

impl LifecycleState {
    pub fn new() -> Self {
        Self { stage: 0 }
    }
}

/// Composite Engine State
///
/// This is the beginning of the full protocol state container.
#[derive(Debug, Clone)]
pub struct EngineCompositeState {
    pub trust: TrustState,
    pub lifecycle: LifecycleState,
}

impl EngineCompositeState {
    pub fn new() -> Self {
        Self {
            trust: TrustState::new(100.0),
            lifecycle: LifecycleState::new(),
        }
    }
}

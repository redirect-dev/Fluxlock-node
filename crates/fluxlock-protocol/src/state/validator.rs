use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Validator {
    pub name: String,
    pub stake: u128,
    pub reputation: i32,
    pub violations: u32, // 🔥 NEW: tracks repeated failures
}

impl Validator {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            stake: 1000,
            reputation: 100,
            violations: 0,
        }
    }

    // 🔥 NONLINEAR SLASHING
    pub fn slash(&mut self, amount: u128) {
        self.stake = self.stake.saturating_sub(amount);

        // increase violation count
        self.violations += 1;

        // escalating penalty
        let penalty = match self.violations {
            1 => 7,
            2 => 12,
            3 => 18,
            _ => 25, // heavy collapse after repeated failures
        };

        self.reputation = (self.reputation - penalty).max(0);
    }

    // 🔥 SLOW RECOVERY
    pub fn reward(&mut self) {
        if self.reputation < 100 {
            self.reputation += 2; // slow recovery
        }

        // gradually reduce violation count (forgiveness over time)
        if self.violations > 0 {
            self.violations -= 1;
        }
    }

    // 🔥 EXILE LOGIC
    pub fn is_exiled(&self) -> bool {
        self.reputation < 20
    }

    pub fn status(&self) -> String {
        if self.is_exiled() {
            "Exiled".to_string()
        } else if self.reputation < 60 {
            "Degraded".to_string()
        } else {
            "Healthy".to_string()
        }
    }
}
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Validator {
    pub name: String,
    pub stake: u128,
    pub reputation: i32,
    pub violations: u32,
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

    pub fn slash(&mut self, amount: u128) {
        self.stake = self.stake.saturating_sub(amount);

        self.violations += 1;

        // 🔥 TRUE ESCALATION (no reset)
        let penalty = match self.violations {
            1 => 8,
            2 => 15,
            3 => 25,
            _ => 40,
        };

        self.reputation = (self.reputation - penalty).max(0);
    }

    pub fn reward(&mut self) {
        // 🔥 VERY SLOW RECOVERY
        if self.reputation < 100 {
            self.reputation += 1;
        }

        // 🔥 NO VIOLATION RESET (this is the key change)
    }

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
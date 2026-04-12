use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Validator {
    pub name: String,
    pub stake: u128,
    pub reputation: i32,
    pub violations: u32,
    pub recent_success: u32,
}

pub const MIN_REENTRY_STAKE: u128 = 200;

impl Validator {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            stake: 1000,
            reputation: 100,
            violations: 0,
            recent_success: 0,
        }
    }

    pub fn slash(&mut self, amount: u128) {
        self.stake = self.stake.saturating_sub(amount);
        self.violations += 1;
        self.recent_success = 0;

        let penalty = match self.violations {
            1 => 10,
            2 => 20,
            3 => 30,
            _ => 50,
        };

        self.reputation = (self.reputation - penalty).max(0);
    }

    pub fn reward(&mut self) {
        self.recent_success += 1;

        if self.recent_success % 4 == 0 && self.violations > 0 {
            self.violations -= 1;
        }

        let bonus = match self.recent_success {
            3 => 5,
            5 => 10,
            8 => 15,
            12 => 20,
            _ => 0,
        };

        if bonus > 0 {
            self.reputation = (self.reputation + bonus).min(100);
        }
    }

    pub fn restake(&mut self, amount: u128) {
        println!("DEBUG: restake called for {}", self.name);
        self.stake += amount;
    }

    pub fn is_exiled(&self) -> bool {
        self.reputation < 20
    }

    pub fn can_participate(&self) -> bool {
        self.reputation >= 20 && self.stake >= MIN_REENTRY_STAKE
    }

    pub fn status(&self) -> String {
        if self.reputation < 20 {
            return "EXILED".to_string();
        }

        if self.stake == 0 {
            return "BANKRUPT".to_string();
        }

        if self.stake < MIN_REENTRY_STAKE {
            return "RECOVERING".to_string();
        }

        if self.reputation < 60 {
            return "DEGRADED".to_string();
        }

        "HEALTHY".to_string()
    }
}
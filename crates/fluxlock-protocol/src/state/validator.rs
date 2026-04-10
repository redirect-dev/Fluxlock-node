#[derive(Clone, Debug)]
pub struct Validator {
    pub stake: u128,
    pub classical_pubkey: Vec<u8>,
    pub pq_pubkey: Vec<u8>,
    pub nonce: u64,
    pub reputation: i32, // 🔥 NEW
}

impl Validator {
    pub fn new(
        stake: u128,
        classical_pubkey: Vec<u8>,
        pq_pubkey: Vec<u8>,
        nonce: u64,
        reputation: i32,
    ) -> Self {
        Self {
            stake,
            classical_pubkey,
            pq_pubkey,
            nonce,
            reputation,
        }
    }

    pub fn slash(&mut self, amount: u128) {
        if self.stake >= amount {
            self.stake -= amount;
        } else {
            self.stake = 0;
        }

        // 🔥 Reputation penalty
        self.reputation -= amount as i32 / 2;
    }

    pub fn reward(&mut self) {
        // 🔥 Small positive reinforcement
        self.reputation += 2;

        if self.reputation > 100 {
            self.reputation = 100;
        }
    }
}
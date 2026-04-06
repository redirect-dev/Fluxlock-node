#[derive(Clone, Debug)]
pub enum ValidatorStatus {
    Active,
    Suspended,
    Slashed,
    Exited,
}

#[derive(Clone, Debug)]
pub struct Validator {
    pub stake: u128,

    pub classical_pubkey: Vec<u8>,
    pub pq_pubkey: Vec<u8>,

    pub last_rotation_tick: u64,
    pub rotation_deadline_tick: u64,

    pub status: ValidatorStatus,
}

impl Validator {
    pub fn new(
        stake: u128,
        classical_pubkey: Vec<u8>,
        pq_pubkey: Vec<u8>,
        current_tick: u64,
        max_lifetime: u64,
    ) -> Self {
        Self {
            stake,
            classical_pubkey,
            pq_pubkey,
            last_rotation_tick: current_tick,
            rotation_deadline_tick: current_tick + max_lifetime,
            status: ValidatorStatus::Active,
        }
    }

    /// 🔥 NEW: Slash validator for protocol violation
    pub fn slash(&mut self, amount: u128) {
        if self.stake <= amount {
            self.stake = 0;
            self.status = ValidatorStatus::Slashed;
        } else {
            self.stake -= amount;
        }

        // 🎬 CINEMATIC OUTPUT (THIS IS THE DEMO MOMENT)
        println!("\n🚨 PROTOCOL VIOLATION DETECTED");
        println!("⚔ VALIDATOR SLASHED");
        println!("🪓 New stake: {}\n", self.stake);
    }
}
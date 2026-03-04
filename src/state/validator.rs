// src/state/validator.rs

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

    // Hybrid identity
    pub classical_pubkey: Vec<u8>,
    pub pq_pubkey: Vec<u8>,

    // Rotation tracking
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
}

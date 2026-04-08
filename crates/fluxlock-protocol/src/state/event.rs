#[derive(Debug, Clone)]
pub enum Event {
    RotationSuccess {
        identity: Vec<u8>,
        epoch: u64,
    },

    InvalidNonce {
        identity: Vec<u8>,
    },

    ForkDetected {
        identity: Vec<u8>,
        epoch: u64,
    },

    IdentityExpired {
        identity: Vec<u8>,
    },

    InvalidContinuity {
        identity: Vec<u8>,
    },

    CommitmentMismatch {
        identity: Vec<u8>,
    },

    ValidatorSlashed {
        amount: u128,
    },
}
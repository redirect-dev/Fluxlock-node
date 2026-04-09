use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    RotationSuccess {
        identity: Vec<u8>,
        epoch: u64,
    },

    InvalidContinuity {
        identity: Vec<u8>,
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

    CommitmentMismatch {
        identity: Vec<u8>,
    },

    ValidatorSlashed {
        amount: u128,
    },
}
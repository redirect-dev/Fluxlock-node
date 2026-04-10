use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    RotationSuccess {
        identity: Vec<u8>,
        new_identity: Vec<u8>,
        epoch: u64,
        validator: String,
    },

    InvalidContinuity {
        identity: Vec<u8>,
        validator: String,
    },

    InvalidNonce {
        identity: Vec<u8>,
        validator: String,
    },

    ForkDetected {
        identity: Vec<u8>,
        epoch: u64,
        validator: String,
    },

    IdentityExpired {
        identity: Vec<u8>,
        validator: String,
    },

    CommitmentMismatch {
        identity: Vec<u8>,
        validator: String,
    },

    ValidatorSlashed {
        amount: u128,
        validator: String,
    },
}
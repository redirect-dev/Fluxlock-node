use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    // -----------------------------
    // ✅ SUCCESSFUL ROTATION
    // -----------------------------
    RotationSuccess {
        identity: Vec<u8>,        // previous identity
        new_identity: Vec<u8>,    // new identity after rotation
        epoch: u64,
    },

    // -----------------------------
    // ❌ CONTINUITY FAILURE
    // -----------------------------
    InvalidContinuity {
        identity: Vec<u8>,
    },

    // -----------------------------
    // ❌ NONCE FAILURE
    // -----------------------------
    InvalidNonce {
        identity: Vec<u8>,
    },

    // -----------------------------
    // ❌ FORK DETECTED
    // -----------------------------
    ForkDetected {
        identity: Vec<u8>,
        epoch: u64,
    },

    // -----------------------------
    // ❌ IDENTITY EXPIRED
    // -----------------------------
    IdentityExpired {
        identity: Vec<u8>,
    },

    // -----------------------------
    // ❌ COMMITMENT FAILURE
    // -----------------------------
    CommitmentMismatch {
        identity: Vec<u8>,
    },

    // -----------------------------
    // ⚔ VALIDATOR PENALTY
    // -----------------------------
    ValidatorSlashed {
        amount: u128,
    },
}
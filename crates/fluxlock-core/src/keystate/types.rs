use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlgorithmId {
    Ed25519,
    Secp256k1,
    MlDsa,
    Hybrid,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RotationPolicy {
    /// Number of ticks before rotation is allowed.
    pub epoch_length: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyState {
    /// Monotonically increasing epoch counter.
    pub key_epoch: u64,

    /// Tick at which this key became active.
    pub activated_at_tick: u64,

    /// Cryptographic algorithm used for verification.
    pub algorithm: AlgorithmId,

    /// Active public key used for signature verification.
    pub current_pubkey: Vec<u8>,

    /// Hash commitment to next public key (optional until set).
    pub next_pubkey_commitment: Option<Vec<u8>>,

    /// Deterministic rotation rules.
    pub rotation_policy: RotationPolicy,

    /// Allows deterministic early rotation when true.
    pub rotation_override: bool,

    /// Optional validity bounds.
    pub not_before_tick: u64,
    pub not_after_tick: Option<u64>,

    /// Hash of previous KeyState for lineage continuity.
    pub parent_key_hash: Option<Vec<u8>>,
}

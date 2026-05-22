use serde::{
    Serialize,
    Deserialize,
};

// =========================
// 📡 CONTINUITY PROOF
// =========================
#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
)]
pub struct ContinuityBroadcast {

    pub validator_id: u32,

    pub epoch: u64,

    pub continuity_hash: String,

    pub trust: f64,

    pub drift: f64,

    pub stability: f64,

    pub governance_weight: f64,

    pub continuity_score: f64,

    pub fracture_severity: f64,

    pub quarantine_level: f64,

    pub topology_cluster: u32,

    pub resonance_score: f64,

    pub healing_wave: f64,

    pub accepted: bool,
}
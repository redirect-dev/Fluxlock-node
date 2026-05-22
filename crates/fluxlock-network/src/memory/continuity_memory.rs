use serde::{
    Serialize,
    Deserialize,
};

use std::collections::HashMap;

// =========================
// 🧠 CONTINUITY MEMORY
// =========================
#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
)]
pub struct ContinuityMemory {

    pub validator_id: u64,

    // =========================
    // 🕰 HISTORY
    // =========================
    pub stable_epochs: u64,

    pub fracture_events: u64,

    pub recovery_events: u64,

    pub successful_rehabilitations: u64,

    // =========================
    // 🧬 LINEAGE MEMORY
    // =========================
    pub lineage_depth: u64,

    pub inherited_trust: f64,

    pub historical_stability: f64,

    // =========================
    // 🌐 NETWORK MEMORY
    // =========================
    pub peer_observations: u64,

    pub network_rejections: u64,

    pub network_acceptances: u64,

    // =========================
    // 🛡 IMMUNE HISTORY
    // =========================
    pub immune_resistance: f64,

    pub adaptive_hardening: f64,

    pub fracture_resistance: f64,

    // =========================
    // 🧠 CONTINUITY SCORE
    // =========================
    pub continuity_memory_score: f64,
}

// =========================
// 🌐 MEMORY STORE
// =========================
pub type MemoryStore =
    HashMap<u64, ContinuityMemory>;

// =========================
// 🧠 CREATE MEMORY
// =========================
pub fn create_memory(
    validator_id: u64,
) -> ContinuityMemory {

    ContinuityMemory {

        validator_id,

        stable_epochs: 0,

        fracture_events: 0,

        recovery_events: 0,

        successful_rehabilitations: 0,

        lineage_depth: 0,

        inherited_trust: 0.0,

        historical_stability: 1.0,

        peer_observations: 0,

        network_rejections: 0,

        network_acceptances: 0,

        immune_resistance: 1.0,

        adaptive_hardening: 1.0,

        fracture_resistance: 1.0,

        continuity_memory_score: 100.0,
    }
}

// =========================
// 🧠 UPDATE MEMORY
// =========================
pub fn update_memory_score(
    memory: &mut ContinuityMemory,
) {

    let stability_bonus =
        memory.stable_epochs as f64 * 0.002;

    let recovery_bonus =
        memory.successful_rehabilitations as f64
        * 0.75;

    let fracture_penalty =
        memory.fracture_events as f64 * 1.5;

    let rejection_penalty =
        memory.network_rejections as f64
        * 0.35;

    let adaptive_bonus =
        memory.adaptive_hardening
        * 2.0;

    memory.continuity_memory_score =
        100.0
        + stability_bonus
        + recovery_bonus
        + adaptive_bonus
        - fracture_penalty
        - rejection_penalty;

    if memory.continuity_memory_score < 0.0 {

        memory.continuity_memory_score = 0.0;
    }
}
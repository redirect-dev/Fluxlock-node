use serde::{
    Serialize,
    Deserialize,
};

use std::collections::HashMap;

// =========================
// 🧠 REMOTE CONTINUITY MEMORY
// =========================
#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
)]
pub struct RemoteContinuityMemory {

    pub validator_id: u32,

    pub last_seen_epoch: u64,

    pub remote_trust: f64,

    pub remote_drift: f64,

    pub remote_reputation: f64,

    pub remote_stability: f64,

    pub remote_anchor_strength: f64,

    pub remote_fracture_severity: f64,

    pub remote_quarantine_level: f64,

    pub remote_recovery_consistency: f64,

    pub remote_governance_weight: f64,

    pub remote_resonance: f64,

    pub remote_entropy: f64,

    pub remote_healing_wave: f64,

    pub remote_topology_cluster: u32,

    pub remote_peer_agreement: f64,

    pub remote_memory_score: f64,

    pub remote_survival_score: f64,

    pub remote_epoch_rotations: u64,

    pub remote_rebirth_count: u64,

    pub remote_status: String,
}

// =========================
// 🌐 DISTRIBUTED MEMORY MAP
// =========================
#[derive(
    Clone,
    Debug,
    Default,
    Serialize,
    Deserialize,
)]
pub struct DistributedMemory {

    pub memories:
        HashMap<
            u32,
            RemoteContinuityMemory
        >,
}

impl DistributedMemory {

    // =========================
    // 🧠 CREATE
    // =========================
    pub fn new() -> Self {

        Self {

            memories:
                HashMap::new(),
        }
    }

    // =========================
    // 💾 STORE MEMORY
    // =========================
    pub fn store_memory(
        &mut self,
        memory:
            RemoteContinuityMemory,
    ) {

        self.memories.insert(
            memory.validator_id,
            memory,
        );
    }

    // =========================
    // 📦 GET MEMORY
    // =========================
    pub fn get_memory(
        &self,
        validator_id: u32,
    ) -> Option<
        &RemoteContinuityMemory
    > {

        self.memories.get(
            &validator_id
        )
    }

    // =========================
    // 🌐 MEMORY COUNT
    // =========================
    pub fn memory_count(
        &self
    ) -> usize {

        self.memories.len()
    }
}
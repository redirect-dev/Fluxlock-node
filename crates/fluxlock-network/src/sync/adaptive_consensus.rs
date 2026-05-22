use fluxlock_core::types::{
    Validator,
};

use crate::memory::continuity_memory::{
    ContinuityMemory,
};

// =========================
// 🌐 ADAPTIVE CONSENSUS
// =========================
pub fn evolve_consensus(

    validator: &mut Validator,

    memory: &ContinuityMemory,
) {

    // =========================
    // 🧠 MEMORY WEIGHTING
    // =========================
    let memory_factor =
        memory.continuity_memory_score
        * 0.001;

    validator.governance_weight +=
        memory_factor * 0.05;

    validator.peer_agreement_ratio +=
        memory_factor * 0.002;

    // =========================
    // 🌐 STABILITY REWARD
    // =========================
    validator.leadership_score +=
        memory.historical_stability
        * 0.01;

    validator.network_influence_score +=
        memory.stable_epochs as f64
        * 0.0001;

    // =========================
    // 🛡 RECOVERY BONUS
    // =========================
    validator.rehabilitation_score +=
        memory.successful_rehabilitations
            as f64
        * 0.01;

    // =========================
    // ☠ FRACTURE PENALTY
    // =========================
    validator.consensus_pressure +=
        memory.fracture_events as f64
        * 0.05;

    validator.instability_radius +=
        memory.network_rejections as f64
        * 0.01;

    // =========================
    // 🔒 LIMITS
    // =========================
    validator.peer_agreement_ratio =
        validator
            .peer_agreement_ratio
            .clamp(0.0, 1.0);

    validator.governance_weight =
        validator
            .governance_weight
            .clamp(0.1, 10.0);

    validator.rehabilitation_score =
        validator
            .rehabilitation_score
            .clamp(0.0, 1000.0);
}
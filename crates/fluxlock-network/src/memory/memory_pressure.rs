use fluxlock_core::types::{
    Validator,
};

// =========================
// 🌐 MEMORY PRESSURE
// =========================
pub fn apply_memory_pressure(
    validator: &mut Validator,
) {

    // =========================
    // 🧠 STABILIZATION FIELD
    // =========================
    validator.stabilization_power +=
        validator.continuity_anchor_strength
        * 0.002;

    // =========================
    // 🌊 HISTORICAL PRESSURE
    // =========================
    validator.consensus_pressure +=
        validator.lineage_stability
        * 0.001;

    // =========================
    // 🛡 IMMUNE HARDENING
    // =========================
    validator.immune_response +=
        validator.resilience_score
        * 0.0005;

    // =========================
    // 🧬 CONTINUITY DENSITY
    // =========================
    validator.trust +=
        validator.inherited_trust
        * 0.0008;

    // =========================
    // 🔒 LIMITS
    // =========================
    if validator.trust > 100.0 {

        validator.trust = 100.0;
    }

    if validator.immune_response > 100.0 {

        validator.immune_response = 100.0;
    }
}
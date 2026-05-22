use fluxlock_core::types::{
    Validator,
};

// =========================
// 🧬 INHERIT LINEAGE
// =========================
pub fn inherit_lineage(
    parent: &Validator,
    child: &mut Validator,
) {

    // =========================
    // 🌱 TRUST INHERITANCE
    // =========================
    child.inherited_trust =
        parent.trust * 0.35;

    // =========================
    // 🧠 STABILITY MEMORY
    // =========================
    child.lineage_stability =
        parent.lineage_stability
        * 0.85;

    // =========================
    // 🛡 RESILIENCE MEMORY
    // =========================
    child.resilience_score =
        parent.resilience_score
        * 0.70;

    // =========================
    // 🧬 ANCESTRAL MEMORY
    // =========================
    child.continuity_anchor_strength =
        parent.continuity_anchor_strength
        * 0.90;

    // =========================
    // ⚠ SCAR INHERITANCE
    // =========================
    child.scar_level +=
        parent.scar_level
        * 0.30;

    // =========================
    // ☣ FRACTURE MEMORY
    // =========================
    child.fracture_severity +=
        parent.fracture_severity
        * 0.20;

    // =========================
    // 🔄 EPOCH MEMORY
    // =========================
    child.epoch_rotations =
        parent.epoch_rotations + 1;

    child.rebirth_count =
        parent.rebirth_count;
}
use fluxlock_core::types::{
    Validator,
};

// =========================
// 🧠 HISTORICAL CONSENSUS
// =========================
pub fn historical_vote_weight(
    validator: &Validator,
) -> f64 {

    let trust_weight =
        validator.trust * 0.35;

    let lineage_weight =
        validator.lineage_stability
        * 0.25;

    let recovery_weight =
        validator.successful_recoveries as f64
        * 1.5;

    let scar_penalty =
        validator.scar_level * 4.0;

    let fracture_penalty =
        validator.fracture_severity
        * 8.0;

    let result =
        trust_weight
        + lineage_weight
        + recovery_weight
        - scar_penalty
        - fracture_penalty;

    if result < 0.0 {

        return 0.0;
    }

    result
}
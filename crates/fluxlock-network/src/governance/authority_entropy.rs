use fluxlock_core::types::Validator;

// =========================
// 🌑 AUTHORITY ENTROPY
// =========================
pub fn apply_authority_entropy(
    validator: &mut Validator,
) {

    let mut decay = 0.0;

    // =========================
    // 👑 HIGH RANK DECAY
    // =========================
    decay +=
        validator.authority_points
        * 0.00005;

    // =========================
    // 🏛 GOVERNANCE TERM
    // =========================
    if validator.governance_term == 0 {

        decay += 1.0;
    }

    // =========================
    // 📉 LOW PARTICIPATION
    // =========================
    if validator.governance_participation
        < 50.0
    {
        decay += 2.0;
    }

    // =========================
    // ⚠ LOW CONSENSUS
    // =========================
    if validator.peer_agreement_ratio
        < 0.75
    {
        decay += 3.0;
    }

    // =========================
    // APPLY
    // =========================
    validator.authority_points -= decay;

    validator.authority_points =
        validator
            .authority_points
            .max(0.0);
}
use fluxlock_core::types::{
    Validator,
    ContinuityState,
};

// =========================
// 🌐 PEER GOVERNANCE RESULT
// =========================
pub struct GovernanceResult {

    pub accepted: bool,

    pub weighted_score: f64,

    pub approvals: u32,

    pub rejections: u32,

    pub quorum_reached: bool,
}

// =========================
// 🌐 PEER GOVERNANCE
// =========================
pub fn evaluate_peer_governance(

    validator: &mut Validator,

    snapshot: &[Validator],
) -> GovernanceResult {

    let mut approvals = 0u32;

    let mut rejections = 0u32;

    let mut weighted_score = 0.0;

    let mut total_weight = 0.0;

    // =========================
    // 🌐 PEER REVIEW
    // =========================
    for peer in snapshot.iter() {

        // =========================
        // 🚫 SKIP SELF
        // =========================
        if peer.id == validator.id {
            continue;
        }

        // =========================
        // 🚫 EXILED PEERS
        // =========================
        if peer.continuity_state
            == ContinuityState::Exiled
        {
            continue;
        }

        // =========================
        // 🌐 PEER WEIGHT
        // =========================
        let peer_weight =
            peer.governance_weight
            *
            (
                peer.trust / 100.0
            );

        total_weight += peer_weight;

        // =========================
        // 🔴 HARD REJECTION
        // =========================
        if validator.continuity_state
            == ContinuityState::Fractured
        ||
        validator.continuity_state
            == ContinuityState::Quarantined
        {

            rejections += 1;

            weighted_score -= peer_weight;

            continue;
        }

        // =========================
        // 🟠 RECOVERING
        // =========================
        if validator.continuity_state
            == ContinuityState::Recovering
        ||
        validator.continuity_state
            == ContinuityState::Rehabilitating
        {

            approvals += 1;

            weighted_score +=
                peer_weight * 0.5;

            continue;
        }

        // =========================
        // 🟢 HEALTHY
        // =========================
        if validator.continuity_state
            == ContinuityState::Healthy
        ||
        validator.continuity_state
            == ContinuityState::Evolving
        {

            approvals += 1;

            weighted_score +=
                peer_weight;

            continue;
        }
    }

    // =========================
    // 🌐 NORMALIZATION
    // =========================
    let normalized_score =
        if total_weight > 0.0 {

            (
                weighted_score
                / total_weight
            )
            .clamp(-1.0, 1.0)

        } else {

            0.0
        };

    // =========================
    // 🌐 QUORUM
    // =========================
    let quorum_reached =
        approvals + rejections >= 3;

    // =========================
    // 🌐 ACCEPTANCE
    // =========================
    let accepted =
        quorum_reached
        &&
        normalized_score > 0.0;

    // =========================
    // 🌐 STATE EFFECTS
    // =========================
    if !accepted {

        validator.consensus_pressure += 5.0;

        validator.continuity_suspicion += 2.0;
    }

    GovernanceResult {

        accepted,

        weighted_score:
            normalized_score,

        approvals,

        rejections,

        quorum_reached,
    }
}
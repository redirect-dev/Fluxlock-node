use fluxlock_core::types::{
    Validator,
    PeerAnnouncement,
};

// =========================
// 🌐 CONSENSUS RESULT
// =========================
#[derive(Clone)]
pub struct ConsensusResult {

    pub accepted: bool,

    pub confidence_delta: f64,

    pub trust_delta: f64,

    pub pressure_delta: f64,

    pub valid_votes: u32,

    pub invalid_votes: u32,

    pub quarantine_delta: f64,

    pub governance_delta: f64,

    pub rehabilitation_delta: f64,

    pub scar_delta: f64,

    pub exile: bool,

    // 🌐 NEW
    pub weighted_confidence: f64,

    pub network_alignment: f64,
}

// =========================
// 🌐 EVALUATE CONSENSUS
// =========================
pub fn evaluate_consensus(

    validator: &Validator,

    announcements: &Vec<PeerAnnouncement>,

) -> ConsensusResult {

    let mut valid_votes = 0;

    let mut invalid_votes = 0;
    
    // =========================
    // 🌐 WEIGHTED CONSENSUS
    // =========================
    let mut weighted_valid = 0.0;

    let mut weighted_invalid = 0.0;

    let mut total_weight = 0.0;
    
    fn authority_score(
    validator: &Validator,
) -> f64 {

    (
        validator.trust * 0.20
        +
        validator.continuity_reputation * 0.25
        +
        validator.adaptive_reputation * 0.20
        +
        validator.continuity_memory_score * 0.15
        +
        validator.evolutionary_authenticity * 0.10
        +
        (
            validator.historical_consensus_accuracy
            * 100.0
        ) * 0.10
    )
    .clamp(0.0, 100.0)
}

    for announcement in announcements {

        if announcement.validator_id
            != validator.id
        {
            continue;
        }

        // =========================
        // 🧠 DYNAMIC WEIGHT
        // =========================
        let authority =
             authority_score(
                 validator
    );

        let mut weight =
            authority / 100.0;

        // =========================
        // 🔗 CONTINUITY BONUS
        // =========================
        if announcement.continuity_hash.len() > 12 {

            weight += 0.15;
        }

        // =========================
        // 🌐 GOVERNANCE BONUS
        // =========================
        weight +=
            validator.governance_weight
            * 0.05;

        // =========================
        // 🧠 REPUTATION BONUS
        // =========================
        weight +=
            validator.peer_reputation
            * 0.002;

        weight =
            weight.clamp(0.05, 5.0);

        total_weight += weight;

        // =========================
        // 🟢 VALID
        // =========================
        if announcement.trust >= 60.0 {

            valid_votes += 1;

            weighted_valid += weight;

        } else {

            invalid_votes += 1;

            weighted_invalid += weight;
        }
    }

    let total_votes =
        valid_votes + invalid_votes;

    // =========================
    // 🌑 NO CONSENSUS
    // =========================
    if total_votes == 0 {

        return ConsensusResult {

            accepted:
                validator.network_accepted,

            confidence_delta: -0.001,

            trust_delta: -0.02,

            pressure_delta: 0.5,

            valid_votes,

            invalid_votes,

            quarantine_delta: 0.01,

            governance_delta: -0.01,

            rehabilitation_delta: -0.10,

            scar_delta: 0.05,

            exile: false,

            weighted_confidence: 0.0,

            network_alignment: 0.0,
        };
    }

    // =========================
    // 🌐 WEIGHTED RATIO
    // =========================
    let weighted_ratio =
        weighted_valid
        / (weighted_valid + weighted_invalid)
            .max(0.0001);

    // =========================
    // 🌐 ALIGNMENT SCORE
    // =========================
    let network_alignment =
        (
            validator.peer_agreement_ratio
            + weighted_ratio
        ) / 2.0;

    // =========================
    // 🟢 STRONG CONSENSUS
    // =========================
    if weighted_ratio >= 0.85 {

        return ConsensusResult {

            accepted: true,

            confidence_delta:
                0.015 * network_alignment,

            trust_delta:
                0.40 * network_alignment,

            pressure_delta: -1.5,

            valid_votes,

            invalid_votes,

            quarantine_delta: -0.15,

            governance_delta: 0.08,

            rehabilitation_delta: 2.0,

            scar_delta: -0.15,

            exile: false,

            weighted_confidence:
                weighted_ratio,

            network_alignment,
        };
    }

    // =========================
    // 🟢 HEALTHY CONSENSUS
    // =========================
    if weighted_ratio >= 0.66 {

        return ConsensusResult {

            accepted: true,

            confidence_delta:
                0.006 * network_alignment,

            trust_delta:
                0.15 * network_alignment,

            pressure_delta: -0.5,

            valid_votes,

            invalid_votes,

            quarantine_delta: -0.05,

            governance_delta: 0.03,

            rehabilitation_delta: 0.8,

            scar_delta: -0.05,

            exile: false,

            weighted_confidence:
                weighted_ratio,

            network_alignment,
        };
    }

    // =========================
    // 🟠 DEGRADED CONSENSUS
    // =========================
    if weighted_ratio >= 0.40 {

        return ConsensusResult {

            accepted: false,

            confidence_delta: -0.015,

            trust_delta: -0.35,

            pressure_delta: 1.5,

            valid_votes,

            invalid_votes,

            quarantine_delta: 0.08,

            governance_delta: -0.04,

            rehabilitation_delta: -0.6,

            scar_delta: 0.20,

            exile: false,

            weighted_confidence:
                weighted_ratio,

            network_alignment,
        };
    }

    // =========================
    // 🔴 FRACTURED CONSENSUS
    // =========================
    let exile =
        validator.fracture_severity > 150.0
        || validator.consensus_failures > 8
        || validator.quarantine_level > 120.0;

    ConsensusResult {

        accepted: false,

        confidence_delta: -0.030,

        trust_delta: -1.0,

        pressure_delta: 5.0,

        valid_votes,

        invalid_votes,

        quarantine_delta: 0.30,

        governance_delta: -0.10,

        rehabilitation_delta: -3.0,

        scar_delta: 0.75,

        exile,

        weighted_confidence:
            weighted_ratio,

        network_alignment,
    }
}
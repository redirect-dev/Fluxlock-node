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

    for announcement in announcements {

        if announcement.validator_id
            != validator.id
        {
            continue;
        }

        if announcement.trust >= 60.0 {

            valid_votes += 1;

        } else {

            invalid_votes += 1;
        }
    }

    let total_votes =
        valid_votes + invalid_votes;

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
        };
    }

    let ratio =
        valid_votes as f64
        / total_votes as f64;

    // =========================
    // 🟢 HEALTHY CONSENSUS
    // =========================
    if ratio >= 0.66 {

        return ConsensusResult {

            accepted: true,

            confidence_delta: 0.004,

            trust_delta: 0.10,

            pressure_delta: -0.5,

            valid_votes,

            invalid_votes,

            quarantine_delta: -0.03,

            governance_delta: 0.02,
        };
    }

    // =========================
    // 🔴 QUARANTINE PRESSURE
    // =========================
    ConsensusResult {

        accepted: false,

        confidence_delta: -0.015,

        trust_delta: -0.40,

        pressure_delta: 2.0,

        valid_votes,

        invalid_votes,

        quarantine_delta: 0.08,

        governance_delta: -0.03,
    }
}
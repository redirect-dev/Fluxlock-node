use serde::{
    Serialize,
    Deserialize,
};

use std::collections::HashMap;

// =========================
// 🔗 IDENTITY LINK
// =========================
#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
)]
pub struct IdentityLink {

    pub public_key:
        Vec<u8>,

    pub signature:
        Option<Vec<u8>>,

    pub continuity_hash:
        String,

    pub parent_hash:
        String,

    pub epoch:
        u64,

    pub validator_id:
        u32,

    pub governance_weight:
        f64,

    pub entropy_score:
        f64,
}

// =========================
// 🌐 VALIDATOR
// =========================
#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
)]
pub struct Validator {

    pub id: u32,

    pub confidence: f64,

    pub trust: f64,

    pub drift: f64,

    pub epoch_age: u64,

    pub chain_valid: bool,

    pub network_accepted: bool,

    pub recovery_timer: u64,

    pub rehabilitation_score: f64,

    pub rehabilitation_epochs: u64,

    pub peer_votes_valid: u32,

    pub peer_votes_invalid: u32,

    pub local_valid: bool,

    pub global_valid: bool,

    pub identity_chain:
        Vec<IdentityLink>,

    pub attack_history: u64,

    pub successful_recoveries: u64,

    pub resilience_score: f64,

    pub scar_level: f64,

    pub immune_response: f64,

    pub consensus_pressure: f64,

    pub instability_radius: f64,

    pub stabilization_power: f64,

    pub rehabilitation_votes: u32,

    pub fracture_severity: f64,

    pub continuity_anchor_strength: f64,

    pub current_epoch: u64,

    pub inherited_trust: f64,

    pub lineage_stability: f64,

    pub epoch_rotations: u64,

    pub rebirth_count: u64,

    pub last_epoch_transition: u64,

    pub quorum_score: f64,

    pub peer_agreement_ratio: f64,

    pub malicious_reports: u64,

    pub consensus_failures: u64,

    pub last_quorum_epoch: u64,

    pub governance_weight: f64,

    pub quarantine_level: f64,

    pub peer_reputation: f64,

    pub leadership_score: f64,

    pub recovery_votes_received: u32,

    pub recovery_votes_given: u32,

    pub governance_participation: f64,

    pub autonomous_trust_bias: f64,

    pub validator_stability_index: f64,

    pub network_influence_score: f64,

    pub isolation_events: u64,

    pub status: String,
}

// =========================
// 🧠 IDENTITY PROOF
// =========================
#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
)]
pub struct IdentityProof {

    pub proof_hash:
        String,

    pub validator_id:
        u32,

    pub epoch:
        u64,
}

// =========================
// 🌐 FLUX IDENTITY
// =========================
#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
)]
pub struct FluxIdentity {

    pub identity_id:
        String,

    pub created_epoch:
        u64,

    pub last_active_epoch:
        u64,

    pub session_count:
        u64,

    pub trust_score:
        f64,

    pub continuity_score:
        f64,

    pub bound_validator:
        u32,

    pub successful_auths:
        u64,

    pub failed_auths:
        u64,

    pub recovery_events:
        u64,

    pub drift_score:
        f64,

    pub status:
        String,

    pub credential_depth:
        u64,

    pub proofs:
        Vec<IdentityProof>,
}

// =========================
// 🌐 IDENTITY REGISTRY
// =========================
#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
)]
pub struct IdentityRegistry {

    pub identities:
        HashMap<
            String,
            FluxIdentity
        >,
}

impl IdentityRegistry {

    pub fn new() -> Self {

        Self {

            identities:
                HashMap::new(),
        }
    }

    // =========================
    // 🧠 CREATE IDENTITY
    // =========================
    pub fn create_identity(
        &mut self,
        identity: FluxIdentity,
    ) {

        self.identities.insert(
            identity.identity_id.clone(),
            identity,
        );
    }
}

// =========================
// 🌐 PEER NODE
// =========================
#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
)]
pub struct PeerNode {

    pub peer_id:
        String,

    pub address:
        String,

    pub validator_id:
        u32,

    pub last_seen_epoch:
        u64,

    pub trust_score:
        f64,

    pub active:
        bool,
}

// =========================
// 📡 PEER ANNOUNCEMENT
// =========================
#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
)]
pub struct PeerAnnouncement {

    pub peer_id:
        String,

    pub validator_id:
        u32,

    pub epoch:
        u64,

    pub trust:
        f64,

    pub continuity_hash:
        String,
}

// =========================
// 🌐 GOSSIP STATE
// =========================
#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
)]
pub struct GossipState {

    pub announcements:
        Vec<PeerAnnouncement>,
}
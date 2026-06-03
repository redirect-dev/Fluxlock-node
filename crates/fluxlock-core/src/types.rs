use serde::{
    Serialize,
    Deserialize,
};

use std::collections::HashMap;

// =========================
// 🧬 CONTINUITY STATE
// =========================
#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
)]
pub enum ContinuityState {

    Healthy,

    Evolving,

    Recovering,

    Fractured,

    Quarantined,

    Rehabilitating,

    Exiled,
}

// =========================
// 🧠 CONTINUITY EVENT TYPE
// =========================
#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
)]
pub enum ContinuityEventType {

    EpochRotation,

    SpikeAttack,

    CriticalBreach,

    ContinuityFracture,

    GovernanceRecovery,

    ConsensusFailure,

    NetworkReacceptance,

    IdentitySuccess,

    IdentityFailure,

    Quarantine,

    Rehabilitation,
}

// =========================
// 🧠 CONTINUITY EVENT
// =========================
#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
)]
pub struct ContinuityEvent {

    pub validator_id: u32,

    pub epoch: u64,

    pub event_type:
        ContinuityEventType,

    pub severity: f64,

    pub trust_delta: f64,

    pub continuity_delta: f64,

    pub recovery_delta: f64,

    pub description: String,
}

// =========================
// 🔑 IDENTITY LINK
// =========================
#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
)]
pub struct IdentityLink {

    pub public_key: Vec<u8>,

    pub signature:
        Option<Vec<u8>>,

    pub continuity_hash:
        String,

    pub parent_hash:
        String,

    pub state_hash:
        String,

    pub lineage_signature:
        Option<Vec<u8>>,

    pub transition_signature:
        Option<Vec<u8>>,

    pub epoch: u64,

    pub continuity_epoch: u64,

    pub validator_id: u32,

    pub governance_weight: f64,

    pub governance_score: f64,

    pub governance_votes: u32,

    pub network_alignment: f64,

    pub continuity_confidence: f64,

    pub peer_agreement_ratio: f64,

    pub entropy_score: f64,

    pub lineage_stability: f64,

    pub fracture_severity: f64,

    pub rehabilitation_factor: f64,

    pub quarantine_level: f64,

    pub malicious_reports: u32,

    pub fork_conflicts: u32,

    pub continuity_verified: bool,
}

// =========================
// 🧬 CONTINUITY TRANSITION PROOF
// =========================
#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
)]
pub struct ContinuityTransitionProof {

    pub validator_id: u32,

    pub previous_hash: String,

    pub new_hash: String,

    pub epoch: u64,

    pub mutation_coherence: f64,

    pub continuity_drift: f64,

    pub lineage_authenticity: f64,

    pub continuity_confidence: f64,

    pub governance_weight: f64,

    pub entropy_score: f64,

    pub transition_signature:
        Option<String>,

    pub lineage_signature:
        Option<String>,

    pub proof_hash:
        String,
    
    pub governance_votes: u32,

    pub governance_approvals: u32,

    pub governance_rejections: u32,

    pub network_alignment: f64,

    pub mutation_shock: f64,

    pub evolutionary_authenticity: f64,

    pub continuity_attestation: String,

    pub continuity_verified: bool,
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

    // =========================
    // 🧠 CONTINUITY MEMORY
    // =========================
    pub continuity_events:
        Vec<ContinuityEvent>,

    pub continuity_memory_score: f64,

    pub historical_consensus_accuracy: f64,

    pub recovery_consistency: f64,

    pub adaptive_reputation: f64,

    pub continuity_reputation: f64,

    pub continuity_survival_score: f64,

    pub fracture_history: u64,

    pub recovery_history: u64,

    pub governance_history: u64,

    pub continuity_age: u64,

    // =========================
    // 🌐 RESILIENCE
    // =========================
    pub attack_history: u64,

    pub successful_recoveries: u64,

    pub resilience_score: f64,

    pub scar_level: f64,

    pub scar_severity: f64,

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

    pub malicious_reports: u32,

    pub consensus_failures: u32,

    pub last_quorum_epoch: u64,

    pub governance_weight: f64,

    pub governance_participation: f64,

    pub autonomous_trust_bias: f64,

    pub quarantine_level: f64,

    pub peer_reputation: f64,

    pub leadership_score: f64,

    pub recovery_votes_received: u32,

    pub recovery_votes_given: u32,

    pub network_influence_score: f64,

    pub isolation_events: u64,

    pub validator_stability_index: f64,

    // =========================
    // 🌐 ECOLOGY
    // =========================
    pub influence_radius: f64,

    pub entropy_output: f64,

    pub immune_strength: f64,

    pub healing_wave: f64,

    pub topology_cluster: u32,

    pub resonance_score: f64,

    pub regional_pressure: f64,

    pub trust_gravity: f64,

        // =========================
    // 🧬 CONTINUITY GOVERNANCE
    // =========================
    pub mutation_shock: f64,

    pub continuity_suspicion: f64,

    pub scrutiny_level: f64,

    pub evolutionary_authenticity: f64,

    pub mutation_pressure: f64,

    pub continuity_state:
    ContinuityState,
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

    pub peer_id: String,

    pub address: String,

    pub validator_id: u32,

    pub last_seen_epoch: u64,

    pub trust_score: f64,

    pub active: bool,
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

    pub peer_id: String,

    pub validator_id: u32,

    pub trust: f64,

    pub confidence: f64,

    pub epoch: u64,

    pub continuity_hash: String,

    pub accepted: bool,
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

// =========================
// 🔐 FLUX IDENTITY
// =========================
#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
)]
pub struct FluxIdentity {

    pub identity_id: String,

    pub created_epoch: u64,

    pub last_active_epoch: u64,

    pub session_count: u64,

    pub trust_score: f64,

    pub continuity_score: f64,

    pub bound_validator: u32,

    pub successful_auths: u64,

    pub failed_auths: u64,

    pub recovery_events: u64,

    pub drift_score: f64,

    pub status: String,

    pub credential_depth: u64,

    pub proofs: Vec<String>,
}

// =========================
// 🗂 IDENTITY REGISTRY
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
    // 🔐 CREATE IDENTITY
    // =========================
    pub fn create_identity(
        &mut self,
        identity_id: String,
        validator_id: u32,
        epoch: u64,
    ) {

        self.identities.insert(

            identity_id.clone(),

            FluxIdentity {

                identity_id,

                created_epoch: epoch,

                last_active_epoch: epoch,

                session_count: 0,

                trust_score: 100.0,

                continuity_score: 100.0,

                bound_validator:
                    validator_id,

                successful_auths: 0,

                failed_auths: 0,

                recovery_events: 0,

                drift_score: 0.0,

                status:
                    "healthy".into(),

                credential_depth: 1,

                proofs: Vec::new(),
            }
        );
    }
}
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

    pub public_key: Vec<u8>,

    pub signature: Option<Vec<u8>>,
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

    pub epoch: u64,

    pub validator_id: u32,

    pub trust: f64,

    pub continuity: f64,

    pub previous_hash: String,

    pub proof_hash: String,
}

// =========================
// 🌊 FLUX IDENTITY
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

    pub proofs: Vec<IdentityProof>,
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

    pub identity_chain: Vec<IdentityLink>,

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

    pub status: String,
}

// =========================
// 🧠 IDENTITY REGISTRY
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

// =========================
// 🧠 REGISTRY IMPL
// =========================
impl IdentityRegistry {

    pub fn new() -> Self {

        Self {

            identities:
                HashMap::new(),
        }
    }

    // =========================
    // 🔐 HASH GENERATOR
    // =========================
    fn build_hash(
        epoch: u64,
        validator_id: u32,
        trust: f64,
        continuity: f64,
        previous_hash: &str,
    ) -> String {

        format!(
            "{:x}",
            md5::compute(
                format!(
                    "{}:{}:{}:{}:{}",
                    epoch,
                    validator_id,
                    trust,
                    continuity,
                    previous_hash
                )
            )
        )
    }

    // =========================
    // 🆕 CREATE IDENTITY
    // =========================
    pub fn create_identity(
        &mut self,
        identity_id: String,
        validator_id: u32,
        current_epoch: u64,
    ) -> FluxIdentity {

        let genesis_hash =
            Self::build_hash(
                current_epoch,
                validator_id,
                50.0,
                50.0,
                "GENESIS",
            );

        let genesis_proof =
            IdentityProof {

                epoch:
                    current_epoch,

                validator_id,

                trust: 50.0,

                continuity: 50.0,

                previous_hash:
                    "GENESIS".into(),

                proof_hash:
                    genesis_hash,
            };

        let identity = FluxIdentity {

            identity_id:
                identity_id.clone(),

            created_epoch:
                current_epoch,

            last_active_epoch:
                current_epoch,

            session_count: 0,

            trust_score: 50.0,

            continuity_score: 50.0,

            bound_validator:
                validator_id,

            successful_auths: 0,

            failed_auths: 0,

            recovery_events: 0,

            drift_score: 0.0,

            status:
                "genesis".into(),

            credential_depth: 1,

            proofs:
                vec![genesis_proof],
        };

        self.identities.insert(
            identity_id,
            identity.clone(),
        );

        identity
    }

    // =========================
    // 🔍 GET OR CREATE
    // =========================
    pub fn get_or_create(
        &mut self,
        identity_id: String,
        validator_id: u32,
        current_epoch: u64,
    ) -> FluxIdentity {

        if let Some(identity) =
            self.identities.get(&identity_id)
        {

            return identity.clone();
        }

        self.create_identity(
            identity_id,
            validator_id,
            current_epoch,
        )
    }

    // =========================
    // 🔗 APPEND LINEAGE
    // =========================
    fn append_lineage(
        identity: &mut FluxIdentity,
        epoch: u64,
    ) {

        let previous_hash =
            identity
                .proofs
                .last()
                .map(|p|
                    p.proof_hash.clone()
                )
                .unwrap_or(
                    "GENESIS".into()
                );

        let proof_hash =
            Self::build_hash(
                epoch,
                identity.bound_validator,
                identity.trust_score,
                identity.continuity_score,
                &previous_hash,
            );

        identity.proofs.push(

            IdentityProof {

                epoch,

                validator_id:
                    identity.bound_validator,

                trust:
                    identity.trust_score,

                continuity:
                    identity.continuity_score,

                previous_hash,

                proof_hash,
            }
        );
    }

    // =========================
    // ✅ SUCCESS
    // =========================
    pub fn successful_auth(
        &mut self,
        identity_id: &str,
        current_epoch: u64,
        confidence: f64,
    ) {

        if let Some(identity) =
            self.identities
                .get_mut(identity_id)
        {

            identity.last_active_epoch =
                current_epoch;

            identity.session_count += 1;

            identity.successful_auths += 1;

            identity.trust_score +=
                1.5 * confidence;

            identity.continuity_score +=
                0.8 * confidence;

            identity.drift_score *= 0.92;

            identity.credential_depth += 1;

            identity.trust_score =
                identity
                    .trust_score
                    .clamp(0.0, 100.0);

            identity.continuity_score =
                identity
                    .continuity_score
                    .clamp(0.0, 100.0);

            Self::append_lineage(
                identity,
                current_epoch,
            );
        }
    }

    // =========================
    // ❌ FAILURE
    // =========================
    pub fn failed_auth(
        &mut self,
        identity_id: &str,
        current_epoch: u64,
    ) {

        if let Some(identity) =
            self.identities
                .get_mut(identity_id)
        {

            identity.last_active_epoch =
                current_epoch;

            identity.failed_auths += 1;

            identity.trust_score *= 0.90;

            identity.continuity_score *= 0.95;

            identity.drift_score += 5.0;

            identity.recovery_events += 1;

            identity.trust_score =
                identity
                    .trust_score
                    .clamp(0.0, 100.0);

            identity.continuity_score =
                identity
                    .continuity_score
                    .clamp(0.0, 100.0);

            Self::append_lineage(
                identity,
                current_epoch,
            );
        }
    }
}
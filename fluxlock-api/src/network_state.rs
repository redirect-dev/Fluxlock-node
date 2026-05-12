use serde::{
    Serialize,
    Deserialize,
};

use fluxlock_core::types::{
    Validator,
    IdentityLink,
    IdentityRegistry,
    PeerNode,
    PeerAnnouncement,
};

use crate::peer_state::PeerState;

use crate::engine::identity_validator::{
    generate_identity,
    rotate_identity,
    verify_lineage,
};

use crate::engine::consensus::{
    evaluate_consensus,
};

// 🔥 STORAGE
use fluxlock_storage::validator_store::{
    save_validator,
};

use fluxlock_storage::identity_store::{
    save_identity,
};

use fluxlock_storage::lineage_store::{
    save_identity_chain,
};

// =========================
// 🌐 NETWORK STATE
// =========================
#[derive(
    Clone,
    Serialize,
    Deserialize
)]
pub struct NetworkState {

    pub validators:
        Vec<Validator>,

    pub identities:
        IdentityRegistry,

    pub peer_state:
        PeerState,

    pub global_epoch: u64,
}

impl NetworkState {

    // =========================
    // 🌐 INIT
    // =========================
    pub fn new() -> Self {

        let mut validators =
            Vec::new();

        for i in 0..12 {

            let genesis_key =
                generate_identity(i);

            let genesis_link =
                IdentityLink {

                    public_key:
                        genesis_key,

                    signature:
                        None,
                };

            validators.push(

                Validator {

                    id: i,

                    confidence: 0.92,

                    trust: 96.0,

                    drift: 2.0,

                    epoch_age: 180,

                    chain_valid: true,

                    network_accepted: true,

                    recovery_timer: 0,

                    rehabilitation_score: 100.0,

                    rehabilitation_epochs: 0,

                    peer_votes_valid: 8,

                    peer_votes_invalid: 1,

                    local_valid: true,

                    global_valid: true,

                    identity_chain:
                        vec![genesis_link],

                    attack_history: 0,

                    successful_recoveries: 0,

                    resilience_score: 100.0,

                    scar_level: 0.0,

                    immune_response: 1.0,

                    consensus_pressure: 0.0,

                    instability_radius: 0.0,

                    stabilization_power: 1.0,

                    rehabilitation_votes: 0,

                    fracture_severity: 0.0,

                    continuity_anchor_strength: 100.0,

                    current_epoch: 0,

                    inherited_trust: 96.0,

                    lineage_stability: 100.0,

                    epoch_rotations: 0,

                    rebirth_count: 0,

                    last_epoch_transition: 0,

                    quorum_score: 100.0,

                    peer_agreement_ratio: 1.0,

                    malicious_reports: 0,

                    consensus_failures: 0,

                    last_quorum_epoch: 0,

                    status:
                        "healthy".into(),
                }
            );
        }

        Self {

            validators,

            identities:
                IdentityRegistry::new(),

            peer_state:
                PeerState::new(),

            global_epoch: 0,
        }
    }

    // =========================
    // 🔁 ENGINE LOOP
    // =========================
    pub fn tick(
        &mut self
    ) {

        self.global_epoch += 1;

        // =========================
        // 🌐 PEER HEALTH
        // =========================
        self.peer_state
            .detect_stale_peers(
                self.global_epoch
            );

        // =========================
        // 📡 GOSSIP EMISSION
        // =========================
        for validator in
            &self.validators
        {

            let continuity_hash =
                format!(
                    "{:x}",
                    md5::compute(
                        format!(
                            "{}:{}:{}",
                            validator.id,
                            validator.trust,
                            validator.identity_chain.len()
                        )
                    )
                );

            self.peer_state
                .push_announcement(

                    PeerAnnouncement {

                        peer_id:
                            self.peer_state
                                .local_peer_id
                                .clone(),

                        validator_id:
                            validator.id,

                        epoch:
                            self.global_epoch,

                        trust:
                            validator.trust,

                        continuity_hash,
                    }
                );
        }

        // =========================
        // 🔁 ROTATION CYCLE
        // =========================
        if self.global_epoch % 1200 == 0 {

            let validator_ids:
                Vec<u32> =
                self.validators
                    .iter()
                    .map(|v| v.id)
                    .collect();

            for id in validator_ids {

                self.perform_epoch_rotation(
                    id
                );
            }
        }

        // =========================
        // 🌐 VALIDATOR LOOP
        // =========================
        for validator in
            &mut self.validators
        {

            validator.epoch_age += 1;

            validator.drift *= 0.998;

            validator.drift =
                validator
                    .drift
                    .clamp(0.0, 100.0);

            validator.chain_valid =
                verify_lineage(
                    &validator.identity_chain,
                    validator.id,
                );

            // =========================
            // 🌐 DISTRIBUTED CONSENSUS
            // =========================
            let consensus =
                evaluate_consensus(
                    validator,
                    &self
                        .peer_state
                        .gossip
                        .announcements,
                );

            validator.network_accepted =
                consensus.accepted;

            validator.peer_votes_valid =
                consensus.valid_votes;

            validator.peer_votes_invalid =
                consensus.invalid_votes;

            validator.confidence +=
                consensus.confidence_delta;

            validator.trust +=
                consensus.trust_delta;

            validator.consensus_pressure +=
                consensus.pressure_delta;

            validator.peer_agreement_ratio =
                if (
                    consensus.valid_votes
                    + consensus.invalid_votes
                ) > 0 {

                    consensus.valid_votes
                        as f64
                    /
                    (
                        consensus.valid_votes
                        + consensus.invalid_votes
                    ) as f64

                } else {

                    0.0
                };

            validator.quorum_score =
                validator.peer_agreement_ratio
                    * validator.trust;

            if !consensus.accepted {

                validator.consensus_failures += 1;

                validator.status =
                    "network-rejected".into();

            } else {

                validator.last_quorum_epoch =
                    self.global_epoch;
            }

            // =========================
            // 🌐 HEALTH EVALUATION
            // =========================
            if validator.drift > 25.0 {

                validator.status =
                    "recovering".into();

                validator.confidence *= 0.993;

                validator.trust *= 0.997;

            } else {

                if validator.network_accepted {

                    validator.status =
                        "healthy".into();
                }

                validator.confidence +=
                    0.0004;

                validator.trust +=
                    0.01;
            }

            validator.confidence =
                validator
                    .confidence
                    .clamp(0.0, 1.0);

            validator.trust =
                validator
                    .trust
                    .clamp(0.0, 100.0);

            // 💾 PERSIST
            let _ =
                save_validator(
                    validator
                );

            let _ =
                save_identity_chain(
                    validator.id,
                    &validator.identity_chain
                );
        }

        // =========================
        // 🧠 IDENTITY LOOP
        // =========================
        for identity in
            self.identities
                .identities
                .values_mut()
        {

            identity.continuity_score +=
                0.01;

            let idle_age =
                self.global_epoch
                    .saturating_sub(
                        identity.last_active_epoch
                    );

            if idle_age > 600 {

                identity.drift_score +=
                    0.03;

                identity.trust_score *=
                    0.999;
            }

            if identity.status
                == "recovering"
                && identity.drift_score < 10.0
            {

                identity.status =
                    "maturing".into();
            }

            // 💾 PERSIST
            let _ =
                save_identity(
                    identity
                );
        }
    }

    // =========================
    // 🌐 REGISTER PEER
    // =========================
    pub fn register_peer(
        &mut self,
        peer_id: String,
        address: String,
        validator_id: u32,
    ) {

        self.peer_state
            .register_peer(

                PeerNode {

                    peer_id,

                    address,

                    validator_id,

                    last_seen_epoch:
                        self.global_epoch,

                    trust_score: 100.0,

                    active: true,
                }
            );
    }

    // =========================
    // 💓 HEARTBEAT
    // =========================
    pub fn peer_heartbeat(
        &mut self,
        peer_id: &str,
    ) {

        self.peer_state
            .heartbeat(
                peer_id,
                self.global_epoch,
            );
    }

    // =========================
    // 🔁 EPOCH ROTATION
    // =========================
    pub fn perform_epoch_rotation(
        &mut self,
        validator_id: u32,
    ) {

        self.evolve_identity(
            validator_id
        );
    }

    // =========================
    // 🔁 EVOLVE IDENTITY
    // =========================
    pub fn evolve_identity(
        &mut self,
        validator_id: u32,
    ) {

        let validator =
            match self.validators
                .iter_mut()
                .find(|v|
                    v.id == validator_id
                )
        {
            Some(v) => v,
            None => return,
        };

        let rotation_index =
            validator.identity_chain.len();

        let message =
            format!(
                "validator:{}:rotation:{}",
                validator_id,
                rotation_index,
            );

        let signature =
            rotate_identity(
                validator_id,
                message.as_bytes(),
            );

        let new_key =
            generate_identity(
                validator_id
            );

        validator.identity_chain.push(

            IdentityLink {

                public_key:
                    new_key,

                signature:
                    Some(signature),
            }
        );

        validator.chain_valid =
            verify_lineage(
                &validator.identity_chain,
                validator.id,
            );

        validator.epoch_rotations += 1;

        validator.last_epoch_transition =
            self.global_epoch;

        if validator.identity_chain.len() > 64 {

            validator.identity_chain.remove(0);
        }

        let _ =
            save_validator(
                validator
            );

        let _ =
            save_identity_chain(
                validator.id,
                &validator.identity_chain
            );
    }

    // =========================
    // ⚡ SPIKE
    // =========================
    pub fn spike_attack(
        &mut self,
        id: u32,
    ) {

        if let Some(v) =
            self.validators
                .iter_mut()
                .find(|v| v.id == id)
        {

            v.drift += 12.0;

            v.trust *= 0.94;

            v.attack_history += 1;

            v.consensus_pressure += 2.0;

            v.status =
                "recovering".into();
        }
    }

    // =========================
    // ☠ BREACH
    // =========================
    pub fn breach_attack(
        &mut self,
        id: u32,
    ) {

        if let Some(v) =
            self.validators
                .iter_mut()
                .find(|v| v.id == id)
        {

            v.drift += 35.0;

            v.trust *= 0.72;

            v.confidence *= 0.75;

            v.attack_history += 1;

            v.network_accepted = false;

            v.local_valid = false;

            v.global_valid = false;

            v.consensus_failures += 1;

            v.status =
                "quarantined".into();
        }
    }

    // =========================
    // 🌊 NETWORK ATTACK
    // =========================
    pub fn network_attack(
        &mut self
    ) {

        for v in
            &mut self.validators
        {

            v.drift += 6.0;

            v.trust *= 0.97;

            v.attack_history += 1;

            v.consensus_pressure += 0.8;
        }
    }

    // =========================
    // 🔁 ACCESS FEEDBACK
    // =========================
    pub fn apply_access_feedback(
        &mut self,
        validator_id: u32,
        allowed: bool,
        confidence: f64,
    ) {

        if let Some(v) =
            self.validators
                .iter_mut()
                .find(|v| v.id == validator_id)
        {

            if allowed {

                v.confidence +=
                    0.01 * confidence;

                v.trust +=
                    0.25 * confidence;

                v.drift *= 0.96;

            } else {

                v.confidence *= 0.98;

                v.trust *= 0.99;

                v.drift += 1.0;

                v.consensus_pressure += 0.5;
            }

            v.confidence =
                v.confidence
                    .clamp(0.0, 1.0);

            v.trust =
                v.trust
                    .clamp(0.0, 100.0);
        }
    }

    // =========================
    // 🧠 IDENTITY MEMORY
    // =========================
    pub fn get_or_create_identity(
        &mut self,
        identity_id: String,
        validator_id: u32,
    ) {

        self.identities
            .get_or_create(
                identity_id,
                validator_id,
                self.global_epoch,
            );
    }

    pub fn identity_success(
        &mut self,
        identity_id: &str,
        confidence: f64,
    ) {

        self.identities
            .successful_auth(
                identity_id,
                self.global_epoch,
                confidence,
            );
    }

    pub fn identity_failure(
        &mut self,
        identity_id: &str,
    ) {

        self.identities
            .failed_auth(
                identity_id,
                self.global_epoch,
            );
    }
}
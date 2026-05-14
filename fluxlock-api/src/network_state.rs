use serde::{
    Serialize,
    Deserialize,
};

use std::fs;

use fluxlock_core::types::{
    Validator,
    IdentityLink,
    IdentityRegistry,
    PeerNode,
    PeerAnnouncement,
    FluxIdentity,
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

use fluxlock_storage::validator_store::{
    save_validator,
};

use fluxlock_storage::identity_store::{
    save_identity,
};

use fluxlock_storage::lineage_store::{
    save_identity_chain,
    load_identity_chain,
};

// =========================
// 💾 GLOBAL EPOCH STORAGE
// =========================
const EPOCH_FILE: &str =
    "fluxlock_epoch.dat";

// =========================
// 📥 LOAD EPOCH
// =========================
fn load_epoch() -> u64 {

    match fs::read_to_string(
        EPOCH_FILE
    ) {

        Ok(contents) => {

            contents
                .trim()
                .parse::<u64>()
                .unwrap_or(1)
        }

        Err(_) => 1,
    }
}

// =========================
// 💾 SAVE EPOCH
// =========================
fn save_epoch(
    epoch: u64
) {

    let _ =
        fs::write(
            EPOCH_FILE,
            epoch.to_string()
        );
}

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

        let restored_epoch =
            load_epoch();

        println!(
            "♻ RESTORED GLOBAL EPOCH {}",
            restored_epoch
        );

        let mut validators =
            Vec::new();

        for i in 0..12 {

            // =========================
            // 🔄 LOAD STORED CHAIN
            // =========================
            let stored_chain =
                load_identity_chain(i)
                    .unwrap_or_default();

            let identity_chain =
                if !stored_chain.is_empty() {

                    println!(
                        "♻ RESTORED VALIDATOR {} CHAIN DEPTH {}",
                        i,
                        stored_chain.len()
                    );

                    stored_chain

                } else {

                    // =========================
                    // 🌱 GENESIS CREATION
                    // =========================
                    let genesis_key =
                        generate_identity(i);

                    let genesis_hash =
                        format!(
                            "{:x}",
                            md5::compute(
                                format!(
                                    "genesis:{}",
                                    i
                                )
                            )
                        );

                    let genesis_link =
                        IdentityLink {

                            public_key:
                                genesis_key,

                            signature:
                                None,

                            continuity_hash:
                                genesis_hash,

                            parent_hash:
                                "GENESIS".into(),

                            epoch: 0,

                            validator_id: i,

                            governance_weight: 1.0,

                            entropy_score: 100.0,
                        };

                    vec![genesis_link]
                };

            let chain_depth =
                identity_chain.len();

            validators.push(

                Validator {

                    id: i,

                    confidence: 0.92,

                    trust: 96.0,

                    drift: 2.0,

                    epoch_age:
                        restored_epoch,

                    chain_valid:
                        verify_lineage(
                            &identity_chain,
                            i,
                        ),

                    network_accepted: true,

                    recovery_timer: 0,

                    rehabilitation_score: 100.0,

                    rehabilitation_epochs: 0,

                    peer_votes_valid: 8,

                    peer_votes_invalid: 1,

                    local_valid: true,

                    global_valid: true,

                    identity_chain,

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

                    current_epoch:
                        restored_epoch,

                    inherited_trust: 96.0,

                    lineage_stability: 100.0,

                    epoch_rotations:
                        chain_depth as u64,

                    rebirth_count: 0,

                    last_epoch_transition:
                        restored_epoch,

                    quorum_score: 100.0,

                    peer_agreement_ratio: 1.0,

                    malicious_reports: 0,

                    consensus_failures: 0,

                    last_quorum_epoch:
                        restored_epoch,

                    governance_weight: 1.0,

                    governance_participation: 100.0,

                    autonomous_trust_bias: 1.0,

                    quarantine_level: 0.0,

                    peer_reputation: 100.0,

                    leadership_score: 100.0,

                    recovery_votes_received: 0,

                    recovery_votes_given: 0,

                    network_influence_score: 100.0,

                    isolation_events: 0,

                    validator_stability_index: 100.0,

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

            global_epoch:
                restored_epoch,
        }
    }

    // =========================
    // 🔁 ENGINE LOOP
    // =========================
    pub fn tick(
        &mut self
    ) {

        self.global_epoch += 1;

        save_epoch(
            self.global_epoch
        );

        self.peer_state
            .detect_stale_peers(
                self.global_epoch
            );

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
        // 🔁 ROTATION SCHEDULE
        // =========================
        if self.global_epoch > 1
            && self.global_epoch % 1200 == 0
        {

            println!(
                "🌐 GLOBAL ROTATION EPOCH {}",
                self.global_epoch
            );

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

        for identity in
            self.identities
                .identities
                .values_mut()
        {

            identity.continuity_score +=
                0.01;

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
    // 🔁 ROTATION
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
    // 🧬 EVOLVE
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

        let rotated_link =
            rotate_identity(
                validator_id,
                rotation_index,
            );

        let previous_hash =
            validator
                .identity_chain
                .last()
                .map(|l|
                    l.continuity_hash.clone()
                )
                .unwrap_or(
                    "GENESIS".into()
                );

        validator.identity_chain.push(

            IdentityLink {

                public_key:
                    rotated_link.public_key,

                signature:
                    rotated_link.signature,

                continuity_hash:
                    rotated_link.continuity_hash,

                parent_hash:
                    previous_hash,

                epoch:
                    self.global_epoch,

                validator_id,

                governance_weight: 1.0,

                entropy_score:
                    validator.trust,
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

        println!(
            "🧬 EVOLVED CHAIN => {}",
            validator.identity_chain.len()
        );

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
    // ⚡ SPIKE ATTACK
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

            v.drift += 15.0;

            v.trust *= 0.95;

            v.status =
                "recovering".into();
        }
    }

    // =========================
    // ☠ BREACH ATTACK
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

            v.drift += 40.0;

            v.trust *= 0.70;

            v.network_accepted =
                false;

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

            v.drift += 5.0;

            v.trust *= 0.98;
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
                    0.10;

            } else {

                v.confidence *= 0.98;

                v.trust *= 0.99;

                v.drift += 1.0;
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
    // 🧠 IDENTITY SESSION
    // =========================
    pub fn get_or_create_identity(
        &mut self,
        identity_id: String,
        validator_id: u32,
    ) {

        if !self.identities
            .identities
            .contains_key(
                &identity_id
            )
        {

            self.identities
                .identities
                .insert(

                    identity_id.clone(),

                    FluxIdentity {

                        identity_id,

                        created_epoch:
                            self.global_epoch,

                        last_active_epoch:
                            self.global_epoch,

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

    // =========================
    // ✅ AUTH SUCCESS
    // =========================
    pub fn identity_success(
        &mut self,
        identity_id: &str,
    ) {

        if let Some(identity) =
            self.identities
                .identities
                .get_mut(identity_id)
        {

            identity.session_count += 1;

            identity.successful_auths += 1;

            identity.last_active_epoch =
                self.global_epoch;

            identity.continuity_score +=
                0.5;

            identity.credential_depth += 1;
        }
    }

    // =========================
    // ❌ AUTH FAILURE
    // =========================
    pub fn identity_failure(
        &mut self,
        identity_id: &str,
    ) {

        if let Some(identity) =
            self.identities
                .identities
                .get_mut(identity_id)
        {

            identity.failed_auths += 1;

            identity.drift_score += 2.0;

            identity.trust_score *= 0.98;

            identity.status =
                "recovering".into();
        }
    }
}
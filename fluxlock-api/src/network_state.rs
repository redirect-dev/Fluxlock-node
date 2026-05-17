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

use crate::engine::governance::{
    evaluate_governance,
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
                                genesis_hash.clone(),

                            parent_hash:
                                "GENESIS".into(),

                            state_hash:
                                genesis_hash.clone(),

                            lineage_signature:
                                None,

                            transition_signature:
                                None,

                            epoch: 0,

                            continuity_epoch: 0,

                            validator_id: i,

                            governance_weight: 1.0,

                            governance_score: 100.0,

                            governance_votes: 0,

                            network_alignment: 1.0,

                            continuity_confidence: 100.0,

                            peer_agreement_ratio: 1.0,

                            entropy_score: 100.0,

                            lineage_stability: 100.0,

                            fracture_severity: 0.0,

                            rehabilitation_factor: 1.0,

                            quarantine_level: 0.0,

                            malicious_reports: 0,

                            fork_conflicts: 0,

                            continuity_verified: true,
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

                    scar_severity: 0.0,

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
            // =========================
// 🧬 REHABILITATION ENGINE
// =========================
for validator in
    self.validators.iter_mut()
{

    let governance =
        evaluate_governance(
            validator
        );

    // =========================
    // 🛡 STABILIZATION
    // =========================
    validator.drift -=
        governance
            .stabilization_delta;

    validator.drift =
        validator.drift.max(0.0);

    // =========================
    // 🔒 QUARANTINE RECOVERY
    // =========================
    validator.quarantine_level -=
        governance
            .quarantine_reduction;

    validator.quarantine_level =
        validator
            .quarantine_level
            .max(0.0);

    // =========================
    // ❤️ TRUST RECOVERY
    // =========================
    validator.trust +=
        governance
            .trust_bonus;

    validator.trust =
        validator.trust
            .clamp(0.0, 100.0);

    // =========================
    // 🧬 REHABILITATION
    // =========================
    validator.rehabilitation_score +=
        governance
            .rehabilitation_boost;

    validator.rehabilitation_score =
        validator
            .rehabilitation_score
            .clamp(0.0, 1000.0);

    // =========================
    // 🔥 SCAR STABILIZATION
    // =========================
    validator.scar_severity -=
        governance
            .scar_reduction;

    validator.scar_severity =
        validator
            .scar_severity
            .max(0.0);

    // =========================
    // 🛡 IMMUNE RESPONSE
    // =========================
    validator.immune_response +=
        governance
            .immune_response_boost;

    validator.immune_response =
        validator
            .immune_response
            .clamp(0.0, 100.0);

    // =========================
    // 🌐 NETWORK REENTRY
    // =========================
    if governance
        .network_reacceptance
        && validator.trust > 70.0
        && validator.drift < 25.0
    {
        validator.network_accepted =
            true;
    }

    // =========================
    // 🟡 RECOVERY STATUS
    // =========================
    if validator.quarantine_level > 20.0 {

        validator.status =
            "quarantined".into();

    } else if validator.drift > 25.0 {

        validator.status =
            "recovering".into();

    } else {

        validator.status =
            "healthy".into();
    }
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

        let rotated_link =
            rotate_identity(

                validator_id,

                rotation_index,

                validator.governance_weight,

                validator.trust,

                previous_hash.clone(),
            );

        validator.identity_chain.push(

            IdentityLink {

                public_key:
                    rotated_link.public_key,

                signature:
                    rotated_link.signature,

                continuity_hash:
                    rotated_link
                        .continuity_hash
                        .clone(),

                parent_hash:
                    previous_hash,

                state_hash:
                    rotated_link
                        .continuity_hash
                        .clone(),

                lineage_signature:
                    None,

                transition_signature:
                    None,

                epoch:
                    self.global_epoch,

                continuity_epoch:
                    self.global_epoch,

                validator_id,

                governance_weight:
                    validator.governance_weight,

                governance_score:
                    validator.governance_weight
                        * 100.0,

                governance_votes:
                    validator.peer_votes_valid,

                network_alignment:
                    validator.peer_agreement_ratio,

                continuity_confidence:
                    validator.confidence
                        * 100.0,

                peer_agreement_ratio:
                    validator.peer_agreement_ratio,

                entropy_score:
                    validator.trust,

                lineage_stability:
                    validator.lineage_stability,

                fracture_severity:
                    validator.fracture_severity,

                rehabilitation_factor:
                    validator.rehabilitation_score,

                quarantine_level:
                    validator.quarantine_level,

                malicious_reports:
                    validator.malicious_reports,

                fork_conflicts: 0,

                continuity_verified:
                    validator.chain_valid,
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

            v.attack_history += 1;

            v.drift += 15.0;

            v.trust *= 0.95;

            v.scar_level += 1.5;

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

            v.attack_history += 1;

            v.drift += 40.0;

            v.trust *= 0.70;

            v.fracture_severity += 25.0;

            v.quarantine_level += 20.0;

            v.scar_level += 5.0;

            v.network_accepted =
                false;

            v.status =
                "quarantined".into();
        }
    }

    // =========================
    // 🧬 FRACTURE ATTACK
    // =========================
    pub fn fracture_attack(
        &mut self,
        id: u32,
    ) {

        if let Some(v) =
            self.validators
                .iter_mut()
                .find(|v| v.id == id)
        {

            if let Some(last) =
                v.identity_chain.last_mut()
            {

                last.parent_hash =
                    "CORRUPTED_LINEAGE".into();

                last.continuity_hash =
                    format!(
                        "fractured-{}-{}",
                        id,
                        self.global_epoch
                    );
            }

            v.attack_history += 1;

            v.chain_valid = false;

            v.network_accepted = false;

            v.trust *= 0.40;

            v.drift += 65.0;

            v.fracture_severity += 75.0;

            v.quarantine_level += 50.0;

            v.consensus_failures += 1;

            v.malicious_reports += 1;

            v.isolation_events += 1;

            v.scar_level += 25.0;

            v.status =
                "fractured".into();
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

            v.consensus_pressure += 1.0;
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
                    .clamp(-100.0, 100.0);
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

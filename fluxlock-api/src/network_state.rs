use serde::{
    Serialize,
    Deserialize,
};

use crate::engine::continuity_state::
    evaluate_continuity_state;

use std::fs;

use fluxlock_core::types::{
    Validator,
    IdentityLink,
    IdentityRegistry,
    PeerNode,
    FluxIdentity,
    ContinuityState,
    ContinuityEvent,
};

use crate::peer_state::PeerState;

use crate::engine::identity_validator::{
    generate_identity,
    rotate_identity,
    verify_lineage,
};

use crate::engine::governance::{
    evaluate_governance,
};

use crate::engine::continuity_transition::{
    generate_transition_proof,
};

use crate::engine::peer_governance::{
    evaluate_peer_governance,
};

use fluxlock_network::memory::{
    MemoryStore,
    create_memory,
    update_memory_score,
};

use fluxlock_network::memory::memory_pressure::{
    apply_memory_pressure,
};

use fluxlock_network::sync::continuity_mesh::{
    propagate_continuity_mesh,
};

use fluxlock_network::sync::fracture_wave::{
    propagate_fracture_wave,
};

use fluxlock_network::sync::stabilization_field::{
    apply_stabilization_field,
};

use fluxlock_network::sync::topology_pressure::{
    apply_topology_pressure,
};

use fluxlock_network::sync::regional_clusters::{
    evolve_regional_clusters,
};

use fluxlock_storage::validator_store::{
    save_validator,
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

    pub memory_store:
        MemoryStore,

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

        let mut memory_store =
            MemoryStore::new();

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

                    continuity_events:
                        Vec::<ContinuityEvent>::new(),

                    continuity_memory_score: 100.0,

                    historical_consensus_accuracy: 1.0,

                    recovery_consistency: 1.0,

                    adaptive_reputation: 100.0,

                    continuity_survival_score: 100.0,

                    fracture_history: 0,

                    recovery_history: 0,

                    governance_history: 0,

                    continuity_age:
                        restored_epoch,

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

                    influence_radius: 25.0,

                    entropy_output: 0.0,

                    immune_strength: 10.0,

                    healing_wave: 5.0,

                    topology_cluster:
                        (i % 4) as u32,

                    resonance_score: 50.0,

                    regional_pressure: 0.0,

                    trust_gravity: 25.0,

                    mutation_shock: 0.0,

                    continuity_suspicion: 0.0,

                    scrutiny_level: 1.0,

                    evolutionary_authenticity: 100.0,

                   mutation_pressure: 0.0,

                continuity_state:
                        ContinuityState::Healthy,
                }
            );

            memory_store.insert(
                i as u64,
                create_memory(i as u64),
            );
        }

        Self {

            validators,

            identities:
                IdentityRegistry::new(),

            peer_state:
                PeerState::new(),

            memory_store,

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

        let snapshot =
            self.validators.clone();

            for validator in
            self.validators.iter_mut()
{

    evaluate_peer_governance(
        validator,
        &snapshot,
    );
}

        // =========================
        // 🌐 DISTRIBUTED MESH
        // =========================
        propagate_continuity_mesh(
            &mut self.validators
        );

        propagate_fracture_wave(
            &mut self.validators
        );

        apply_stabilization_field(
            &mut self.validators
        );

        apply_topology_pressure(
            &mut self.validators
        );

        evolve_regional_clusters(
            &mut self.validators
        );

        for validator in
            self.validators.iter_mut()
        {
                // =========================
                // 🔗 REVERIFY LINEAGE
                // =========================
                validator.chain_valid =
                   verify_lineage(
                   &validator.identity_chain,
                  validator.id,
                );

                println!(
                "VALIDATOR {} CHAIN VALID = {}",
                 validator.id,
                 validator.chain_valid
                );

                evaluate_continuity_state(
                     validator
                );

            if let Some(memory) =
                self.memory_store
                    .get_mut(&(validator.id as u64))
            {

                memory.stable_epochs += 1;

                memory.lineage_depth =
                    validator
                        .identity_chain
                        .len() as u64;

                memory.inherited_trust =
                    validator.inherited_trust;

                memory.historical_stability =
                    validator.lineage_stability;

                memory.peer_observations += 1;

                if validator.network_accepted {

                    memory.network_acceptances += 1;

                } else {

                    memory.network_rejections += 1;
                }

                if validator.continuity_state
                    == ContinuityState::Fractured
                {

                    memory.fracture_events += 1;
                }

                if validator.continuity_state
                     == ContinuityState::Recovering
                {

                    memory.recovery_events += 1;
                }

                update_memory_score(
                    memory
                );

                validator.continuity_memory_score =
                    memory.continuity_memory_score;

                apply_memory_pressure(
                    validator
                );
            }

            let governance =
                evaluate_governance(
                    validator
                );
            // =========================
            // 🌐 NETWORK REACCEPTANCE
            // =========================
                if governance.network_reacceptance
                    && validator.chain_valid
                    && validator.trust > 50.0
                    && validator.drift < 50.0 {

                  validator.network_accepted = true;

    if validator.recovery_timer > 0 {

        validator.successful_recoveries += 1;

        validator.recovery_timer = 0;
    }
}

            validator.drift -=
                governance
                    .stabilization_delta;

            validator.drift =
                validator.drift.max(0.0);

            validator.quarantine_level -=
                governance
                    .quarantine_reduction;

            validator.quarantine_level =
                validator
                    .quarantine_level
                    .max(0.0);

            validator.trust +=
                governance
                    .trust_bonus;

            validator.trust =
                validator.trust
                    .clamp(0.0, 100.0);

            validator.rehabilitation_score +=
                governance
                    .rehabilitation_boost;

            validator.rehabilitation_score =
                validator
                    .rehabilitation_score
                    .clamp(0.0, 1000.0);

            validator.scar_severity -=
                governance
                    .scar_reduction;

            validator.scar_severity =
                validator
                    .scar_severity
                    .max(0.0);

            validator.immune_response +=
                governance
                    .immune_response_boost;

            validator.immune_response =
                validator
                    .immune_response
                    .clamp(0.0, 100.0);

            validator.epoch_age += 1;

            validator.continuity_age += 1;

            validator.governance_history += 1;

            validator.continuity_memory_score += 0.01;

            validator.adaptive_reputation += 0.01;

                        // =========================
            // 🧬 MUTATION GOVERNANCE
            // =========================
            validator.mutation_shock =
                (
                    validator.entropy_output
                    * 0.50
                )
                +
                (
                    validator.fracture_severity
                    * 0.30
                )
                +
                (
                    validator.consensus_pressure
                    * 0.20
                );

            validator.mutation_pressure +=
                validator.mutation_shock
                * 0.002;

            if validator.mutation_shock > 40.0 {

                validator.continuity_suspicion +=
                    0.50;

                validator.scrutiny_level +=
                    0.05;

            } else {

                validator.continuity_suspicion *=
                    0.995;

                validator.scrutiny_level *=
                    0.999;
            }

            validator.evolutionary_authenticity =
                (
                    validator.lineage_stability
                    * 0.40
                )
                +
                (
                    validator.continuity_memory_score
                    * 0.40
                )
                -
                (
                    validator.mutation_shock
                    * 0.20
                );

            validator.continuity_suspicion =
                validator
                    .continuity_suspicion
                    .clamp(0.0, 100.0);

            validator.scrutiny_level =
                validator
                    .scrutiny_level
                    .clamp(1.0, 10.0);

            validator.evolutionary_authenticity =
                validator
                    .evolutionary_authenticity
                    .clamp(0.0, 100.0);
            
                    // =========================
                    // 🧬 CANONICAL RECOVERY
                    // =========================
            if validator.network_accepted
                && validator.chain_valid
                && validator.trust > 80.0
                && validator.drift < 15.0 {

               validator.fracture_severity *= 0.995;

               validator.quarantine_level *= 0.990;

               validator.continuity_suspicion *= 0.995;

               validator.rehabilitation_score += 0.10;
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

        v.continuity_state =
                ContinuityState::Recovering;
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

        v.network_accepted =
            false;

        v.continuity_state =
             ContinuityState::Quarantined;
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

        v.chain_valid = false;

        v.network_accepted = false;

        v.trust *= 0.40;

        v.drift += 65.0;

        v.fracture_severity += 75.0;

        v.continuity_state =
             ContinuityState::Fractured;
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
                .clamp(-100.0, 100.0);
    }
}

// =========================
// 🔐 GET OR CREATE IDENTITY
// =========================
pub fn get_or_create_identity(
    &mut self,
    identity_id: String,
    validator_id: u32,
) {

    if !self
        .identities
        .identities
        .contains_key(
            &identity_id
        )
    {

        self.identities
            .create_identity(

                identity_id,

                validator_id,

                self.global_epoch,
            );
    }
}

// =========================
// ✅ IDENTITY SUCCESS
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

        identity.successful_auths += 1;

        identity.session_count += 1;

        identity.last_active_epoch =
            self.global_epoch;

        identity.trust_score += 1.0;

        identity.continuity_score += 0.5;

        identity.status =
            "healthy".into();
    }
}

// =========================
// ❌ IDENTITY FAILURE
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

        identity.drift_score += 5.0;

        identity.trust_score *= 0.97;

        identity.continuity_score *= 0.98;

        identity.status =
            "recovering".into();
    }
}

// =========================
// 🧬 EVOLVE IDENTITY
// =========================
pub fn evolve_identity(
    &mut self,
    validator_id: u32,
) {

    if let Some(v) =
        self.validators
            .iter_mut()
            .find(|v|
                v.id == validator_id
            )
    {

        let previous_hash =
            v.identity_chain
                .last()
                .map(|l|
                    l.continuity_hash
                        .clone()
                )
                .unwrap_or_else(
                    ||
                    "GENESIS".into()
                );

        let new_link =
            rotate_identity(

                validator_id,

                v.identity_chain.len(),

                v.governance_weight,

                v.entropy_output,

                previous_hash.clone(),
            );

                let proof =
            generate_transition_proof(

                v,

                previous_hash.clone(),

                new_link
                    .continuity_hash
                    .clone(),
            );

        println!(
            "🧬 CONTINUITY PROOF | Validator {} | Confidence {:.2} | Drift {:.2} | Verified {}",
            validator_id,
            proof.continuity_confidence,
            proof.continuity_drift,
            proof.continuity_verified
        );

        v.identity_chain.push(
            new_link
        );

        v.epoch_rotations += 1;

        v.last_epoch_transition =
            self.global_epoch;

        save_identity_chain(
            validator_id,
            &v.identity_chain
        )
        .ok();

        save_validator(v).ok();
    }
}
}

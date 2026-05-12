use serde::{
    Serialize,
    Deserialize,
};

use fluxlock_core::types::{
    Validator,
    IdentityLink,
    IdentityRegistry,
};

use crate::engine::identity_validator::{
    generate_identity,
    rotate_identity,
    verify_lineage,
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

                    status:
                        "healthy".into(),
                }
            );
        }

        Self {

            validators,

            identities:
                IdentityRegistry::new(),

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

            if validator.drift > 25.0 {

                validator.status =
                    "recovering".into();

                validator.confidence *= 0.993;

                validator.trust *= 0.997;

            } else {

                validator.status =
                    "healthy".into();

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

            // =========================
            // 💾 PERSIST VALIDATOR
            // =========================
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

            // =========================
            // 💾 PERSIST IDENTITY
            // =========================
            let _ =
                save_identity(
                    identity
                );
        }
    }

    // =========================
    // 🌐 EPOCH ROTATION
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

        if validator.identity_chain.len() > 64 {

            validator.identity_chain.remove(0);
        }

        // =========================
        // 💾 PERSIST EVOLVED LINEAGE
        // =========================
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
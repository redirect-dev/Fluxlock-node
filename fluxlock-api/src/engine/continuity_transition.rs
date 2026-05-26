use fluxlock_core::types::{
    Validator,
    ContinuityTransitionProof,
};

use sha2::{
    Sha256,
    Digest,
};

// =========================
// 🧬 TRANSITION RESULT
// =========================
pub struct TransitionValidation {

    pub continuity_verified: bool,

    pub mutation_coherence: f64,

    pub continuity_drift: f64,

    pub lineage_authenticity: f64,

    pub continuity_confidence: f64,

    pub mutation_shock: f64,

    pub evolutionary_authenticity: f64,
}

// =========================
// 🧬 VALIDATE TRANSITION
// =========================
pub fn validate_transition(
    validator: &Validator,
) -> TransitionValidation {

    let chain_depth =
        validator.identity_chain.len()
            as f64;

    // =========================
    // 🧠 MUTATION COHERENCE
    // =========================
    let mutation_coherence =
        (
            validator.lineage_stability
            * 0.45
        )
        +
        (
            validator.continuity_memory_score
            * 0.35
        )
        +
        (
            validator.peer_agreement_ratio
            * 100.0
            * 0.20
        );

    // =========================
    // ☠ CONTINUITY DRIFT
    // =========================
    let continuity_drift =
        validator.drift
        +
        (
            validator.fracture_severity
            * 0.5
        )
        +
        (
            validator.entropy_output
            * 0.25
        );

    // =========================
    // 🧬 LINEAGE AUTHENTICITY
    // =========================
    let lineage_authenticity =
        (
            validator.inherited_trust
            * 0.40
        )
        +
        (
            validator.lineage_stability
            * 0.40
        )
        +
        (
            chain_depth
            * 0.20
        );

    // =========================
    // ⚠ MUTATION SHOCK
    // =========================
    let mutation_shock =
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

    // =========================
    // 🧠 EVOLUTION AUTHENTICITY
    // =========================
    let evolutionary_authenticity =
        mutation_coherence
        +
        lineage_authenticity
        -
        mutation_shock;

    // =========================
    // 🔐 CONTINUITY CONFIDENCE
    // =========================
    let continuity_confidence =
        evolutionary_authenticity
        -
        continuity_drift;

    // =========================
    // 🧬 VERIFIED
    // =========================
    let continuity_verified =
        continuity_confidence > 60.0
        &&
        continuity_drift < 80.0
        &&
        mutation_shock < 70.0
        &&
        validator.chain_valid;

    TransitionValidation {

        continuity_verified,

        mutation_coherence,

        continuity_drift,

        lineage_authenticity,

        continuity_confidence,

        mutation_shock,

        evolutionary_authenticity,
    }
}

// =========================
// 🔐 GOVERNANCE HASH
// =========================
pub fn governance_attestation_hash(

    validator: &Validator,

    validation: &TransitionValidation,
) -> String {

    let chain_depth =
        validator.identity_chain.len();

    let payload = format!(

        "{}:{}:{}:{}:{}:{}:{}",

        validator.id,

        chain_depth,

        validator.governance_weight,

        validator.peer_agreement_ratio,

        validation.mutation_shock,

        validation.evolutionary_authenticity,

        validation.continuity_confidence,
    );

    let mut hasher =
        Sha256::new();

    hasher.update(
        payload.as_bytes()
    );

    format!(
        "{:x}",
        hasher.finalize()
    )
}

// =========================
// 🧬 GENERATE PROOF
// =========================
pub fn generate_transition_proof(

    validator: &Validator,

    previous_hash: String,

    new_hash: String,
) -> ContinuityTransitionProof {

    let validation =
        validate_transition(
            validator
        );

    let attestation =
        governance_attestation_hash(
            validator,
            &validation,
        );

    ContinuityTransitionProof {

        validator_id:
            validator.id,

        previous_hash:
            previous_hash.clone(),

        new_hash:
             new_hash.clone(),

        epoch:
            validator.current_epoch,

        mutation_coherence:
            validation
                .mutation_coherence,

        continuity_drift:
            validation
                .continuity_drift,

        lineage_authenticity:
            validation
                .lineage_authenticity,

        continuity_confidence:
            validation
                .continuity_confidence,

        governance_weight:
            validator
                .governance_weight,

        entropy_score:
            validator
                .entropy_output,
        
        transition_signature:
               Some(
        attestation.clone()
    ),

        lineage_signature:
               Some(
        previous_hash.clone()
    ),

proof_hash:
    attestation.clone(),
        continuity_verified:
            validation
                .continuity_verified,

        governance_votes: 0,

        governance_approvals: 0,

        governance_rejections: 0,

        network_alignment:
            validator.peer_agreement_ratio,

        mutation_shock:
            validation.mutation_shock,

        evolutionary_authenticity:
            validation.evolutionary_authenticity,

        continuity_attestation:
            attestation,
    }
}
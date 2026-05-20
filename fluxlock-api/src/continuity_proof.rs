use serde::{
    Serialize,
    Deserialize,
};

use fluxlock_core::types::{
    Validator,
    IdentityLink,
};

use sha2::{
    Sha256,
    Digest,
};

use base64::{
    engine::general_purpose,
    Engine as _,
};

use pqcrypto_dilithium::dilithium2;

use pqcrypto_traits::sign::{
    SecretKey,
    SignedMessage,
};

// =========================
// 🧬 CONTINUITY PROOF
// =========================
#[derive(
    Clone,
    Serialize,
    Deserialize,
)]
pub struct ContinuityProof {

    // =========================
    // 🌐 IDENTITY
    // =========================
    pub validator_id: u32,

    pub lineage_depth: usize,

    pub continuity_hash: String,

    pub parent_hash: String,

    // =========================
    // 🧠 GOVERNANCE
    // =========================
    pub governance_weight: f64,

    pub governance_score: f64,

    pub rehabilitation_score: f64,

    pub peer_agreement_ratio: f64,

    // =========================
    // 🌊 STABILITY
    // =========================
    pub trust: f64,

    pub drift: f64,

    pub fracture_severity: f64,

    pub quarantine_level: f64,

    pub resilience_score: f64,

    // =========================
    // 🌐 NETWORK
    // =========================
    pub network_accepted: bool,

    pub chain_valid: bool,

    pub status: String,

    // =========================
    // 🔐 CRYPTO
    // =========================
    pub proof_hash: String,

    pub validator_signature: String,

    pub proof_version: u32,

    pub signed_epoch: u64,
}

// =========================
// 🧬 BUILD PROOF
// =========================
pub fn build_continuity_proof(
    validator: &Validator,
    signing_key: &dilithium2::SecretKey,
) -> Option<ContinuityProof> {

    let latest =
        validator
            .identity_chain
            .last()?;

    let proof_hash =
        generate_proof_hash(
            validator,
            latest,
        );

    // =========================
    // 🔐 SIGN HASH
    // =========================
    let signed =
        dilithium2::sign(
            proof_hash.as_bytes(),
            signing_key,
        );

    let signature =
        general_purpose::STANDARD.encode(
            signed.as_bytes()
        );

    Some(

        ContinuityProof {

            validator_id:
                validator.id,

            lineage_depth:
                validator
                    .identity_chain
                    .len(),

            continuity_hash:
                latest
                    .continuity_hash
                    .clone(),

            parent_hash:
                latest
                    .parent_hash
                    .clone(),

            governance_weight:
                validator
                    .governance_weight,

            governance_score:
                latest
                    .governance_score,

            rehabilitation_score:
                validator
                    .rehabilitation_score,

            peer_agreement_ratio:
                validator
                    .peer_agreement_ratio,

            trust:
                validator.trust,

            drift:
                validator.drift,

            fracture_severity:
                validator
                    .fracture_severity,

            quarantine_level:
                validator
                    .quarantine_level,

            resilience_score:
                validator
                    .resilience_score,

            network_accepted:
                validator
                    .network_accepted,

            chain_valid:
                validator
                    .chain_valid,

            status:
                validator
                    .status
                    .clone(),

            proof_hash,

            validator_signature:
                signature,

            proof_version: 1,

            signed_epoch:
                validator
                    .current_epoch,
        }
    )
}

// =========================
// 🔐 HASH PROOF
// =========================
fn generate_proof_hash(
    validator: &Validator,
    latest: &IdentityLink,
) -> String {

    let payload =
        format!(

            "{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",

            validator.id,

            validator.identity_chain.len(),

            latest.continuity_hash,

            latest.parent_hash,

            validator.governance_weight,

            validator.trust,

            validator.drift,

            validator.fracture_severity,

            validator.peer_agreement_ratio,

            validator.network_accepted,

            validator.current_epoch
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
// ✅ VERIFY PROOF
// =========================
pub fn verify_proof(
    proof: &ContinuityProof,
) -> bool {

    if !proof.chain_valid {
        return false;
    }

    if proof.lineage_depth == 0 {
        return false;
    }

    if proof.continuity_hash.is_empty() {
        return false;
    }

    if proof.governance_weight <= 0.0 {
        return false;
    }

    if proof.trust < 0.0 {
        return false;
    }

    if proof.drift > 500.0 {
        return false;
    }

    if proof.validator_signature.is_empty() {
        return false;
    }

    true
}
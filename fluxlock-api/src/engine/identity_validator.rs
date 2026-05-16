use pqcrypto_dilithium::dilithium2;

use pqcrypto_traits::sign::{
    PublicKey,
    DetachedSignature,
};

use std::collections::HashMap;
use std::sync::Mutex;

use once_cell::sync::Lazy;

use fluxlock_core::types::{
    IdentityLink,
};

// =========================
// 🔐 GLOBAL KEY STORE
// =========================
pub static KEY_STORE: Lazy<
    Mutex<
        HashMap<
            u32,
            (
                dilithium2::PublicKey,
                dilithium2::SecretKey
            )
        >
    >
> = Lazy::new(|| Mutex::new(HashMap::new()));

// =========================
// 🔐 GOVERNANCE ENTROPY
// =========================
fn governance_entropy(
    validator_id: u32,
    chain_depth: usize,
    governance_weight: f64,
    entropy_score: f64,
) -> String {

    format!(
        "{}:{}:{:.4}:{:.4}",
        validator_id,
        chain_depth,
        governance_weight,
        entropy_score
    )
}

// =========================
// 🔑 CREATE IDENTITY
// =========================
pub fn generate_identity(
    id: u32
) -> Vec<u8> {

    let (pk, sk) =
        dilithium2::keypair();

    let mut store =
        KEY_STORE.lock().unwrap();

    store.insert(
        id,
        (pk.clone(), sk)
    );

    pk.as_bytes().to_vec()
}

// =========================
// 🔁 ROTATE IDENTITY
// =========================
pub fn rotate_identity(

    validator_id: u32,

    chain_depth: usize,

    governance_weight: f64,

    entropy_score: f64,

    previous_hash: String,

) -> IdentityLink {

    let mut store =
        KEY_STORE.lock().unwrap();

    let (_, old_sk) =
        store
            .get(&validator_id)
            .unwrap()
            .clone();

    // =========================
    // 🔑 NEW KEYPAIR
    // =========================
    let (new_pk, new_sk) =
        dilithium2::keypair();

    let public_key =
        new_pk.as_bytes().to_vec();

    // =========================
    // 🧠 ENTROPY PAYLOAD
    // =========================
    let entropy_payload =
        governance_entropy(
            validator_id,
            chain_depth,
            governance_weight,
            entropy_score,
        );

    // =========================
    // 🔗 SUCCESSION MESSAGE
    // =========================
    let message =
        format!(
            "{}:{}:{}",
            validator_id,
            chain_depth,
            previous_hash
        );

    // =========================
    // 🔐 SIGN SUCCESSOR
    // =========================
    let detached =
        dilithium2::detached_sign(
            message.as_bytes(),
            &old_sk
        );

    let signature =
        Some(
            detached
                .as_bytes()
                .to_vec()
        );

    // =========================
    // 🔁 STORE NEW KEYS
    // =========================
    store.insert(
        validator_id,
        (
            new_pk.clone(),
            new_sk
        )
    );

    // =========================
    // 🧬 CONTINUITY HASH
    // =========================
    let continuity_hash =
        format!(
            "{:x}",
            md5::compute(
                format!(
                    "{}:{}:{}:{}",
                    validator_id,
                    chain_depth,
                    previous_hash,
                    entropy_payload
                )
            )
        );

    // =========================
    // 🔗 RETURN LINK
    // =========================
    IdentityLink {

        // =========================
        // 🔐 CRYPTO
        // =========================
        public_key,

        signature,

        // =========================
        // 🔗 CONTINUITY
        // =========================
        continuity_hash:
            continuity_hash.clone(),

        parent_hash:
            previous_hash,

        state_hash:
            continuity_hash.clone(),

        lineage_signature:
            None,

        transition_signature:
            None,

        // =========================
        // 🌐 EPOCH
        // =========================
        epoch:
            chain_depth as u64,

        continuity_epoch:
            chain_depth as u64,

        validator_id,

        // =========================
        // 🧠 GOVERNANCE
        // =========================
        governance_weight,

        governance_score:
            governance_weight * 100.0,

        governance_votes: 0,

        // =========================
        // 🌐 CONSENSUS
        // =========================
        network_alignment:
            1.0,

        continuity_confidence:
            100.0,

        peer_agreement_ratio:
            1.0,

        // =========================
        // 🧬 STABILITY
        // =========================
        entropy_score,

        lineage_stability:
            100.0,

        fracture_severity:
            0.0,

        rehabilitation_factor:
            1.0,

        // =========================
        // ⚠ SECURITY
        // =========================
        quarantine_level:
            0.0,

        malicious_reports: 0,

        fork_conflicts: 0,

        continuity_verified:
            true,
    }
}

// =========================
// ✅ VERIFY LINK
// =========================
pub fn verify_link(

    old_pk_bytes: &[u8],

    message: &[u8],

    sig_bytes: &[u8],

) -> bool {

    let pk =
        match dilithium2::PublicKey
            ::from_bytes(old_pk_bytes)
    {
        Ok(pk) => pk,
        Err(_) => return false,
    };

    let sig =
        match dilithium2::DetachedSignature
            ::from_bytes(sig_bytes)
    {
        Ok(sig) => sig,
        Err(_) => return false,
    };

    dilithium2
        ::verify_detached_signature(
            &sig,
            message,
            &pk
        )
        .is_ok()
}

// =========================
// 🔗 VERIFY ENTIRE LINEAGE
// =========================
pub fn verify_lineage(

    chain: &Vec<IdentityLink>,

    validator_id: u32,

) -> bool {

    if chain.is_empty() {

        return false;
    }

    // =========================
    // 🟢 GENESIS VALID
    // =========================
    if chain.len() == 1 {

        return true;
    }

    // =========================
    // 🔗 VERIFY SUCCESSION
    // =========================
    for i in 1..chain.len() {

        let previous =
            &chain[i - 1];

        let current =
            &chain[i];

        // =========================
        // 🔗 HASH CONTINUITY
        // =========================
        if current.parent_hash
            != previous.continuity_hash
        {

            return false;
        }

        // =========================
        // 🔐 SIGNATURE REQUIRED
        // =========================
        let signature =
            match &current.signature {

                Some(sig) => sig,

                None => return false,
            };

        // =========================
        // 🔐 REBUILD MESSAGE
        // =========================
        let message =
            format!(
                "{}:{}:{}",
                validator_id,
                i,
                previous.continuity_hash
            );

        // =========================
        // 🔐 VERIFY SUCCESSION
        // =========================
        let valid =
            verify_link(
                &previous.public_key,
                message.as_bytes(),
                signature,
            );

        if !valid {

            return false;
        }

        // =========================
        // 🧠 ENTROPY FLOOR
        // =========================
        if current.entropy_score < 0.0 {

            return false;
        }

        // =========================
        // 🌐 GOVERNANCE FLOOR
        // =========================
        if current.governance_weight < 0.0 {

            return false;
        }
    }

    true
}

// =========================
// 🧠 VALIDATION RESULT
// =========================
pub struct ValidationResult {

    pub valid: bool,

    pub reason: String,
}

// =========================
// 🧠 VALIDATION LOGIC
// =========================
pub fn validate_identity_logic(

    trust: f64,

    drift: f64,

    epoch_age: u64,

    epoch_valid: bool,

    compromised: bool,

    network_accepted: bool,

    governance_weight: f64,

    continuity_score: f64,

) -> ValidationResult {

    if compromised {

        return ValidationResult {

            valid: false,

            reason:
                "identity compromised"
                    .into(),
        };
    }

    if !epoch_valid {

        return ValidationResult {

            valid: false,

            reason:
                "broken lineage detected"
                    .into(),
        };
    }

    if epoch_age < 120 {

        return ValidationResult {

            valid: false,

            reason:
                "identity too new"
                    .into(),
        };
    }

    if !network_accepted {

        return ValidationResult {

            valid: false,

            reason:
                "network rejected identity"
                    .into(),
        };
    }

    if drift > 80.0 {

        return ValidationResult {

            valid: false,

            reason:
                "identity unstable"
                    .into(),
        };
    }

    if governance_weight <= 0.0 {

        return ValidationResult {

            valid: false,

            reason:
                "governance collapse"
                    .into(),
        };
    }

    if continuity_score < 25.0 {

        return ValidationResult {

            valid: false,

            reason:
                "continuity collapse"
                    .into(),
        };
    }

    if trust < 60.0 {

        return ValidationResult {

            valid: true,

            reason:
                "identity recovering"
                    .into(),
        };
    }

    ValidationResult {

        valid: true,

        reason:
            "identity stable and verified"
                .into(),
    }
}
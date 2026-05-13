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
    id: u32,
    chain_depth: usize,
) -> IdentityLink {

    let mut store =
        KEY_STORE.lock().unwrap();

    let (_, old_sk) =
        store
            .get(&id)
            .unwrap()
            .clone();

    // =========================
    // 🔑 NEW KEYPAIR
    // =========================
    let (new_pk, new_sk) =
        dilithium2::keypair();

    // =========================
    // 🔗 SIGN SUCCESSOR
    // =========================
    let message =
        format!(
            "validator:{}:depth:{}",
            id,
            chain_depth
        );

    let signature =
        dilithium2::detached_sign(
            message.as_bytes(),
            &old_sk
        );

    // =========================
    // 🔁 STORE NEW KEYS
    // =========================
    store.insert(
        id,
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
                    "{}:{}",
                    id,
                    chain_depth
                )
            )
        );

    // =========================
    // 🔗 RETURN LINK
    // =========================
    IdentityLink {

        public_key:
            new_pk
                .as_bytes()
                .to_vec(),

        signature:
            Some(
                signature
                    .as_bytes()
                    .to_vec()
            ),

        continuity_hash:
            continuity_hash,

        parent_hash:
            format!(
                "parent:{}",
                chain_depth
            ),

        epoch:
            chain_depth as u64,

        validator_id:
            id,

        governance_weight:
            1.0,

        entropy_score:
            100.0,
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

        let signature =
            match &current.signature {

                Some(sig) => sig,

                None => return false,
            };

        let message =
            format!(
                "validator:{}:depth:{}",
                validator_id,
                i
            );

        let valid =
            verify_link(
                &previous.public_key,
                message.as_bytes(),
                signature,
            );

        if !valid {

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
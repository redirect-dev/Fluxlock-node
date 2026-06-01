use axum::{
    extract::State,
    Json,
};

use fluxlock_core::types::ContinuityState;

use serde::{
    Deserialize,
    Serialize,
};

use std::collections::HashSet;
use std::sync::{
    Arc,
    Mutex,
};

use std::time::{
    SystemTime,
    UNIX_EPOCH,
};

use once_cell::sync::Lazy;

use base64::{
    engine::general_purpose,
    Engine as _,
};

use pqcrypto_dilithium::dilithium2;
use pqcrypto_traits::sign::SignedMessage;

use crate::network_state::NetworkState;
use crate::state::KEY_STORE;

// =========================
// 🔐 REPLAY PROTECTION
// =========================
static NONCE_STORE:
    Lazy<Mutex<HashSet<String>>> =
        Lazy::new(|| Mutex::new(HashSet::new()));

const MAX_TIME_WINDOW: u64 = 30;

// =========================
// 📥 AUTH REQUEST
// =========================
#[derive(Debug, Deserialize)]
pub struct AuthRequest {

    pub message: String,

    pub signature: String,

    pub validator_id: u32,

    pub identity_id: String,

    pub nonce: String,

    pub timestamp: serde_json::Value,
}

// =========================
// 📤 AUTH RESPONSE
// =========================
#[derive(Serialize)]
pub struct AuthResponse {

    pub authenticated: bool,

    pub signature_valid: bool,

    pub identity_valid: bool,

    pub allowed: bool,

    pub confidence: f64,

    pub reason: String,

    pub epoch_age: u64,

    pub trust: f64,

    pub drift: f64,

    pub status: String,

    pub identity_id: String,

    pub continuity_score: f64,

    pub session_count: u64,

    pub credential_depth: u64,

    pub lineage_depth: usize,
}

// =========================
// 🔐 AUTH FLOW
// =========================
pub async fn auth_flow(

    State(state):
        State<Arc<Mutex<NetworkState>>>,

    Json(payload):
        Json<AuthRequest>,
) -> Json<AuthResponse> {

    println!("🔥 AUTH REQUEST RECEIVED");

    // =========================
    // ⏱ TIMESTAMP
    // =========================
    let timestamp =
        payload.timestamp
            .as_u64()
            .unwrap_or(0);

    let now =
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

    if now.abs_diff(timestamp)
        > MAX_TIME_WINDOW
    {

        return failure_response(

            payload.identity_id,

            "timestamp expired",

            "expired",
        );
    }

    // =========================
    // 🔁 NONCE CHECK
    // =========================
    {
        let mut nonce_store =
            NONCE_STORE.lock().unwrap();

        if nonce_store.contains(
            &payload.nonce
        ) {

            return failure_response(

                payload.identity_id,

                "replay detected",

                "replay",
            );
        }

        nonce_store.insert(
            payload.nonce.clone()
        );
    }

    // =========================
    // 🔐 FETCH VALIDATOR KEY
    // =========================
    let store =
        KEY_STORE.lock().unwrap();

    let (pk, _) =
        match store.get(
            &payload.validator_id
        )
    {

        Some(pair) => pair,

        None => {

            return failure_response(

                payload.identity_id,

                "validator not found",

                "unknown",
            );
        }
    };

    // =========================
    // 🔓 DECODE SIGNATURE
    // =========================
    let decoded =
        match general_purpose
            ::STANDARD
            .decode(
                &payload.signature
            )
    {

        Ok(bytes) => bytes,

        Err(_) => {

            return failure_response(

                payload.identity_id,

                "invalid signature encoding",

                "invalid",
            );
        }
    };

    // =========================
    // 🔓 PARSE MESSAGE
    // =========================
    let signed_msg =
        match dilithium2
            ::SignedMessage
            ::from_bytes(
                &decoded
            )
    {

        Ok(msg) => msg,

        Err(_) => {

            return failure_response(

                payload.identity_id,

                "invalid signed message",

                "invalid",
            );
        }
    };

    // =========================
    // 🔐 VERIFY SIGNATURE
    // =========================
    let opened =
        match dilithium2::open(
            &signed_msg,
            pk,
        )
    {

        Ok(msg) => msg,

        Err(_) => {

            return failure_response(

                payload.identity_id,

                "signature verification failed",

                "invalid",
            );
        }
    };

    let opened_str =
        String::from_utf8_lossy(
            &opened
        );

    if opened_str != payload.message {

        return failure_response(

            payload.identity_id,

            "message mismatch",

            "invalid",
        );
    }

    drop(store);

    // =========================
    // 🌐 NETWORK STATE
    // =========================
    let mut state =
        state.lock().unwrap();

    state.get_or_create_identity(

        payload.identity_id.clone(),

        payload.validator_id,
    );

    // =========================
    // 🔍 VALIDATOR SNAPSHOT
    // =========================
    let validator =
        match state
            .validators
            .iter()
            .find(
                |v|
                    v.id
                    == payload.validator_id
            )
    {

        Some(v) => v.clone(),

        None => {

            return failure_response(

                payload.identity_id,

                "validator missing",

                "unknown",
            );
        }
    };

    // =========================
    // 🔗 CONTINUITY VALIDATION
    // =========================
let mut allowed = false;

let mut confidence = 0.0;

let reason: String;

match validator.continuity_state {

    ContinuityState::Exiled => {

        reason =
            "continuity permanently rejected"
                .into();
    }

    ContinuityState::Fractured => {

        reason =
            "continuity fractured"
                .into();
    }

    ContinuityState::Quarantined => {

        confidence = 0.10;

        reason =
            "continuity quarantined"
                .into();
    }

    ContinuityState::Rehabilitating => {

        allowed = true;

        confidence = 0.50;

        reason =
            "continuity rehabilitation active"
                .into();
    }

    ContinuityState::Recovering => {

        allowed = true;

        confidence =
            (
                validator.confidence * 0.75
            )
            .clamp(0.0, 1.0);

        reason =
            "continuity recovery active"
                .into();
    }

    ContinuityState::Evolving => {

        allowed = true;

        confidence = 0.75;

        reason =
            "identity evolution in progress"
                .into();
    }

    ContinuityState::Healthy => {

        if !validator.network_accepted {

            confidence =
                validator.confidence
                    * 0.5;

            reason =
                "network rejected identity"
                    .into();

        } else {

            allowed = true;

            confidence =
                validator
                    .confidence
                    .clamp(0.0, 1.0);

            reason =
                "authenticated (continuity verified)"
                    .into();
        }
    }
}

    // =========================
    // 🧠 FEEDBACK
    // =========================
    state.apply_access_feedback(

        payload.validator_id,

        allowed,

        confidence,
    );

    // =========================
    // 🔗 EVOLUTION
    // =========================
    if allowed {

        state.identity_success(
            &payload.identity_id,
        );

        state.evolve_identity(
            payload.validator_id,
        );

    } else {

        state.identity_failure(
            &payload.identity_id,
        );
    }

    // =========================
    // 🔄 REVERIFY LINEAGE
    // =========================
    if let Some(v) =
        state.validators
            .iter_mut()
            .find(
                |v|
                    v.id
                    == payload.validator_id
            )
    {

        v.chain_valid =
            crate::engine
                ::identity_validator
                ::verify_lineage(
                    &v.identity_chain,
                    v.id,
                );
    }

    // =========================
    // 🔍 UPDATED VALIDATOR
    // =========================
    let evolved_validator =
        state.validators
            .iter()
            .find(
                |v|
                    v.id
                    == payload.validator_id
            )
            .unwrap()
            .clone();

    println!(
        "🔗 CHAIN DEPTH: {}",
        evolved_validator
            .identity_chain
            .len()
    );

    let identity =
        state.identities
            .identities
            .get(
                &payload.identity_id
            )
            .unwrap()
            .clone();

    // =========================
    // 📤 RESPONSE
    // =========================
    Json(
        AuthResponse {

            authenticated:
                allowed,

            signature_valid:
                true,

            identity_valid:
                evolved_validator
                    .chain_valid,

            allowed,

            confidence,

            reason,

            epoch_age:
                evolved_validator
                    .epoch_age,

            trust:
                evolved_validator
                    .trust,

            drift:
                evolved_validator
                    .drift,

            status:
                format!(
                    "{:?}",
                    evolved_validator
                        .continuity_state
                ),

            identity_id:
                identity.identity_id,

            continuity_score:
                identity
                    .continuity_score,

            session_count:
                identity
                    .session_count,

            credential_depth:
                identity
                    .credential_depth,

            lineage_depth:
                evolved_validator
                    .identity_chain
                    .len(),
        }
    )
}

// =========================
// ❌ FAILURE RESPONSE
// =========================
fn failure_response(

    identity_id: String,

    reason: &str,

    status: &str,
) -> Json<AuthResponse> {

    Json(
        AuthResponse {

            authenticated: false,

            signature_valid: false,

            identity_valid: false,

            allowed: false,

            confidence: 0.0,

            reason:
                reason.into(),

            epoch_age: 0,

            trust: 0.0,

            drift: 0.0,

            status:
                status.into(),

            identity_id,

            continuity_score: 0.0,

            session_count: 0,

            credential_depth: 0,

            lineage_depth: 0,
        }
    )
}
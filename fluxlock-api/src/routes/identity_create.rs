use axum::{
    extract::State,
    Json,
};

use serde::{
    Deserialize,
    Serialize,
};

use std::sync::{
    Arc,
    Mutex,
};

use std::time::{
    SystemTime,
    UNIX_EPOCH,
};

use crate::network_state::NetworkState;

use fluxlock_core::types::{
    FluxIdentity,
};

// =========================
// 📥 REQUEST
// =========================
#[derive(
    Deserialize
)]
pub struct IdentityCreateRequest {

    pub validator_id: u32,
}

// =========================
// 📤 RESPONSE
// =========================
#[derive(
    Serialize
)]
pub struct IdentityCreateResponse {

    pub success: bool,

    pub identity_id: String,

    pub validator_id: u32,

    pub continuity_score: f64,

    pub trust_score: f64,

    pub credential_depth: u64,

    pub created_epoch: u64,

    pub status: String,
}

// =========================
// 🆕 CREATE IDENTITY
// =========================
pub async fn create_identity(

    State(state):
        State<
            Arc<
                Mutex<NetworkState>
            >
        >,

    Json(payload):
        Json<IdentityCreateRequest>,

) -> Json<IdentityCreateResponse> {

    let mut state =
        state.lock().unwrap();

    // =========================
    // 🧬 UNIQUE ID
    // =========================
    let timestamp =
        SystemTime::now()
            .duration_since(
                UNIX_EPOCH
            )
            .unwrap()
            .as_millis();

    let identity_id =
        format!(
            "flux-{}-{}",
            payload.validator_id,
            timestamp
        );

    let current_epoch =
        state.global_epoch;

    // =========================
    // 🌐 CREATE IDENTITY
    // =========================
    let identity =
        FluxIdentity {

            identity_id:
                identity_id.clone(),

            created_epoch:
                current_epoch,

            last_active_epoch:
                current_epoch,

            session_count: 0,

            trust_score: 100.0,

            continuity_score: 100.0,

            bound_validator:
                payload.validator_id,

            successful_auths: 0,

            failed_auths: 0,

            recovery_events: 0,

            drift_score: 0.0,

            status:
                "healthy".into(),

            credential_depth: 1,

            proofs: Vec::new(),
        };

    state
        .identities
        .create_identity(
            identity.clone()
        );

    // =========================
    // 💾 RESPONSE
    // =========================
    Json(
        IdentityCreateResponse {

            success: true,

            identity_id:
                identity.identity_id,

            validator_id:
                payload.validator_id,

            continuity_score:
                identity.continuity_score,

            trust_score:
                identity.trust_score,

            credential_depth:
                identity.credential_depth,

            created_epoch:
                identity.created_epoch,

            status:
                identity.status,
        }
    )
}
use axum::{
    extract::State,
    Json,
};

use serde::{
    Serialize,
    Deserialize,
};

use std::sync::{
    Arc,
    Mutex,
};

use crate::network_state::NetworkState;

// =========================
// 🌐 REGISTER REQUEST
// =========================
#[derive(
    Deserialize
)]
pub struct RegisterPeerRequest {

    pub peer_id: String,

    pub address: String,

    pub validator_id: u32,
}

// =========================
// 🌐 REGISTER RESPONSE
// =========================
#[derive(
    Serialize
)]
pub struct RegisterPeerResponse {

    pub success: bool,

    pub peer_count: usize,
}

// =========================
// 🌐 REGISTER PEER
// =========================
pub async fn register_peer(

    State(state):
        State<
            Arc<
                Mutex<NetworkState>
            >
        >,

    Json(payload):
        Json<RegisterPeerRequest>,

) -> Json<RegisterPeerResponse> {

    let mut state =
        state.lock().unwrap();

    state.register_peer(
        payload.peer_id,
        payload.address,
        payload.validator_id,
    );

    Json(
        RegisterPeerResponse {

            success: true,

            peer_count:
                state
                    .peer_state
                    .peers
                    .len(),
        }
    )
}
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

use fluxlock_core::types::{
    PeerNode,
};

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

    let current_epoch =
        state.global_epoch;

    state
        .peer_state
        .register_peer(

            PeerNode {

                peer_id:
                    payload.peer_id,

                address:
                    payload.address,

                validator_id:
                    payload.validator_id,

                last_seen_epoch:
                    current_epoch,

                trust_score: 100.0,

                active: true,
            }
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
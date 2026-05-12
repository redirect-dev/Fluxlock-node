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
    Validator,
    PeerAnnouncement,
};

// =========================
// 🌐 REMOTE PEER STATE
// =========================
#[derive(
    Clone,
    Serialize,
    Deserialize,
)]
pub struct RemotePeerState {

    pub validators:
        Vec<Validator>,

    pub gossip:
        Vec<PeerAnnouncement>,
}

// =========================
// 🌐 EXPORT PEER STATE
// =========================
pub async fn export_peer_state(

    State(state):
        State<
            Arc<
                Mutex<NetworkState>
            >
        >,
) -> Json<RemotePeerState> {

    let state =
        state.lock().unwrap();

    Json(

        RemotePeerState {

            validators:
                state
                    .validators
                    .clone(),

            gossip:
                state
                    .peer_state
                    .gossip
                    .announcements
                    .clone(),
        }
    )
}
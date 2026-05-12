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
    PeerAnnouncement,
};

use reqwest;

// =========================
// 📡 GOSSIP REQUEST
// =========================
#[derive(
    Deserialize,
    Serialize,
    Clone,
)]
pub struct GossipRequest {

    pub announcements:
        Vec<PeerAnnouncement>,
}

// =========================
// 📡 GOSSIP RESPONSE
// =========================
#[derive(
    Serialize
)]
pub struct GossipResponse {

    pub success: bool,

    pub received: usize,

    pub total_gossip: usize,
}

// =========================
// 📡 RECEIVE GOSSIP
// =========================
pub async fn receive_gossip(

    State(state):
        State<
            Arc<
                Mutex<NetworkState>
            >
        >,

    Json(payload):
        Json<GossipRequest>,

) -> Json<GossipResponse> {

    let mut state =
        state.lock().unwrap();

    for announcement in
        payload.announcements.clone()
    {

        state
            .peer_state
            .push_announcement(
                announcement
            );
    }

    Json(
        GossipResponse {

            success: true,

            received:
                payload
                    .announcements
                    .len(),

            total_gossip:
                state
                    .peer_state
                    .gossip
                    .announcements
                    .len(),
        }
    )
}

// =========================
// 🌐 OUTBOUND GOSSIP
// =========================
pub async fn propagate_gossip(

    peer_address: String,

    announcements:
        Vec<PeerAnnouncement>,
) {

    let client =
        reqwest::Client::new();

    let payload =
        GossipRequest {
            announcements,
        };

    let endpoint =
        format!(
            "{}/peer/gossip",
            peer_address
        );

    let _ =
        client
            .post(endpoint)
            .json(&payload)
            .send()
            .await;
}
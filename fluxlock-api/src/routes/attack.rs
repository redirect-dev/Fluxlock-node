use axum::{
    extract::State,
    Json,
};

use serde::{
    Deserialize,
};

use std::sync::{
    Arc,
    Mutex,
};

use crate::network_state::NetworkState;

// =========================
// ⚔ ATTACK REQUEST
// =========================
#[derive(
    Deserialize
)]
pub struct AttackRequest {

    pub id: u32,
}

// =========================
// ⚡ SPIKE ATTACK
// =========================
pub async fn spike(

    State(state):
        State<
            Arc<
                Mutex<NetworkState>
            >
        >,

    Json(payload):
        Json<AttackRequest>,

) -> &'static str {

    let mut state =
        state.lock().unwrap();

    state.spike_attack(
        payload.id
    );

    println!(
        "⚡ SPIKE ATTACK validator={}",
        payload.id
    );

    "ok"
}

// =========================
// ☠ BREACH ATTACK
// =========================
pub async fn breach(

    State(state):
        State<
            Arc<
                Mutex<NetworkState>
            >
        >,

    Json(payload):
        Json<AttackRequest>,

) -> &'static str {

    let mut state =
        state.lock().unwrap();

    state.breach_attack(
        payload.id
    );

    println!(
        "☠ BREACH ATTACK validator={}",
        payload.id
    );

    "ok"
}

// =========================
// 🌊 NETWORK ATTACK
// =========================
pub async fn network(

    State(state):
        State<
            Arc<
                Mutex<NetworkState>
            >
        >,

) -> &'static str {

    let mut state =
        state.lock().unwrap();

    state.network_attack();

    println!(
        "🌊 NETWORK-WIDE ATTACK"
    );

    "ok"
}

// =========================
// 🧬 FRACTURE ATTACK
// =========================
pub async fn fracture(

    State(state):
        State<
            Arc<
                Mutex<NetworkState>
            >
        >,

    Json(payload):
        Json<AttackRequest>,

) -> &'static str {

    let mut state =
        state.lock().unwrap();

    state.fracture_attack(
        payload.id
    );

    println!(
        "🧬 CONTINUITY FRACTURE validator={}",
        payload.id
    );

    "ok"
}
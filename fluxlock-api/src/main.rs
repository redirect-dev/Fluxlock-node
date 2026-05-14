mod state;
mod network_state;
mod peer_state;
mod identity;
mod engine;
mod routes;

use axum::{
    routing::{get, post},
    Router,
};

use std::sync::{Arc, Mutex};

use routes::sign::sign;
use routes::verify::verify;
use routes::validate::validate_identity;

use routes::attack::{
    spike,
    breach,
    network,
};

use routes::evaluate::evaluate;

use routes::access::access;

use routes::auth::auth_flow;

use routes::identity_create::create_identity;

use routes::continuity::{
    export_continuity_proof,
};

use routes::peer_register::register_peer;

use routes::peer_gossip::{
    receive_gossip,
    propagate_gossip,
};

use routes::peer_state::{
    export_peer_state,
};

use engine::peer_sync::{
    synchronize_peers,
};

use network_state::NetworkState;

// 🌐 CORS
use tower_http::cors::{
    Any,
    CorsLayer,
};

use axum::http::Method;

// 🔥 STORAGE
use fluxlock_storage::db::init_db;

use fluxlock_storage::schema::init_schema;

#[tokio::main]
async fn main() {

    // =========================
    // 🧠 STORAGE INIT
    // =========================
    init_db();

    init_schema();

    println!(
        "🧠 Fluxlock persistence initialized"
    );

    // =========================
    // 🌐 GLOBAL STATE
    // =========================
    let state = Arc::new(
        Mutex::new(
            NetworkState::new()
        )
    );

    // =========================
    // 🔁 ENGINE LOOP
    // =========================
    let state_clone =
        state.clone();

    tokio::spawn(async move {

        loop {

            {
                let mut s =
                    state_clone
                        .lock()
                        .unwrap();

                s.tick();
            }

            tokio::time::sleep(
                std::time::Duration::from_millis(300)
            )
            .await;
        }
    });

    // =========================
    // 📡 GOSSIP PROPAGATION
    // =========================
    let propagation_state =
        state.clone();

    tokio::spawn(async move {

        loop {

            let (
                peers,
                announcements,
            ) = {

                let s =
                    propagation_state
                        .lock()
                        .unwrap();

                (
                    s.peer_state
                        .peers
                        .values()
                        .cloned()
                        .collect::<Vec<_>>(),

                    s.peer_state
                        .gossip
                        .announcements
                        .clone(),
                )
            };

            for peer in peers {

                if peer.active {

                    propagate_gossip(
                        peer.address.clone(),
                        announcements.clone(),
                    )
                    .await;
                }
            }

            tokio::time::sleep(
                std::time::Duration::from_secs(5)
            )
            .await;
        }
    });

    // =========================
    // 🌐 PEER SYNC
    // =========================
    let sync_state =
        state.clone();

    tokio::spawn(async move {

        synchronize_peers(
            sync_state
        )
        .await;
    });

    // =========================
    // 🌐 CORS
    // =========================
    let cors = CorsLayer::new()

        .allow_origin(Any)

        .allow_methods([
            Method::GET,
            Method::POST,
        ])

        .allow_headers(Any);

    // =========================
    // 🚀 ROUTER
    // =========================
    let app = Router::new()

        // 🔐 CRYPTO
        .route(
            "/sign",
            post(sign)
        )

        .route(
            "/verify",
            post(verify)
        )

        // 🧠 VALIDATION
        .route(
            "/validate",
            post(validate_identity)
        )

        // 🌐 NETWORK STATE
        .route(
            "/state",
            get(get_state)
        )

        // 🌐 PEER NETWORK
        .route(
            "/peer/register",
            post(register_peer)
        )

        .route(
            "/peer/gossip",
            post(receive_gossip)
        )

        .route(
            "/peer/state",
            get(export_peer_state)
        )

        // 🧬 CONTINUITY
        .route(
            "/continuity/:validator_id",
            get(export_continuity_proof)
        )

        // 🔥 IDENTITY
        .route(
            "/identity/create",
            post(create_identity)
        )

        // 🔥 CORE PRODUCT
        .route(
            "/evaluate",
            post(evaluate)
        )

        .route(
            "/access",
            post(access)
        )

        .route(
            "/auth/flow",
            post(auth_flow)
        )

        // ⚔ ATTACKS
        .route(
            "/attack/spike",
            post(spike)
        )

        .route(
            "/attack/breach",
            post(breach)
        )

        .route(
            "/attack/network",
            post(network)
        )

        .layer(cors)

        .with_state(
            state.clone()
        );

    // =========================
    // 🌐 SERVER START
    // =========================
    let listener =
        tokio::net::TcpListener::bind(
            "127.0.0.1:3001"
        )
        .await
        .unwrap();

    println!(
        "🚀 Fluxlock API running on http://127.0.0.1:3001"
    );

    println!(
        "🌐 Distributed peer synchronization enabled"
    );

    println!(
        "🧬 Continuity proof export enabled"
    );

    axum::serve(
        listener,
        app
    )
    .await
    .unwrap();
}

// =========================
// 🌐 STATE ENDPOINT
// =========================
use axum::{
    extract::State,
    Json,
};

async fn get_state(

    State(state):
        State<
            Arc<
                Mutex<NetworkState>
            >
        >,

) -> Json<NetworkState> {

    let state =
        state.lock()
            .unwrap();

    Json(
        state.clone()
    )
}
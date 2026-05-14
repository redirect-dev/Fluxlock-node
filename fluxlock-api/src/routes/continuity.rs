use axum::{
    extract::{
        Path,
        State,
    },
    Json,
};

use serde::{
    Serialize,
};

use std::sync::{
    Arc,
    Mutex,
};

use crate::network_state::NetworkState;

use fluxlock_core::types::{
    IdentityLink,
};

// =========================
// 🧬 CONTINUITY PROOF
// =========================
#[derive(
    Clone,
    Serialize,
)]
pub struct ContinuityProof {

    pub validator_id: u32,

    pub chain_depth: usize,

    pub valid: bool,

    pub latest_hash: String,

    pub genesis_hash: String,

    pub epochs: Vec<u64>,

    pub lineage: Vec<IdentityLink>,
}

// =========================
// 🧬 EXPORT PROOF
// =========================
pub async fn export_continuity_proof(

    Path(validator_id):
        Path<u32>,

    State(state):
        State<
            Arc<
                Mutex<NetworkState>
            >
        >,

) -> Json<ContinuityProof> {

    let state =
        state.lock().unwrap();

    let validator =
        state.validators
            .iter()
            .find(|v|
                v.id == validator_id
            );

    match validator {

        Some(v) => {

            let latest_hash =
                v.identity_chain
                    .last()
                    .map(|l|
                        l.continuity_hash.clone()
                    )
                    .unwrap_or_default();

            let genesis_hash =
                v.identity_chain
                    .first()
                    .map(|l|
                        l.continuity_hash.clone()
                    )
                    .unwrap_or_default();

            let epochs =
                v.identity_chain
                    .iter()
                    .map(|l|
                        l.epoch
                    )
                    .collect::<Vec<_>>();

            Json(

                ContinuityProof {

                    validator_id,

                    chain_depth:
                        v.identity_chain.len(),

                    valid:
                        v.chain_valid,

                    latest_hash,

                    genesis_hash,

                    epochs,

                    lineage:
                        v.identity_chain.clone(),
                }
            )
        }

        None => {

            Json(

                ContinuityProof {

                    validator_id,

                    chain_depth: 0,

                    valid: false,

                    latest_hash:
                        String::new(),

                    genesis_hash:
                        String::new(),

                    epochs:
                        Vec::new(),

                    lineage:
                        Vec::new(),
                }
            )
        }
    }
}
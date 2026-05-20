use axum::{
    extract::{
        Path,
        State,
    },
    Json,
};

use std::sync::{
    Arc,
    Mutex,
};

use serde::{
    Serialize,
};

use pqcrypto_dilithium::dilithium2;

use crate::{
    continuity_proof::{
        build_continuity_proof,
        verify_proof,
        ContinuityProof,
    },
    network_state::NetworkState,
    state::KEY_STORE,
};

// =========================
// 📤 RESPONSE
// =========================
#[derive(
    Serialize
)]
pub struct ProofResponse {

    pub success: bool,

    pub validator_id: u32,

    pub valid: bool,

    pub proof:
        Option<ContinuityProof>,
}

// =========================
// 🌐 GET VALIDATOR PROOF
// =========================
pub async fn get_proof(

    Path(id): Path<u32>,

    State(state):
        State<
            Arc<
                Mutex<NetworkState>
            >
        >,
) -> Json<ProofResponse> {

    let state =
        state
            .lock()
            .unwrap();

    let validator =
        match state
            .validators
            .iter()
            .find(|v| v.id == id)
        {

            Some(v) => v,

            None => {

                return Json(
                    ProofResponse {

                        success: false,

                        validator_id: id,

                        valid: false,

                        proof: None,
                    }
                );
            }
        };

    // =========================
    // 🔐 FETCH SIGNING KEY
    // =========================
    let store =
        KEY_STORE
            .lock()
            .unwrap();

    let (_, sk) =
        match store.get(&id)
    {

        Some(pair) => pair,

        None => {

            return Json(
                ProofResponse {

                    success: false,

                    validator_id: id,

                    valid: false,

                    proof: None,
                }
            );
        }
    };

    // =========================
    // 🧬 BUILD PROOF
    // =========================
    let proof =
        build_continuity_proof(
            validator,
            sk,
        );

    let valid =
        proof
            .as_ref()
            .map(verify_proof)
            .unwrap_or(false);

    Json(
        ProofResponse {

            success: proof.is_some(),

            validator_id: id,

            valid,

            proof,
        }
    )
}

// =========================
// 🌐 GET ALL PROOFS
// =========================
pub async fn get_all_proofs(

    State(state):
        State<
            Arc<
                Mutex<NetworkState>
            >
        >,
) -> Json<Vec<ContinuityProof>> {

    let state =
        state
            .lock()
            .unwrap();

    let store =
        KEY_STORE
            .lock()
            .unwrap();

    let mut proofs =
        Vec::new();

    for validator in
        &state.validators
    {

        let (_, sk) =
            match store.get(&validator.id)
        {

            Some(pair) => pair,

            None => continue,
        };

        if let Some(proof) =
            build_continuity_proof(
                validator,
                sk,
            )
        {

            proofs.push(
                proof
            );
        }
    }

    Json(proofs)
}
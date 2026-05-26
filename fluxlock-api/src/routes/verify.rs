use axum::{
    Json,
    extract::Json as ExtractJson,
};

use serde::{
    Deserialize,
    Serialize,
};

use base64::{
    engine::general_purpose,
    Engine as _,
};

use pqcrypto_dilithium::dilithium2;
use pqcrypto_traits::sign::SignedMessage;

use crate::state::{
    KEY_STORE,
};

// =========================
// 🔐 REQUEST
// =========================
#[derive(Deserialize)]
pub struct VerifyRequest {

    pub message: String,

    pub signature: String,

    pub validator_id: u32,
}

// =========================
// 🔐 RESPONSE
// =========================
#[derive(Serialize)]
pub struct VerifyResponse {

    pub signature_valid: bool,

    pub identity_valid: bool,

    pub continuity_verified: bool,

    pub governance_verified: bool,

    pub lineage_verified: bool,
}

// =========================
// 🔐 VERIFY
// =========================
pub async fn verify(

    ExtractJson(payload):
        ExtractJson<VerifyRequest>,
) -> Json<VerifyResponse> {

    // =========================
    // 🔐 FETCH KEYS
    // =========================
    let store =
        KEY_STORE.lock().unwrap();

    let (pk, _) =
        match store.get(
            &payload.validator_id
        ) {

            Some(pair) => pair,

            None => {

                return Json(
                    VerifyResponse {

                        signature_valid: false,

                        identity_valid: false,

                        continuity_verified: false,

                        governance_verified: false,

                        lineage_verified: false,
                    }
                );
            }
        };

    // =========================
// 🧠 PLACEHOLDER
// =========================
let continuity_verified = true;

let governance_verified = true;

let lineage_verified = true;

    // =========================
    // 🔐 DECODE SIGNATURE
    // =========================
    let decoded =
        match general_purpose
            ::STANDARD
            .decode(
                payload.signature
            ) {

            Ok(bytes) => bytes,

            Err(_) => {

                return Json(
                    VerifyResponse {

                        signature_valid: false,

                        identity_valid: false,

                        continuity_verified: false,

                        governance_verified: false,

                        lineage_verified: false,
                    }
                );
            }
        };

    let signed_msg =
        match dilithium2
            ::SignedMessage
            ::from_bytes(
                &decoded
            ) {

            Ok(msg) => msg,

            Err(_) => {

                return Json(
                    VerifyResponse {

                        signature_valid: false,

                        identity_valid: false,

                        continuity_verified: false,

                        governance_verified: false,

                        lineage_verified: false,
                    }
                );
            }
        };

    // =========================
    // 🔐 VERIFY
    // =========================
    let result =
        dilithium2::open(
            &signed_msg,
            pk
        );

    let signature_valid =
        result.is_ok();

    // =========================
    // 🔐 FINAL
    // =========================
    let identity_valid =
        signature_valid
        &&
        continuity_verified
        &&
        governance_verified
        &&
        lineage_verified;

    Json(
        VerifyResponse {

            signature_valid,

            identity_valid,

            continuity_verified,

            governance_verified,

            lineage_verified,
        }
    )
}
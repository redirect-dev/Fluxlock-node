use fluxlock_core::{
    EngineCompositeState,
    TickInput,
};

use fluxlock_core::keystate::{
    should_rotate,
    rotate_key,
    validate_transition,
};

use blake3;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use std::convert::TryInto;

//
// ============================================================
// Deterministic State Hash
// ============================================================
//
pub fn hash_state(state: &EngineCompositeState) -> String {

    let mut hasher = blake3::Hasher::new();

    hasher.update(&state.trust.trust_score.to_le_bytes());
    hasher.update(&[state.lifecycle.stage]);
    hasher.update(&[state.lock.level]);
    hasher.update(&[state.recovery.is_recovering as u8]);
    hasher.update(&state.recovery.recovery_ticks.to_le_bytes());
    hasher.update(&state.recovery.grace_ticks_remaining.to_le_bytes());

    hasher.update(&state.key_state.key_epoch.to_le_bytes());
    hasher.update(&state.key_state.activated_at_tick.to_le_bytes());
    hasher.update(&state.key_state.current_pubkey);

    if let Some(commitment) = &state.key_state.next_pubkey_commitment {
        hasher.update(commitment);
    }

    hasher.finalize().to_hex().to_string()
}

//
// ============================================================
// Apply Tick
// ============================================================
//
pub fn apply_tick(
    state: &mut EngineCompositeState,
    input: &TickInput,
    tick_index: u64,
) -> Result<(), String> {

    let previous = state.clone();

    if let Some(revealed) = &input.revealed_pubkey {

        // -----------------------------------------
        // GENESIS INITIALIZATION
        // -----------------------------------------
        if state.key_state.current_pubkey.is_empty() {

            state.key_state.current_pubkey = revealed.clone();
            state.key_state.activated_at_tick = tick_index;
            state.key_state.key_epoch = 0;

        } else {

            // -------------------------------------
            // NORMAL ROTATION
            // -------------------------------------
            if let Some(signature_bytes) = &input.signature {

                let mut message = Vec::new();
                message.extend(revealed);
                message.extend(tick_index.to_le_bytes());

                let pubkey_array: [u8; 32] =
                    state.key_state.current_pubkey
                        .clone()
                        .try_into()
                        .map_err(|_| "Invalid public key length")?;

                let verifying_key =
                    VerifyingKey::from_bytes(&pubkey_array)
                        .map_err(|_| "Invalid public key")?;

                let sig_array: [u8; 64] =
                    signature_bytes
                        .clone()
                        .try_into()
                        .map_err(|_| "Invalid signature length")?;

                let signature = Signature::from_bytes(&sig_array);

                verifying_key
                    .verify(&message, &signature)
                    .map_err(|_| "Signature verification failed")?;
            }

            if should_rotate(&state.key_state, tick_index) {

                let rotated =
                    rotate_key(&state.key_state, revealed.clone(), tick_index)?;

                state.key_state = rotated;
            }
        }
    }

    validate_transition(&previous.key_state, &state.key_state, tick_index)
        .map_err(|_| "KeyState invariant violation".to_string())?;

    if state.trust.trust_score <= 0.0 {

        let preserved_lock = state.lock.level;

        *state = EngineCompositeState::new();

        state.lock.level = preserved_lock;
        state.trust.trust_score = 25.0;
        state.recovery.grace_ticks_remaining = 5;
    }

    Ok(())
}

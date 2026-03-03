use blake3::Hasher;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};

use fluxlock_core::{EngineCompositeState, TickInput};

/// Deterministic state hashing
pub fn hash_state(state: &EngineCompositeState) -> String {
    let mut hasher = Hasher::new();

    if let Some(pk) = &state.key_state.current_pubkey {
        hasher.update(pk);
    }

    if let Some(commit) = &state.key_state.pending_commitment {
        hasher.update(commit);
    }

    hex::encode(hasher.finalize().as_bytes())
}

/// Deterministic tick transition
pub fn apply_tick(
    state: &mut EngineCompositeState,
    input: &TickInput,
    tick_index: u64,
) -> Result<(), String> {

    // -----------------------------------------
    // COMMIT PHASE
    // -----------------------------------------
    if let Some(commit) = &input.commit_pubkey {
        state.key_state.pending_commitment = Some(commit.clone());
    }

    // -----------------------------------------
    // REVEAL PHASE
    // -----------------------------------------
    if let Some(reveal) = &input.reveal_pubkey {

        // Genesis reveal
        if state.key_state.current_pubkey.is_none() {
            state.key_state.current_pubkey = Some(reveal.clone());
            return Ok(());
        }

        // Must have commitment
        let pending = state.key_state
            .pending_commitment
            .as_ref()
            .ok_or("Missing commitment")?;

        if pending != reveal {
            return Err("Commitment mismatch".into());
        }

        let sig_bytes = input.signature
            .as_ref()
            .ok_or("Missing signature")?;

        if sig_bytes.len() != 64 {
            return Err("Invalid signature length".into());
        }

        let pk_bytes = state.key_state
            .current_pubkey
            .as_ref()
            .ok_or("Missing current pubkey")?;

        if pk_bytes.len() != 32 {
            return Err("Invalid public key length".into());
        }

        // Convert to fixed-size arrays safely
        let pk_array: [u8; 32] = pk_bytes
            .as_slice()
            .try_into()
            .map_err(|_| "Key conversion failed")?;

        let sig_array: [u8; 64] = sig_bytes
            .as_slice()
            .try_into()
            .map_err(|_| "Sig conversion failed")?;

        let verifying_key =
            VerifyingKey::from_bytes(&pk_array)
                .map_err(|_| "Invalid verifying key")?;

        let signature = Signature::from_bytes(&sig_array);

        let mut message = Vec::new();
        message.extend(reveal);
        message.extend(tick_index.to_le_bytes());

        verifying_key
            .verify(&message, &signature)
            .map_err(|_| "Signature verification failed")?;

        state.key_state.current_pubkey = Some(reveal.clone());
        state.key_state.pending_commitment = None;
    }

    Ok(())
}

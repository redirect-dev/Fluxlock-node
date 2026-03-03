use blake3::Hasher;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};

use fluxlock_core::{EngineCompositeState, TickInput};

const MIN_DELAY: u64 = 2;
const MAX_DELAY: u64 = 6;

pub fn hash_state(state: &EngineCompositeState) -> String {
    let mut hasher = Hasher::new();

    if let Some(pk) = &state.key_state.current_pubkey {
        hasher.update(pk);
    }

    if let Some(commit) = &state.key_state.pending_commitment {
        hasher.update(commit);
    }

    if let Some(tick) = state.key_state.commitment_tick {
        hasher.update(&tick.to_le_bytes());
    }

    hex::encode(hasher.finalize().as_bytes())
}

pub fn apply_tick(
    state: &mut EngineCompositeState,
    input: &TickInput,
    tick_index: u64,
) -> Result<(), String> {

    // --------------------------------------------------
    // EXPIRE COMMITMENT IF TOO OLD
    // --------------------------------------------------
    if let (Some(commit_tick), Some(_)) =
        (state.key_state.commitment_tick, &state.key_state.pending_commitment)
    {
        if tick_index > commit_tick + MAX_DELAY {
            state.key_state.pending_commitment = None;
            state.key_state.commitment_tick = None;
        }
    }

    // --------------------------------------------------
    // COMMIT
    // --------------------------------------------------
    if let Some(commit) = &input.commit_pubkey {
        state.key_state.pending_commitment = Some(commit.clone());
        state.key_state.commitment_tick = Some(tick_index);
    }

    // --------------------------------------------------
    // REVEAL
    // --------------------------------------------------
    if let Some(reveal) = &input.reveal_pubkey {

        if state.key_state.current_pubkey.is_none() {
            state.key_state.current_pubkey = Some(reveal.clone());
            return Ok(());
        }

        let pending = state.key_state
            .pending_commitment
            .as_ref()
            .ok_or("Missing commitment")?;

        let commit_tick = state.key_state
            .commitment_tick
            .ok_or("Missing commitment tick")?;

        if tick_index < commit_tick + MIN_DELAY {
            return Err("Reveal too early".into());
        }

        if tick_index > commit_tick + MAX_DELAY {
            return Err("Reveal after expiry window".into());
        }

        if pending != reveal {
            return Err("Commitment mismatch".into());
        }

        let sig_bytes = input.signature
            .as_ref()
            .ok_or("Missing signature")?;

        let pk_bytes = state.key_state
            .current_pubkey
            .as_ref()
            .ok_or("Missing current pubkey")?;

        let pk_array: [u8; 32] =
            pk_bytes.as_slice().try_into()
                .map_err(|_| "Key conversion failed")?;

        let sig_array: [u8; 64] =
            sig_bytes.as_slice().try_into()
                .map_err(|_| "Sig conversion failed")?;

        let verifying_key =
            VerifyingKey::from_bytes(&pk_array)
                .map_err(|_| "Invalid verifying key")?;

        let signature =
            Signature::from_bytes(&sig_array);

        let mut message = Vec::new();
        message.extend(reveal);
        message.extend(tick_index.to_le_bytes());

        verifying_key
            .verify(&message, &signature)
            .map_err(|_| "Signature verification failed")?;

        state.key_state.current_pubkey = Some(reveal.clone());
        state.key_state.pending_commitment = None;
        state.key_state.commitment_tick = None;
    }

    Ok(())
}

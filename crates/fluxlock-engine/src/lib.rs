use blake3::Hasher;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};

use fluxlock_core::{EngineCompositeState, TickInput};

const MIN_DELAY: u64 = 2;
const MAX_DELAY: u64 = 6;

pub fn hash_state(state: &EngineCompositeState) -> String {
    let mut hasher = Hasher::new();

    if let Some(pk) = &state.key_state.current_classical_pubkey {
        hasher.update(pk);
    }

    if let Some(pk) = &state.key_state.current_pq_pubkey {
        hasher.update(pk);
    }

    hex::encode(hasher.finalize().as_bytes())
}

pub fn apply_tick(
    state: &mut EngineCompositeState,
    input: &TickInput,
    tick_index: u64,
) -> Result<(), String> {

    // Expiry
    if let Some(commit_tick) = state.key_state.commitment_tick {
        if tick_index > commit_tick + MAX_DELAY {
            state.key_state.pending_classical_commitment = None;
            state.key_state.pending_pq_commitment = None;
            state.key_state.commitment_tick = None;
        }
    }

    // Commit
    if input.commit_classical.is_some() && input.commit_pq.is_some() {
        state.key_state.pending_classical_commitment =
            input.commit_classical.clone();
        state.key_state.pending_pq_commitment =
            input.commit_pq.clone();
        state.key_state.commitment_tick = Some(tick_index);
    }

    // Reveal
    if input.reveal_classical.is_some() && input.reveal_pq.is_some() {

        if state.key_state.current_classical_pubkey.is_none() {
            state.key_state.current_classical_pubkey =
                input.reveal_classical.clone();
            state.key_state.current_pq_pubkey =
                input.reveal_pq.clone();
            return Ok(());
        }

        let commit_tick =
            state.key_state.commitment_tick.ok_or("Missing commitment tick")?;

        if tick_index < commit_tick + MIN_DELAY {
            return Err("Reveal too early".into());
        }

        if tick_index > commit_tick + MAX_DELAY {
            return Err("Reveal expired".into());
        }

        // Verify commitments match
        if state.key_state.pending_classical_commitment
            != input.reveal_classical
        {
            return Err("Classical commitment mismatch".into());
        }

        if state.key_state.pending_pq_commitment
            != input.reveal_pq
        {
            return Err("PQ commitment mismatch".into());
        }

        // Verify classical signature
        verify_sig(
            state.key_state.current_classical_pubkey.as_ref().unwrap(),
            input.reveal_classical.as_ref().unwrap(),
            input.classical_signature.as_ref().unwrap(),
            tick_index,
        )?;

        // Verify PQ signature (simulated with ed25519)
        verify_sig(
            state.key_state.current_pq_pubkey.as_ref().unwrap(),
            input.reveal_pq.as_ref().unwrap(),
            input.pq_signature.as_ref().unwrap(),
            tick_index,
        )?;

        state.key_state.current_classical_pubkey =
            input.reveal_classical.clone();
        state.key_state.current_pq_pubkey =
            input.reveal_pq.clone();

        state.key_state.pending_classical_commitment = None;
        state.key_state.pending_pq_commitment = None;
        state.key_state.commitment_tick = None;
    }

    Ok(())
}

fn verify_sig(
    pubkey: &Vec<u8>,
    reveal: &Vec<u8>,
    sig: &Vec<u8>,
    tick_index: u64,
) -> Result<(), String> {

    let pk_array: [u8; 32] =
        pubkey.as_slice().try_into()
            .map_err(|_| "Key conversion failed")?;

    let sig_array: [u8; 64] =
        sig.as_slice().try_into()
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

    Ok(())
}

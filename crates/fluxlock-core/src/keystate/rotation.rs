use sha2::{Digest, Sha256};

use super::{KeyState, KeyStateError};

pub fn should_rotate(state: &KeyState, current_tick: u64) -> bool {
    let epoch_elapsed =
        current_tick.saturating_sub(state.activated_at_tick) >= state.rotation_policy.epoch_length;

    epoch_elapsed || state.rotation_override
}

pub fn rotate_key(
    prev: &KeyState,
    revealed_pubkey: Vec<u8>,
    current_tick: u64,
) -> Result<KeyState, KeyStateError> {
    if !should_rotate(prev, current_tick) {
        return Err(KeyStateError::RotationNotAllowed);
    }

    let commitment = match &prev.next_pubkey_commitment {
        Some(c) => c,
        None => return Err(KeyStateError::MissingCommitment),
    };

    let mut hasher = Sha256::new();
    hasher.update((revealed_pubkey.len() as u64).to_le_bytes());
    hasher.update(&revealed_pubkey);
    let calculated = hasher.finalize();

    if commitment.as_slice() != calculated.as_slice() {
        return Err(KeyStateError::CommitmentMismatch);
    }

    Ok(KeyState {
        key_epoch: prev.key_epoch + 1,
        activated_at_tick: current_tick,
        algorithm: prev.algorithm.clone(),
        current_pubkey: revealed_pubkey,
        next_pubkey_commitment: None,
        rotation_policy: prev.rotation_policy.clone(),
        rotation_override: false,
        not_before_tick: current_tick,
        not_after_tick: None,
        parent_key_hash: Some(prev.hash().to_vec()),
    })
}

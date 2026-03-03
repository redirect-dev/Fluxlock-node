use super::{AlgorithmId, KeyState};

#[derive(Debug)]
pub enum KeyStateError {
    EpochRegression,
    AlgorithmDowngrade,
    ParentHashMissing,
    ParentHashMismatch,
    ActivationBeforeAllowedTick,
    KeyExpired,
    CommitmentMutated,
    RotationNotAllowed,
    MissingCommitment,
    CommitmentMismatch,
}

pub fn validate_transition(
    prev: &KeyState,
    next: &KeyState,
    current_tick: u64,
) -> Result<(), KeyStateError> {
    if next.key_epoch < prev.key_epoch {
        return Err(KeyStateError::EpochRegression);
    }

    if algorithm_rank(&next.algorithm) < algorithm_rank(&prev.algorithm) {
        return Err(KeyStateError::AlgorithmDowngrade);
    }

    let prev_hash = prev.hash();
    match &next.parent_key_hash {
        Some(parent) if parent.as_slice() == prev_hash => {}
        Some(_) => return Err(KeyStateError::ParentHashMismatch),
        None => return Err(KeyStateError::ParentHashMissing),
    }

    if current_tick < next.not_before_tick {
        return Err(KeyStateError::ActivationBeforeAllowedTick);
    }

    if let Some(expiry) = next.not_after_tick {
        if current_tick > expiry {
            return Err(KeyStateError::KeyExpired);
        }
    }

    if next.key_epoch == prev.key_epoch {
        if next.next_pubkey_commitment != prev.next_pubkey_commitment {
            return Err(KeyStateError::CommitmentMutated);
        }
    }

    Ok(())
}

fn algorithm_rank(algo: &AlgorithmId) -> u8 {
    match algo {
        AlgorithmId::Ed25519 => 0,
        AlgorithmId::Secp256k1 => 1,
        AlgorithmId::MlDsa => 2,
        AlgorithmId::Hybrid => 3,
    }
}

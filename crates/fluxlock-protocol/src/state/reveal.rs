use blake3;

use crate::state::account::{Account, FLAG_IDENTITY_EXPIRED};
use crate::tx::transaction::RotationRevealTx;
use crate::pq;
use crate::state::validator::Validator;
use crate::state::event::Event;

/// Apply rotation reveal (EVENTS ALWAYS RETURNED)
pub fn apply_rotation_reveal(
    accounts: &mut Vec<Account>,
    validator: &mut Validator,
    tx: &RotationRevealTx,
) -> (Vec<Event>, Result<(), String>) {
    let mut events = Vec::new();

    let acc = match accounts
        .iter_mut()
        .find(|a| a.current_classical_pubkey == tx.from)
    {
        Some(a) => a,
        None => {
            return (events, Err("Account not found".into()));
        }
    };

    // NONCE
    if tx.nonce != acc.nonce {
        validator.slash(5);
        events.push(Event::InvalidNonce { identity: tx.from.clone() });
        events.push(Event::ValidatorSlashed { amount: 5 });
        return (events, Err("Invalid nonce".into()));
    }

    acc.nonce += 1;

    // FORK
    if tx.epoch <= acc.rotation_epoch {
        validator.slash(20);
        events.push(Event::ForkDetected {
            identity: tx.from.clone(),
            epoch: tx.epoch,
        });
        events.push(Event::ValidatorSlashed { amount: 20 });
        return (events, Err("FORK_DETECTED".into()));
    }

    // EXPIRATION
    if acc.has_flag(FLAG_IDENTITY_EXPIRED) {
        validator.slash(10);
        events.push(Event::IdentityExpired { identity: tx.from.clone() });
        events.push(Event::ValidatorSlashed { amount: 10 });
        return (events, Err("IDENTITY_EXPIRED".into()));
    }

    // CONTINUITY
    let continuity_valid = pq::verify(
        &tx.new_pq_key,
        &tx.link_signature,
        &acc.current_pq_pubkey,
    );

    if !continuity_valid {
        validator.slash(15);
        events.push(Event::InvalidContinuity { identity: tx.from.clone() });
        events.push(Event::ValidatorSlashed { amount: 15 });
        return (events, Err("INVALID_LINK_SIGNATURE".into()));
    }

    // COMMIT
    let commitment = match acc.rotation_commitment.clone() {
        Some(c) => c,
        None => return (events, Err("No commit found".into())),
    };

    let mut hasher = blake3::Hasher::new();
    hasher.update(&tx.new_classical_key);
    hasher.update(&tx.new_pq_key);

    let calculated = hasher.finalize().as_bytes().to_vec();

    if calculated != commitment {
        validator.slash(10);
        events.push(Event::CommitmentMismatch { identity: tx.from.clone() });
        events.push(Event::ValidatorSlashed { amount: 10 });
        return (events, Err("Commitment mismatch".into()));
    }

    // SUCCESS
    acc.current_classical_pubkey = tx.new_classical_key.clone();
    acc.current_pq_pubkey = tx.new_pq_key.clone();
    acc.rotation_epoch = tx.epoch;

    acc.rotation_commitment = None;
    acc.rotation_deadline_tick = None;

    acc.clear_flag(FLAG_IDENTITY_EXPIRED);

    events.push(Event::RotationSuccess {
        identity: tx.from.clone(),
        epoch: tx.epoch,
    });

    (events, Ok(()))
}
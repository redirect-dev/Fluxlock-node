use std::collections::HashMap;

use crate::tx::transaction::RotationRevealTx;
use crate::pq;
use crate::epoch::EpochProvider;

#[derive(Clone, Debug)]
pub struct IdentityState {
    pub current_classical_key: Vec<u8>,
    pub current_pq_key: Vec<u8>,
    pub epoch: u64,
}

#[derive(Default)]
pub struct NetworkState {
    pub identities: HashMap<Vec<u8>, IdentityState>,
}

/// Tracks rotations per epoch to prevent forks
#[derive(Default)]
pub struct RotationTracker {
    pub seen: HashMap<(Vec<u8>, u64), bool>, // (identity, epoch)
}

pub struct Validator<E: EpochProvider> {
    pub state: NetworkState,
    pub epoch_provider: E,
    pub rotation_tracker: RotationTracker,
}

impl<E: EpochProvider> Validator<E> {
    pub fn new(epoch_provider: E) -> Self {
        Self {
            state: NetworkState::default(),
            epoch_provider,
            rotation_tracker: RotationTracker::default(),
        }
    }

    /// Apply a single transaction
    pub fn apply_tx(&mut self, tx: RotationRevealTx) -> Result<(), String> {
        let current_epoch = self.epoch_provider.current_epoch().number;

        let node_id = tx.from.clone();

        // -----------------------------
        // RULE 1 — TIME VALIDITY
        // -----------------------------
        if tx.epoch != current_epoch {
            return Err("INVALID_EPOCH".into());
        }

        // -----------------------------
        // RULE 4 — FORK PREVENTION
        // -----------------------------
        let key = (node_id.clone(), tx.epoch);
        if self.rotation_tracker.seen.contains_key(&key) {
            return Err("FORK_DETECTED_SLASH".into());
        }

        // -----------------------------
        // LOAD EXISTING STATE
        // -----------------------------
        let prev_state = self.state.identities.get(&node_id);

        // -----------------------------
        // RULE 3 — CONTINUITY
        // -----------------------------
        if let Some(prev) = prev_state {
            // message = new PQ key (can be extended later)
            let message = &tx.new_pq_key;

            let valid_link = pq::verify(
                message,
                &tx.link_signature,
                &prev.current_pq_key,
            );

            if !valid_link {
                return Err("INVALID_LINK_SIGNATURE".into());
            }

            // -----------------------------
            // RULE 2 — EXPIRATION
            // -----------------------------
            if prev.epoch < current_epoch {
                return Err("IDENTITY_EXPIRED".into());
            }
        }

        // -----------------------------
        // VERIFY NEW KEY OWNERSHIP (PQ)
        // -----------------------------
        let valid_new_key = pq::verify(
            &tx.new_pq_key,
            &tx.pq_signature,
            &tx.new_pq_key,
        );

        if !valid_new_key {
            return Err("INVALID_NEW_KEY_SIGNATURE".into());
        }

        // -----------------------------
        // APPLY STATE UPDATE
        // -----------------------------
        let new_state = IdentityState {
            current_classical_key: tx.new_classical_key.clone(),
            current_pq_key: tx.new_pq_key.clone(),
            epoch: tx.epoch,
        };

        self.state.identities.insert(node_id.clone(), new_state);

        // mark rotation as seen
        self.rotation_tracker.seen.insert(key, true);

        Ok(())
    }

    /// Apply transactions deterministically
    pub fn apply_block(&mut self, mut txs: Vec<RotationRevealTx>) -> Result<(), String> {
        // -----------------------------
        // SORT (DETERMINISTIC ORDER)
        // -----------------------------
        txs.sort_by(|a, b| {
            a.epoch
                .cmp(&b.epoch)
                .then(a.timestamp.cmp(&b.timestamp))
        });

        // -----------------------------
        // APPLY IN ORDER
        // -----------------------------
        for tx in txs {
            self.apply_tx(tx)?;
        }

        Ok(())
    }
}
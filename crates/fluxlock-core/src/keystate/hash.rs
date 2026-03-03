use sha2::{Digest, Sha256};

use super::{AlgorithmId, KeyState};

impl KeyState {
    /// Deterministic SHA256 hash of KeyState.
    /// This MUST remain stable across compiler versions and platforms.
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();

        // --- key_epoch (u64 LE)
        hasher.update(self.key_epoch.to_le_bytes());

        // --- activated_at_tick (u64 LE)
        hasher.update(self.activated_at_tick.to_le_bytes());

        // --- algorithm (1 byte fixed mapping)
        hasher.update([algorithm_to_u8(&self.algorithm)]);

        // --- current_pubkey (length + bytes)
        hash_vec(&mut hasher, &self.current_pubkey);

        // --- next_pubkey_commitment (option)
        hash_option_vec(&mut hasher, &self.next_pubkey_commitment);

        // --- rotation_policy.epoch_length
        hasher.update(self.rotation_policy.epoch_length.to_le_bytes());

        // --- not_before_tick
        hasher.update(self.not_before_tick.to_le_bytes());

        // --- not_after_tick (option)
        hash_option_u64(&mut hasher, &self.not_after_tick);

        // --- parent_key_hash (option)
        hash_option_vec(&mut hasher, &self.parent_key_hash);

        hasher.finalize().into()
    }
}

fn algorithm_to_u8(algo: &AlgorithmId) -> u8 {
    match algo {
        AlgorithmId::Ed25519 => 0,
        AlgorithmId::Secp256k1 => 1,
        AlgorithmId::MlDsa => 2,
        AlgorithmId::Hybrid => 3,
    }
}

fn hash_vec(hasher: &mut Sha256, data: &Vec<u8>) {
    hasher.update((data.len() as u64).to_le_bytes());
    hasher.update(data);
}

fn hash_option_vec(hasher: &mut Sha256, data: &Option<Vec<u8>>) {
    match data {
        Some(vec) => {
            hasher.update([1u8]);
            hash_vec(hasher, vec);
        }
        None => {
            hasher.update([0u8]);
        }
    }
}

fn hash_option_u64(hasher: &mut Sha256, value: &Option<u64>) {
    match value {
        Some(v) => {
            hasher.update([1u8]);
            hasher.update(v.to_le_bytes());
        }
        None => {
            hasher.update([0u8]);
        }
    }
}

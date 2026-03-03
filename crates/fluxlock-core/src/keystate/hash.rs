use sha2::{Digest, Sha256};

use super::{AlgorithmId, KeyState};

impl KeyState {
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();

        hasher.update(self.key_epoch.to_le_bytes());
        hasher.update(self.activated_at_tick.to_le_bytes());
        hasher.update([algorithm_to_u8(&self.algorithm)]);

        hash_vec(&mut hasher, &self.current_pubkey);
        hash_option_vec(&mut hasher, &self.next_pubkey_commitment);

        hasher.update(self.rotation_policy.epoch_length.to_le_bytes());

        // NEW: rotation_override flag
        hasher.update([self.rotation_override as u8]);

        hasher.update(self.not_before_tick.to_le_bytes());
        hash_option_u64(&mut hasher, &self.not_after_tick);
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

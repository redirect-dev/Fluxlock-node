use blake3;
use crate::state::account::Account;

/// Compute hash of full account state
pub fn hash_accounts(accounts: &Vec<Account>) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();

    for acc in accounts {
        hasher.update(&acc.balance.to_le_bytes());
        hasher.update(&acc.nonce.to_le_bytes());
        hasher.update(&acc.current_classical_pubkey);
        hasher.update(&acc.current_pq_pubkey);
    }

    let hash = hasher.finalize();

    *hash.as_bytes()
}
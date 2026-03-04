use blake3::Hasher;

use crate::state::account::Account;
use crate::state::validator::Validator;

#[derive(Clone)]
pub struct GenesisState {
    pub tick: u64,
    pub accounts: Vec<Account>,
    pub validators: Vec<Validator>,
    pub state_root: [u8; 32],
}

impl GenesisState {
    pub fn new(
        accounts: Vec<Account>,
        validators: Vec<Validator>,
    ) -> Self {
        let tick = 0;

        let state_root = Self::compute_state_root(&accounts, &validators, tick);

        Self {
            tick,
            accounts,
            validators,
            state_root,
        }
    }

    fn compute_state_root(
        accounts: &Vec<Account>,
        validators: &Vec<Validator>,
        tick: u64,
    ) -> [u8; 32] {
        let mut hasher = Hasher::new();

        hasher.update(&tick.to_le_bytes());

        for account in accounts {
            hasher.update(&account.balance.to_le_bytes());
            hasher.update(&account.nonce.to_le_bytes());
            hasher.update(&account.current_classical_pubkey);
            hasher.update(&account.current_pq_pubkey);
        }

        for validator in validators {
            hasher.update(&validator.stake.to_le_bytes());
            hasher.update(&validator.classical_pubkey);
            hasher.update(&validator.pq_pubkey);
        }

        *hasher.finalize().as_bytes()
    }
}

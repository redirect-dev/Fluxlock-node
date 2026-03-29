use blake3;

use crate::genesis::state::GenesisState;
use crate::genesis::GenesisConfig;
use crate::state::account::Account;
use crate::state::validator::Validator;

/// Build deterministic genesis state
pub fn build_genesis_state(config: GenesisConfig) -> GenesisState {
    let accounts: Vec<Account> = config.initial_accounts;
    let validators: Vec<Validator> = config.initial_validators;

    let state_root = compute_state_root(&accounts, &validators);

    GenesisState::new(accounts, validators, state_root)
}

/// Simple deterministic state root (placeholder)
fn compute_state_root(
    accounts: &Vec<Account>,
    validators: &Vec<Validator>,
) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();

    hasher.update(&(accounts.len() as u64).to_le_bytes());
    hasher.update(&(validators.len() as u64).to_le_bytes());

    let hash = hasher.finalize();

    *hash.as_bytes()
}
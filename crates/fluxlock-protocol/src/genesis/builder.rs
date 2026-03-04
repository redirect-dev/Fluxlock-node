use crate::genesis::state::GenesisState;
use crate::genesis::GenesisConfig;

use crate::state::account::Account;
use crate::state::validator::Validator;

pub fn build_genesis_state(config: GenesisConfig) -> GenesisState {
    let mut accounts: Vec<Account> = config.initial_accounts.clone();
    let mut validators: Vec<Validator> = Vec::new();

    for v in config.initial_validators.iter() {
        if v.stake < config.min_validator_stake {
            panic!("Validator stake below minimum requirement");
        }

        validators.push(v.clone());

        // Create validator account mirror
        let account = Account::new(
            v.stake,
            v.classical_pubkey.clone(),
            v.pq_pubkey.clone(),
        );

        accounts.push(account);
    }

    // Deterministic ordering by classical pubkey
    accounts.sort_by(|a, b| a.current_classical_pubkey.cmp(&b.current_classical_pubkey));

    validators.sort_by(|a, b| a.classical_pubkey.cmp(&b.classical_pubkey));

    GenesisState::new(accounts, validators)
}

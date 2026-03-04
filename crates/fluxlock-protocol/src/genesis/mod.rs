pub mod builder;
pub mod state;
pub mod block;

use crate::state::account::Account;
use crate::state::validator::Validator;

#[derive(Clone)]
pub struct GenesisConfig {
    pub tick_duration_seconds: u64,

    pub validator_max_identity_lifetime: u64,
    pub account_max_identity_lifetime: u64,

    pub commit_delay_ticks: u64,
    pub reveal_window_ticks: u64,

    pub min_validator_stake: u128,

    pub initial_validators: Vec<Validator>,
    pub initial_accounts: Vec<Account>,
}

impl GenesisConfig {
    pub fn default() -> Self {
        Self {
            tick_duration_seconds: 2,
            validator_max_identity_lifetime: 100_000,
            account_max_identity_lifetime: 1_000_000,
            commit_delay_ticks: 500,
            reveal_window_ticks: 5_000,
            min_validator_stake: 1_000_000,
            initial_validators: vec![],
            initial_accounts: vec![],
        }
    }
}

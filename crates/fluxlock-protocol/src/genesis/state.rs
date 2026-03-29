use crate::state::account::Account;
use crate::state::validator::Validator;

#[derive(Clone)]
pub struct GenesisState {
    pub tick: u64,
    pub accounts: Vec<Account>,
    pub validators: Vec<Validator>,
    pub state_root: [u8; 32],

    pub counter: u64,
}

impl GenesisState {
    pub fn new(
        accounts: Vec<Account>,
        validators: Vec<Validator>,
        state_root: [u8; 32],
    ) -> Self {
        Self {
            tick: 0,
            accounts,
            validators,
            state_root,
            counter: 0,
        }
    }
}
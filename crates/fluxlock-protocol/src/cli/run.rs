use std::{thread, time::Duration};

use crate::genesis::builder::build_genesis_state;
use crate::genesis::block::build_genesis_block;
use crate::genesis::GenesisConfig;

use crate::block::producer::produce_block;
use crate::state::validator::Validator;
use crate::state::account::Account;

pub fn run_chain() {
    println!("⛓ Starting Fluxlock live chain...\n");

    // --- Simple validator ---
    let validator = Validator::new(
        1_000_000,
        vec![9; 32],
        vec![8; 32],
        0,
        100_000,
    );

    // --- Simple accounts ---
    let alice = Account::new(1_000, vec![1; 32], vec![2; 32]);
    let bob = Account::new(0, vec![3; 32], vec![4; 32]);

    let mut config = GenesisConfig::default();
    config.initial_validators.push(validator);
    config.initial_accounts.push(alice);
    config.initial_accounts.push(bob);

    let mut state = build_genesis_state(config.clone());
    let mut block = build_genesis_block(&state);

    println!("Genesis tick: {}\n", block.tick);

    loop {
        let result = produce_block(
            &block,
            &mut state.validators,
            &mut state.accounts,
            vec![],
            state.counter,
        );

        match result {
            Ok((next_block, new_counter)) => {
                state.counter = new_counter;
                block = next_block;
            }
            Err(e) => {
                println!("Block rejected: {}", e);
            }
        }

        let alice = &state.accounts[0];

        println!(
            "Tick {} | Balance: {} | Epoch: {}",
            block.tick,
            alice.balance,
            alice.rotation_epoch
        );

        // slow it down so humans can see it
        thread::sleep(Duration::from_millis(1000));
    }
}
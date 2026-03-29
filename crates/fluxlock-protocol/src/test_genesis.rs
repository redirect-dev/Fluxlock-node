use crate::genesis::builder::build_genesis_state;
use crate::genesis::block::build_genesis_block;
use crate::genesis::GenesisConfig;

use crate::block::producer::produce_block;
use crate::state::validator::Validator;
use crate::state::account::Account;
use crate::tx::transaction::TransferTx;

pub fn run_genesis_test() {
    println!("--- Fluxlock Genesis Test ---");

    // 🔐 Create validator
    let validator = Validator::new(
        1_000_000,
        vec![9, 9, 9],
        vec![8, 8, 8],
        0,
        100_000,
    );

    // 👤 Create accounts
    let alice_key = vec![1, 1, 1];
    let bob_key = vec![2, 2, 2];

    let alice = Account::new(1_000, alice_key.clone(), vec![0]);
    let bob = Account::new(0, bob_key.clone(), vec![0]);

    let mut config = GenesisConfig::default();
    config.initial_validators.push(validator);
    config.initial_accounts.push(alice);
    config.initial_accounts.push(bob);

    // Build genesis
    let mut genesis_state = build_genesis_state(config.clone());
    let genesis_block = build_genesis_block(&genesis_state);

    println!("Genesis tick: {}", genesis_block.tick);

    let mut current_block = genesis_block;

    for i in 1..=5 {
        // 🔥 Create transaction only for first block
        let txs = if i == 1 {
            vec![TransferTx {
                from: alice_key.clone(),
                to: bob_key.clone(),
                amount: 100,
                nonce: 0,
            }]
        } else {
            vec![]
        };

        let (next_block, new_counter) = produce_block(
            &current_block,
            &genesis_state.validators,
            &mut genesis_state.accounts,
            txs,
            genesis_state.counter,
        );

        genesis_state.counter = new_counter;

        // Print balances AFTER execution
        let alice_balance = genesis_state.accounts[0].balance;
        let bob_balance = genesis_state.accounts[1].balance;

        println!(
            "After Block {} | Alice: {} | Bob: {}",
            i, alice_balance, bob_balance
        );

        current_block = next_block;
    }

    println!("--- Success: Block execution pipeline working ---");
}
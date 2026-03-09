use crate::genesis::builder::build_genesis_state;
use crate::genesis::block::build_genesis_block;
use crate::genesis::GenesisConfig;

use crate::block::producer::produce_block;
use crate::state::validator::Validator;

pub fn run_genesis_test() {
    println!("--- Fluxlock Genesis Test ---");

    // Create dummy validator
    let validator = Validator::new(
        1_000_000,
        vec![1, 2, 3], // dummy classical key
        vec![4, 5, 6], // dummy pq key
        0,
        100_000,
    );

    let mut config = GenesisConfig::default();
    config.initial_validators.push(validator);

    // Build genesis state
    let genesis_state = build_genesis_state(config.clone());

    // Build genesis block
    let genesis_block = build_genesis_block(&genesis_state);

    println!("Genesis tick: {}", genesis_block.tick);

    // Produce next block
    let mut current_block = genesis_block;

for i in 1..=5 {
    let next_block = produce_block(&current_block, &genesis_state.validators);

    println!("Block {} tick: {}", i, next_block.tick);

    current_block = next_block;
}
}
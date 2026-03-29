use crate::genesis::builder::build_genesis_state;
use crate::genesis::block::build_genesis_block;
use crate::genesis::GenesisConfig;

use crate::block::producer::produce_block;
use crate::state::validator::Validator;
use crate::state::account::Account;

use crate::tx::transaction::{Tx, TransferTx, RotationCommitTx};

use ed25519_dalek::{SigningKey, Signer};

use blake3;

pub fn run_genesis_test() {
    println!("--- Fluxlock Enforcement Test ---");

    // 🔐 Key
    let signing_key = SigningKey::from_bytes(&[1u8; 32]);
    let verify_key = signing_key.verifying_key();
    let alice_key = verify_key.to_bytes().to_vec();

    let bob_key = vec![2; 32];

    // 🔐 Fake next key (for commitment)
    let new_key = vec![7; 32];
    let mut hasher = blake3::Hasher::new();
    hasher.update(&new_key);
    let commitment = hasher.finalize().as_bytes().to_vec();

    let validator = Validator::new(
        1_000_000,
        vec![9; 32],
        vec![8; 32],
        0,
        100_000,
    );

    let alice = Account::new(1_000, alice_key.clone(), vec![0]);
    let bob = Account::new(0, bob_key.clone(), vec![0]);

    let mut config = GenesisConfig::default();
    config.initial_validators.push(validator);
    config.initial_accounts.push(alice);
    config.initial_accounts.push(bob);

    let mut genesis_state = build_genesis_state(config.clone());
    let genesis_block = build_genesis_block(&genesis_state);

    println!("Genesis tick: {}", genesis_block.tick);

    let mut current_block = genesis_block;

    for i in 1..=12 {
        let txs = if i == 1 {
            // 🔥 Commit rotation ONLY (no reveal later)
            vec![
                Tx::RotationCommit(RotationCommitTx {
                    from: alice_key.clone(),
                    new_key_commitment: commitment.clone(),
                    nonce: 0,
                    signature: vec![0; 64],
                }),
            ]
        } else if i == 11 {
            // 🔥 Attempt transfer AFTER deadline
            let mut message = vec![];
            message.extend(&alice_key);
            message.extend(&bob_key);
            message.extend(&100u128.to_le_bytes());
            message.extend(&1u64.to_le_bytes());

            let sig = signing_key.sign(&message);

            vec![
                Tx::Transfer(TransferTx {
                    from: alice_key.clone(),
                    to: bob_key.clone(),
                    amount: 100,
                    nonce: 1,
                    signature: sig.to_bytes().to_vec(),
                }),
            ]
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

        let alice = &genesis_state.accounts[0];

        println!(
            "Block {} | Balance: {} | Deadline: {:?}",
            i,
            alice.balance,
            alice.rotation_deadline_tick
        );

        current_block = next_block;
    }

    println!("--- Enforcement test complete ---");
}
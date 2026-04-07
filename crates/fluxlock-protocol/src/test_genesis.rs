use crate::genesis::builder::build_genesis_state;
use crate::genesis::block::build_genesis_block;
use crate::genesis::GenesisConfig;

use crate::block::producer::produce_block;
use crate::state::validator::Validator;
use crate::state::account::{Account, FLAG_IDENTITY_EXPIRED};

use crate::tx::transaction::{
    Tx,
    TransferTx,
    RotationCommitTx,
    RotationRevealTx,
};

use ed25519_dalek::{SigningKey, Signer};

use blake3;

pub fn run_genesis_test() {
    println!("--- Fluxlock FULL Rotation Test ---");

    let signing_key = SigningKey::from_bytes(&[1u8; 32]);
    let verify_key = signing_key.verifying_key();
    let alice_key = verify_key.to_bytes().to_vec();

    let bob_key = vec![2; 32];

    let (pq_public, pq_secret) = crate::pq::generate_keypair();

    let new_signing_key = SigningKey::from_bytes(&[7u8; 32]);
    let new_verify_key = new_signing_key.verifying_key();
    let new_classical = new_verify_key.to_bytes().to_vec();

    let (new_pq_public, new_pq_secret) = crate::pq::generate_keypair();

    let mut hasher = blake3::Hasher::new();
    hasher.update(&new_classical);
    hasher.update(&new_pq_public);
    let commitment = hasher.finalize().as_bytes().to_vec();

    let validator = Validator::new(
        1_000_000,
        vec![9; 32],
        vec![8; 32],
        0,
        100_000,
    );

    let alice = Account::new(1_000, alice_key.clone(), pq_public.clone());
    let bob = Account::new(0, bob_key.clone(), vec![0]);

    let mut config = GenesisConfig::default();
    config.initial_validators.push(validator);
    config.initial_accounts.push(alice);
    config.initial_accounts.push(bob);

    let mut genesis_state = build_genesis_state(config.clone());
    let genesis_block = build_genesis_block(&genesis_state);

    println!("Genesis tick: {}", genesis_block.tick);

    let mut current_block = genesis_block;
    let mut expired_triggered = false;

    for i in 1..=12 {
        let epoch = i as u64;

        let txs = if i == 1 {
            println!("\n🔐 ROTATION COMMIT INITIATED");

            let mut message = vec![];
            message.extend(&epoch.to_be_bytes());
            message.extend(&alice_key);
            message.extend(&commitment);
            message.extend(&0u64.to_le_bytes());

            let classical_sig = signing_key.sign(&message).to_bytes().to_vec();
            let pq_sig = crate::pq::sign(&message, &pq_secret);

            vec![Tx::RotationCommit(RotationCommitTx {
                from: alice_key.clone(),
                new_key_commitment: commitment.clone(),
                nonce: 0,
                epoch,
                classical_signature: classical_sig,
                pq_signature: pq_sig,
            })]
        } else if i == 3 {
            println!("\n🔐 ROTATION REVEAL — NEW IDENTITY ACTIVATED");

            let mut message = vec![];
            message.extend(&epoch.to_be_bytes());
            message.extend(&alice_key);
            message.extend(&new_classical);
            message.extend(&new_pq_public);
            message.extend(&1u64.to_le_bytes());

            let classical_sig = signing_key.sign(&message).to_bytes().to_vec();
            let pq_sig = crate::pq::sign(&message, &pq_secret);

            vec![Tx::RotationReveal(RotationRevealTx {
                from: alice_key.clone(),
                new_classical_key: new_classical.clone(),
                new_pq_key: new_pq_public.clone(),
                nonce: 1,
                epoch,
                classical_signature: classical_sig,
                pq_signature: pq_sig,
            })]
        } else if i == 9 {
            println!("\n🚨 THREAT: Attempt to reuse expired credentials");

            let mut message = vec![];
            message.extend(&epoch.to_be_bytes());
            message.extend(&alice_key); // OLD KEY
            message.extend(&bob_key);
            message.extend(&50u128.to_le_bytes());
            message.extend(&2u64.to_le_bytes());

            let classical_sig = signing_key.sign(&message).to_bytes().to_vec();
            let pq_sig = crate::pq::sign(&message, &pq_secret);

            vec![Tx::Transfer(TransferTx {
                from: alice_key.clone(),
                to: bob_key.clone(),
                amount: 50,
                nonce: 2,
                epoch,
                classical_signature: classical_sig,
                pq_signature: pq_sig,
            })]
        } else if i == 11 {
            println!("\n✅ VALID TRANSACTION WITH ROTATED IDENTITY");

            let mut message = vec![];
            message.extend(&epoch.to_be_bytes());
            message.extend(&new_classical);
            message.extend(&bob_key);
            message.extend(&100u128.to_le_bytes());
            message.extend(&2u64.to_le_bytes());

            let classical_sig = new_signing_key.sign(&message).to_bytes().to_vec();
            let pq_sig = crate::pq::sign(&message, &new_pq_secret);

            vec![Tx::Transfer(TransferTx {
                from: new_classical.clone(),
                to: bob_key.clone(),
                amount: 100,
                nonce: 2,
                epoch,
                classical_signature: classical_sig,
                pq_signature: pq_sig,
            })]
        } else {
            vec![]
        };

        let result = produce_block(
            &current_block,
            &mut genesis_state.validators,
            &mut genesis_state.accounts,
            txs,
            genesis_state.counter,
        );

        match result {
            Ok((next_block, new_counter)) => {
                genesis_state.counter = new_counter;
                current_block = next_block;
            }
            Err(_) => {
                println!("❌ Transaction rejected: identity no longer valid");
                println!("🛑 Network rejected expired identity");
            }
        }

        let alice = &genesis_state.accounts[0];
        let expired = alice.has_flag(FLAG_IDENTITY_EXPIRED);

        println!(
            "Block {} | Balance: {} | Expired: {} | Epoch: {}",
            i,
            alice.balance,
            expired,
            alice.rotation_epoch
        );

        if expired && !expired_triggered {
            println!("\n🚨 IDENTITY HAS EXPIRED AT BLOCK {}\n", i);
            expired_triggered = true;
        }
    }

    println!("\n--- FULL rotation test complete ---");
}
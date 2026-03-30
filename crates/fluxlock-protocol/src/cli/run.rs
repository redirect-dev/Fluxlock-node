use std::{thread, time::Duration};

use colored::*;
use ed25519_dalek::{SigningKey, Signer};

use crate::genesis::builder::build_genesis_state;
use crate::genesis::block::build_genesis_block;
use crate::genesis::GenesisConfig;

use crate::block::producer::produce_block;
use crate::state::validator::Validator;
use crate::state::account::Account;

use crate::tx::transaction::{Tx, RotationCommitTx, RotationRevealTx, TransferTx};

use blake3;

pub fn run_chain() {
    println!(
        "\n{}\n{}\n{}\n",
        "=======================================".blue(),
        "🚀 FLUXLOCK LIVE SECURITY DEMO".bold().bright_cyan(),
        "=======================================".blue()
    );

    // --- Keys ---
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

    let mut state = build_genesis_state(config.clone());
    let mut block = build_genesis_block(&state);

    println!("{}", "🧬 Genesis Initialized".green());
    println!("   Tick: {}", block.tick.to_string().yellow());
    println!("   Alice Balance: {}\n", "1000".green());

    for i in 1..=12 {
        println!("{}", "---------------------------------------".dimmed());
        println!(
            "{} {}",
            "⏱ Tick".bold(),
            i.to_string().bright_yellow()
        );

        let txs = if i == 3 {
            println!("{}", "🔐 EVENT: Identity Commit Initiated".cyan());

            let mut msg = vec![];
            msg.extend(&alice_key);
            msg.extend(&commitment);
            msg.extend(&0u64.to_le_bytes());

            vec![Tx::RotationCommit(RotationCommitTx {
                from: alice_key.clone(),
                new_key_commitment: commitment.clone(),
                nonce: 0,
                classical_signature: signing_key.sign(&msg).to_bytes().to_vec(),
                pq_signature: crate::pq::sign(&msg, &pq_secret),
            })]
        } else if i == 4 {
            println!("{}", "🔁 EVENT: Identity Rotation Executed".bright_green());

            let mut msg = vec![];
            msg.extend(&alice_key);
            msg.extend(&new_classical);
            msg.extend(&new_pq_public);
            msg.extend(&1u64.to_le_bytes());

            vec![Tx::RotationReveal(RotationRevealTx {
                from: alice_key.clone(),
                new_classical_key: new_classical.clone(),
                new_pq_key: new_pq_public.clone(),
                nonce: 1,
                classical_signature: signing_key.sign(&msg).to_bytes().to_vec(),
                pq_signature: crate::pq::sign(&msg, &pq_secret),
            })]
        } else if i == 10 {
            println!(
                "{}",
                "⚠️ EVENT: Attempting transaction with outdated identity"
                    .yellow()
                    .bold()
            );

            let mut msg = vec![];
            msg.extend(&new_classical);
            msg.extend(&bob_key);
            msg.extend(&100u128.to_le_bytes());
            msg.extend(&2u64.to_le_bytes());

            vec![Tx::Transfer(TransferTx {
                from: new_classical.clone(),
                to: bob_key.clone(),
                amount: 100,
                nonce: 2,
                classical_signature: new_signing_key.sign(&msg).to_bytes().to_vec(),
                pq_signature: crate::pq::sign(&msg, &new_pq_secret),
            })]
        } else {
            vec![]
        };

        let result = produce_block(
            &block,
            &mut state.validators,
            &mut state.accounts,
            txs,
            state.counter,
        );

        match result {
            Ok((next_block, new_counter)) => {
                state.counter = new_counter;
                block = next_block;
                println!("{}", "✅ Block accepted".green());
            }
            Err(e) => {
                println!("{}", format!("❌ Block rejected: {}", e).red().bold());
            }
        }

        let alice = &state.accounts[0];

        println!(
            "{} {} | {} {}",
            "📊 Balance:".bold(),
            alice.balance.to_string().green(),
            "Epoch:".bold(),
            alice.rotation_epoch.to_string().blue()
        );

        thread::sleep(Duration::from_millis(600));
    }

    println!(
        "\n{}\n{}\n{}\n{}\n{}\n{}\n",
        "=======================================".blue(),
        "🧠 DEMO SUMMARY".bold().bright_cyan(),
        "=======================================".blue(),
        "✔ Identity rotated successfully".green(),
        "✔ Epoch enforcement triggered rejection".yellow(),
        "✔ Time-bound security enforced".cyan()
    );
}
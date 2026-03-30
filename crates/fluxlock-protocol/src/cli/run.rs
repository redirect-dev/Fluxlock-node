use std::{thread, time::Duration};

use colored::*;

use crate::cli::wallet::load_wallet;
use crate::cli::mempool::drain_txs;

use crate::genesis::builder::build_genesis_state;
use crate::genesis::block::build_genesis_block;
use crate::genesis::GenesisConfig;

use crate::block::producer::produce_block;
use crate::state::validator::Validator;
use crate::state::account::Account;

pub fn run_chain() {
    println!(
        "\n{}\n{}\n{}\n",
        "=======================================".blue(),
        "🚀 FLUXLOCK LIVE SECURITY DEMO".bold().bright_cyan(),
        "=======================================".blue()
    );

    // 🔥 LOAD WALLET (THIS IS THE FIX)
    let wallet = match load_wallet() {
        Some(w) => w,
        None => return,
    };

    let alice_key = wallet.classical_public.clone();
    let alice_pq = wallet.pq_public.clone();

    let bob_key = vec![2; 32];

    let validator = Validator::new(
        1_000_000,
        vec![9; 32],
        vec![8; 32],
        0,
        100_000,
    );

    let alice = Account::new(1_000, alice_key.clone(), alice_pq.clone());
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
        println!("⏱ Tick {}", i.to_string().bright_yellow());

        // 🔥 THIS IS NOW REAL TX INPUT
        let txs = drain_txs();

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
        "\n{}\n{}\n{}\n",
        "=======================================".blue(),
        "🧠 DEMO COMPLETE".bold().bright_cyan(),
        "=======================================".blue(),
    );
}
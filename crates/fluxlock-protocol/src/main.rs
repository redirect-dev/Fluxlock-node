use std::env;

mod simulation;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "demo" => {
            println!("🚀 Running Fluxlock demo...\n");
            fluxlock_protocol::test_genesis::run_genesis_test();
        }

        "new-account" => {
            fluxlock_protocol::cli::wallet::create_wallet();
        }

        // 🔥 REMOVED broken run_chain
        "run" => {
            println!("⚠️ Run command not implemented in Phase 9 yet\n");
        }

        "send" => {
            fluxlock_protocol::cli::send::send_tx();
        }

        "simulate" => {
            println!("🧪 Running multi-validator simulation...\n");
            simulation::run_simulation();
        }

        "help" => {
            print_help();
        }

        _ => {
            println!("❌ Unknown command\n");
            print_help();
        }
    }
}

fn print_help() {
    println!("Fluxlock CLI\n");
    println!("Usage:");
    println!("  fluxlock-protocol demo          Run demo");
    println!("  fluxlock-protocol new-account   Create wallet");
    println!("  fluxlock-protocol run           (disabled in Phase 9)");
    println!("  fluxlock-protocol send          Send transaction");
    println!("  fluxlock-protocol simulate      Run validator simulation");
    println!("  fluxlock-protocol help          Show help\n");
}
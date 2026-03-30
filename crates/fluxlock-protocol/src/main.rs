use std::env;

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
            fluxlock_protocol::cli::account::create_account();
        }

        "run" => {
            fluxlock_protocol::cli::run::run_chain();
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
    println!("  fluxlock-protocol demo          Run full protocol demo");
    println!("  fluxlock-protocol new-account   Generate a new account");
    println!("  fluxlock-protocol run           Run live chain");
    println!("  fluxlock-protocol help          Show this help message\n");
}
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "run" => run_demo(false),
        "attack" => run_demo(true),
        "explain" => explain(),
        _ => print_help(),
    }
}

fn run_demo(attack: bool) {
    if attack {
        println!("Running Fluxlock demo in ATTACK mode...\n");
        std::process::Command::new("cargo")
            .args(["run", "-p", "fluxlock-protocol", "--bin", "fluxlock-demo", "--", "--attack"])
            .status()
            .expect("failed to run demo");
    } else {
        println!("Running Fluxlock demo...\n");
        std::process::Command::new("cargo")
            .args(["run", "-p", "fluxlock-protocol", "--bin", "fluxlock-demo"])
            .status()
            .expect("failed to run demo");
    }
}

fn explain() {
    println!("\n==================== FLUXLOCK ====================");
    println!("Fluxlock enforces time-bound identity.\n");
    println!("Instead of trusting identity indefinitely:");
    println!("→ identities must remain current");
    println!("→ expired identities are rejected");
    println!("→ invalid acceptance results in penalties\n");

    println!("Core rule:");
    println!("valid key + valid time = valid identity\n");

    println!("This reduces:");
    println!("• credential replay");
    println!("• long-lived access");
    println!("• delayed breach impact");

    println!("=================================================\n");
}

fn print_help() {
    println!("\nFluxlock CLI\n");
    println!("Usage:");
    println!("  fluxlock run      → run demo");
    println!("  fluxlock attack   → run attack simulation");
    println!("  fluxlock explain  → explain the model\n");
}
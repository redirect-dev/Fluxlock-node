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
        "validate" => validate(&args),
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

fn validate(args: &[String]) {
    let mut identity = String::new();
    let mut epoch: i32 = -1;

    for i in 0..args.len() {
        if args[i] == "--identity" && i + 1 < args.len() {
            identity = args[i + 1].clone();
        }
        if args[i] == "--epoch" && i + 1 < args.len() {
            epoch = args[i + 1].parse().unwrap_or(-1);
        }
    }

    if identity.is_empty() || epoch < 0 {
        println!("Usage:");
        println!("fluxlock validate --identity ID-1000 --epoch 1");
        return;
    }

    // simple rule for demo:
    // ID-1000 = epoch 0
    // ID-1001 = epoch 1

    let valid = match identity.as_str() {
        "ID-1000" => epoch == 0,
        "ID-1001" => epoch == 1,
        _ => false,
    };

    println!("\n================ VALIDATION =================");

    if valid {
        println!("✅ VALID: identity is current");
        println!("Identity: {}", identity);
        println!("Epoch: {}", epoch);
    } else {
        println!("❌ INVALID: identity expired or out of sync");
        println!("Identity: {}", identity);
        println!("Epoch: {}", epoch);
    }

    println!("===========================================\n");
}

fn print_help() {
    println!("\nFluxlock CLI\n");
    println!("Commands:");
    println!("  fluxlock run");
    println!("  fluxlock attack");
    println!("  fluxlock explain");
    println!("  fluxlock validate --identity ID-1000 --epoch 1\n");
}
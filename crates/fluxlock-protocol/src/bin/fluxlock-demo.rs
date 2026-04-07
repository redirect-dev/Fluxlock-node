use std::env;
use std::thread::sleep;
use std::time::Duration;

fn pause() {
    sleep(Duration::from_millis(600));
}

// simple identity generator (static for demo clarity)
fn identity(epoch: u32) -> String {
    format!("ID-{}", 1000 + epoch)
}

fn attack_mode(expired_id: &str) {
    println!("\n==================== ATTACK SIMULATION ====================");
    println!("Targeting expired identity: {}\n", expired_id);

    let mut stake = 1_000_000;

    for i in 1..=3 {
        println!("🚨 Attack Attempt {}", i);
        pause();

        println!("Using expired identity: {}", expired_id);
        println!("❌ Transaction rejected: identity expired");

        stake -= 25_000;
        println!("⚔ Validator slashed → New stake: {}", stake);

        println!("--------------------------------------------------\n");
        pause();
    }

    println!("🛑 Attack unsuccessful");
    println!("System invariant preserved: expired identity remains invalid\n");

    println!("==========================================================\n");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let attack = args.iter().any(|arg| arg == "--attack");

    println!("\n==================== FLUXLOCK ====================");
    println!("Time-bound identity enforcement demo");
    println!("Identity expires. Expired identity cannot act.");
    println!("=================================================\n");

    pause();

    println!("🌱 Bootstrapping chain...");
    pause();

    // identity setup
    let id_epoch_0 = identity(0);
    let id_epoch_1 = identity(1);

    println!("\n--- ROTATION PHASE ---");
    println!("🔐 Identity commit initiated");
    println!("Current identity: {}", id_epoch_0);
    pause();

    println!("Block 1 | Identity {} | Epoch: 0", id_epoch_0);
    pause();

    println!("Block 2 | Identity {} | Epoch: 0", id_epoch_0);
    pause();

    println!("\n🔁 Identity reveal → NEW identity activated");
    println!("New identity: {}", id_epoch_1);
    pause();

    println!("Block 3 | Identity {} | Epoch: 1", id_epoch_1);
    pause();

    println!("Block 4 | Identity {} | Epoch: 1", id_epoch_1);
    pause();

    println!("\n--- EXPIRATION EVENT ---");
    println!("⚠ Identity {} is now INVALID", id_epoch_0);
    pause();

    println!("Block 5 | {} expired", id_epoch_0);
    pause();

    println!("\n--- ATTACK ATTEMPT ---");
    println!("🚨 Attempting to reuse expired identity: {}", id_epoch_0);
    pause();

    println!("\n🚨 PROTOCOL VIOLATION DETECTED");
    println!("❌ Transaction rejected: {} is expired", id_epoch_0);
    println!("⚔ Validator slashed");
    pause();

    println!("\nBlock 6 | State unchanged | Attack failed");
    pause();

    if attack {
        attack_mode(&id_epoch_0);
    }

    println!("\n--- VALID TRANSACTION ---");
    println!("✅ Using current identity: {}", id_epoch_1);
    pause();

    println!("Block 7 | Transaction accepted");
    pause();

    println!("\n--- WHAT YOU JUST SAW ---");
    println!("• Identity {} rotated → {}", id_epoch_0, id_epoch_1);
    println!("• Epoch advanced");
    println!("• {} expired", id_epoch_0);
    println!("• Expired identity was rejected");
    println!("• Invalid behavior triggered slashing");

    println!("\n==================== RESULT ====================");
    println!("Identity is NOT permanent");
    println!("Expired identities cannot act");
    println!("The network enforces validity over time");
    println!("===============================================\n");
}
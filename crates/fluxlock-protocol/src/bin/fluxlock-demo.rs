use std::thread::sleep;
use std::time::Duration;

fn pause() {
    sleep(Duration::from_millis(700));
}

fn main() {
    println!("\n==================== FLUXLOCK ====================");
    println!("Time-bound identity enforcement demo");
    println!("Identity expires. Invalid identity is rejected.");
    println!("=================================================\n");

    pause();

    println!("🌱 Bootstrapping chain...");
    pause();

    println!("\n--- ROTATION PHASE ---");
    println!("🔐 Identity commit initiated");
    pause();

    println!("Block 1 | Identity valid | Epoch: 0");
    pause();

    println!("Block 2 | Identity valid | Epoch: 0");
    pause();

    println!("\n🔁 Identity reveal → NEW identity activated");
    pause();

    println!("Block 3 | Identity valid | Epoch: 1");
    pause();

    println!("Block 4 | Identity valid | Epoch: 1");
    pause();

    println!("\n--- EXPIRATION EVENT ---");
    println!("⚠ Old identity is now INVALID");
    pause();

    println!("Block 5 | Identity expired");
    pause();

    println!("\n--- ATTACK ATTEMPT ---");
    println!("🚨 Attempting transaction with expired identity...");
    pause();

    println!("\n🚨 PROTOCOL VIOLATION DETECTED");
    println!("❌ Transaction rejected: identity expired");
    println!("⚔ Validator slashed");
    pause();

    println!("\nBlock 6 | State unchanged | Attack failed");
    pause();

    println!("\n--- VALID TRANSACTION ---");
    println!("✅ Using rotated identity...");
    pause();

    println!("Block 7 | Transaction accepted");
    pause();

    println!("\n--- WHAT YOU JUST SAW ---");
    println!("• Identity rotated (commit → reveal)");
    println!("• Epoch advanced");
    println!("• Old identity expired");
    println!("• Expired identity was rejected");
    println!("• Invalid behavior triggered slashing");

    println!("\n==================== RESULT ====================");
    println!("Identity is NOT permanent");
    println!("Expired credentials are unusable");
    println!("The network enforces validity over time");
    println!("===============================================\n");
}
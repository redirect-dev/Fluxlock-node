use std::{thread, time::Duration};

fn pause() {
    thread::sleep(Duration::from_millis(500));
}

fn divider(title: &str) {
    println!("\n==================== {} ====================\n", title);
}

fn main() {
    divider("FLUXLOCK — LIVE SECURITY DEMO");
    pause();

    println!("🧠 Time-bound identity enforced at protocol level");
    println!("🔐 Dual-signature security (Ed25519 + Dilithium)");
    println!("⚔ Invalid behavior is punished automatically");

    pause();

    divider("LIVE EXECUTION");

    println!("🌱 Bootstrapping chain...\n");
    pause();

    // 🚀 RUN REAL ENGINE ONLY (NO FAKE NARRATIVE AFTER)
    fluxlock_protocol::test_genesis::run_genesis_test();

    pause();

    divider("KEY TAKEAWAY");

    println!("🧠 Identity is not permanent");
    println!("🔐 Expired keys cannot be reused");
    println!("⚔ The network actively defends itself");

    println!("\n✨ END OF DEMO\n");
}
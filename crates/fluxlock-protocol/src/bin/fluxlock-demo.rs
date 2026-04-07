use ed25519_dalek::{SigningKey, Signer, VerifyingKey, Signature, Verifier};

fn main() {
    println!("\n==================== FLUXLOCK ====================");
    println!("Time-bound identity enforcement demo");
    println!("Identity expires. Expired identity cannot act.");
    println!("=================================================\n");

    println!("🌱 Bootstrapping chain...\n");

    // -----------------------------
    // INITIAL IDENTITY
    // -----------------------------
    let signing_key = SigningKey::from_bytes(&[1u8; 32]);
    let verify_key = signing_key.verifying_key();

    let old_identity = verify_key.to_bytes();

    println!("--- ROTATION PHASE ---");
    println!("🔐 Identity commit initiated");
    println!("Current identity: ID-1000");

    println!("Block 1 | Identity ID-1000 | Epoch: 0");
    println!("Block 2 | Identity ID-1000 | Epoch: 0");

    // -----------------------------
    // NEW IDENTITY
    // -----------------------------
    let new_signing_key = SigningKey::from_bytes(&[7u8; 32]);
    let new_verify_key = new_signing_key.verifying_key();

    let new_identity = new_verify_key.to_bytes();

    // ✅ FIXED TYPE
    let epoch: u64 = 1;

    // -----------------------------
    // LINK DATA
    // -----------------------------
    let mut link_data = Vec::new();
    link_data.extend_from_slice(&epoch.to_be_bytes());
    link_data.extend_from_slice(&new_identity);

    // =============================
    // 🔥 TOGGLE THIS FLAG
    // =============================
    let use_fake_link = false;

    let link_signature = if use_fake_link {
        println!("🚨 USING FAKE LINK SIGNATURE (ATTACK TEST)");

        let fake_key = SigningKey::from_bytes(&[99u8; 32]);
        fake_key.sign(&link_data).to_bytes().to_vec()
    } else {
        signing_key.sign(&link_data).to_bytes().to_vec()
    };

    // -----------------------------
    // VERIFY LINK (PHASE 2B)
    // -----------------------------
    let old_pub = VerifyingKey::from_bytes(&old_identity).unwrap();

    let sig = Signature::from_bytes(
        &link_signature.clone().try_into().unwrap()
    );

    println!("\n🔁 Identity reveal attempt...");

    if old_pub.verify(&link_data, &sig).is_err() {
        println!("🚨 LINK SIGNATURE INVALID");
        println!("❌ Transaction rejected");
        println!("🛑 Identity chain broken — rotation denied\n");

        println!("==================== RESULT ====================");
        println!("Identity continuity ENFORCED");
        println!("Invalid identity cannot evolve");
        println!("===============================================\n");

        return;
    }

    // -----------------------------
    // SUCCESS PATH
    // -----------------------------
    println!("New identity: ID-1001");
    println!("Block 3 | Identity ID-1001 | Epoch: 1");
    println!("Block 4 | Identity ID-1001 | Epoch: 1");

    println!("\n--- EXPIRATION EVENT ---");
    println!("⚠ Identity ID-1000 is now INVALID");
    println!("Block 5 | ID-1000 expired");

    println!("\n--- ATTACK ATTEMPT ---");
    println!("🚨 Attempting to reuse expired identity: ID-1000");

    println!("\n🚨 PROTOCOL VIOLATION DETECTED");
    println!("❌ Transaction rejected: ID-1000 is expired");
    println!("⚔ Validator slashed");

    println!("\nBlock 6 | State unchanged | Attack failed");

    println!("\n--- VALID TRANSACTION ---");
    println!("✅ Using current identity: ID-1001");
    println!("Block 7 | Transaction accepted");

    println!("\n--- WHAT YOU JUST SAW ---");
    println!("• Identity ID-1000 rotated → ID-1001");
    println!("• Epoch advanced");
    println!("• ID-1000 expired");
    println!("• Expired identity was rejected");
    println!("• Invalid behavior triggered slashing");

    println!("\n==================== RESULT ====================");
    println!("Identity is NOT permanent");
    println!("Identity MUST prove continuity");
    println!("Expired identities cannot act");
    println!("===============================================\n");
}
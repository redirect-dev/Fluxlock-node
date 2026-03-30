use ed25519_dalek::SigningKey;

pub fn create_account() {
    println!("🔐 Generating new Fluxlock account...\n");

    // --- Classical key ---
    let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
    let verify_key = signing_key.verifying_key();

    let classical_pub = verify_key.to_bytes();

    // --- PQ key ---
    let (pq_public, _pq_secret) = crate::pq::generate_keypair();

    println!("✅ Account Created:\n");

    println!("Classical Public Key:");
    println!("{:?}\n", classical_pub);

    println!("PQ Public Key:");
    println!("{:?}\n", pq_public);

    println!("⚠️ Save your keys securely (no recovery implemented yet)\n");

    // NOTE: In future:
    // - save to file
    // - encrypt
    // - wallet support
}
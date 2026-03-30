use std::fs;
use std::path::Path;

use serde::{Serialize, Deserialize};
use ed25519_dalek::SigningKey;

const WALLET_PATH: &str = "wallet.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct Wallet {
    pub classical_public: Vec<u8>,
    pub classical_secret: Vec<u8>,
    pub pq_public: Vec<u8>,
    pub pq_secret: Vec<u8>,
    pub nonce: u64,
}

// -----------------------------
// CREATE WALLET (FIXED)
// -----------------------------
pub fn create_wallet() {
    println!("🔐 Generating new Fluxlock account...\n");

    // Classical keypair
    let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
    let verifying_key = signing_key.verifying_key();

    // PQ keypair
    let (pq_public, pq_secret) = crate::pq::generate_keypair();

    let wallet = Wallet {
        classical_public: verifying_key.to_bytes().to_vec(),
        classical_secret: signing_key.to_bytes().to_vec(),
        pq_public,
        pq_secret,
        nonce: 0,
    };

    save_wallet(&wallet);

    println!("✅ Account Created:\n");
    println!("Classical Public Key:\n{:?}\n", wallet.classical_public);
    println!("PQ Public Key:\n{:?}\n", wallet.pq_public);
    println!("⚠️ Save your keys securely (no recovery implemented yet)\n");
}

// -----------------------------
// LOAD
// -----------------------------
pub fn load_wallet() -> Option<Wallet> {
    if !Path::new(WALLET_PATH).exists() {
        println!("❌ No wallet found. Run `new-account` first.");
        return None;
    }

    let data = fs::read_to_string(WALLET_PATH).ok()?;
    serde_json::from_str(&data).ok()
}

// -----------------------------
// SAVE
// -----------------------------
pub fn save_wallet(wallet: &Wallet) {
    let json = serde_json::to_string_pretty(wallet)
        .expect("Failed to serialize wallet");

    fs::write(WALLET_PATH, json)
        .expect("Failed to write wallet");
}
use std::fs;
use std::path::Path;

use ed25519_dalek::SigningKey;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Wallet {
    pub classical_secret: Vec<u8>,
    pub classical_public: Vec<u8>,
    pub pq_public: Vec<u8>,
    pub pq_secret: Vec<u8>,
}

const WALLET_PATH: &str = "wallet.json";

pub fn save_wallet(wallet: &Wallet) {
    let json = serde_json::to_string_pretty(wallet)
        .expect("Failed to serialize wallet");

    fs::write(WALLET_PATH, json)
        .expect("Failed to write wallet file");

    println!("💾 Wallet saved to {}", WALLET_PATH);
}

pub fn load_wallet() -> Option<Wallet> {
    if !Path::new(WALLET_PATH).exists() {
        println!("❌ No wallet found. Run `new-account` first.");
        return None;
    }

    let data = fs::read_to_string(WALLET_PATH)
        .expect("Failed to read wallet file");

    let wallet: Wallet = serde_json::from_str(&data)
        .expect("Failed to parse wallet");

    Some(wallet)
}

/// Create and save a new wallet
pub fn create_wallet() {
    println!("🔐 Generating new wallet...\n");

    // Classical
    let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
    let verify_key = signing_key.verifying_key();

    let classical_secret = signing_key.to_bytes().to_vec();
    let classical_public = verify_key.to_bytes().to_vec();

    // PQ
    let (pq_public, pq_secret) = crate::pq::generate_keypair();

    let wallet = Wallet {
        classical_secret,
        classical_public,
        pq_public,
        pq_secret,
    };

    save_wallet(&wallet);

    println!("✅ Wallet created and saved\n");
}
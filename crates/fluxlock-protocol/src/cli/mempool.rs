use std::fs;
use std::path::Path;

use serde::{Serialize, Deserialize};

use crate::tx::transaction::Tx;

const MEMPOOL_PATH: &str = "mempool.json";

#[derive(Serialize, Deserialize)]
pub struct Mempool {
    pub txs: Vec<Tx>,
}

pub fn load_mempool() -> Mempool {
    if !Path::new(MEMPOOL_PATH).exists() {
        return Mempool { txs: vec![] };
    }

    let data = fs::read_to_string(MEMPOOL_PATH)
        .expect("Failed to read mempool");

    serde_json::from_str(&data).unwrap_or(Mempool { txs: vec![] })
}

pub fn save_mempool(pool: &Mempool) {
    let json = serde_json::to_string_pretty(pool)
        .expect("Failed to serialize mempool");

    fs::write(MEMPOOL_PATH, json)
        .expect("Failed to write mempool");
}

pub fn add_tx(tx: Tx) {
    let mut pool = load_mempool();
    pool.txs.push(tx);
    save_mempool(&pool);

    println!("📥 Transaction added to mempool");
}

pub fn drain_txs() -> Vec<Tx> {
    let mut pool = load_mempool();
    let txs = pool.txs.clone();
    pool.txs.clear();
    save_mempool(&pool);
    txs
}
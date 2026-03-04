use ed25519_dalek::{SigningKey, VerifyingKey, Signer};
use rand::rngs::StdRng;
use rand::{SeedableRng, RngCore};


use fluxlock_core::{EngineCompositeState, TickInput, TickRecord, TickLog};
use fluxlock_engine::{apply_tick, hash_state};

use fluxlock_pq::{dilithium_keypair, dilithium_sign};

fn main() {
    // Deterministic RNG
    let mut rng = StdRng::seed_from_u64(42);

    // Deterministic classical key
    let mut classical_seed = [0u8; 32];
    rng.fill_bytes(&mut classical_seed);
    let signing = SigningKey::from_bytes(&classical_seed);
    let verifying: VerifyingKey = signing.verifying_key();

    // Deterministic Dilithium key
    let (pq_pub, pq_secret) = dilithium_keypair();

    let mut state = EngineCompositeState::new();
    let mut tick_log = TickLog::new();

    let mut parent_hash = "GENESIS".to_string();

    for tick_index in 0u64..20 {
        let mut input = TickInput {
            commit_classical: None,
            commit_pq: None,
            reveal_classical: None,
            reveal_pq: None,
            classical_signature: None,
            pq_signature: None,
        };

        // --- Genesis Reveal ---
        if tick_index == 0 {
            input.reveal_classical = Some(verifying.to_bytes().to_vec());
            input.reveal_pq = Some(pq_pub.clone());
        }

        // --- Sign state hash ---
        let state_hash = hash_state(&state);
        let message_bytes = state_hash.as_bytes();

        // Classical signature
        let classical_sig = signing.sign(message_bytes);
        input.classical_signature = Some(classical_sig.to_bytes().to_vec());

        // PQ signature
        let pq_sig = dilithium_sign(&pq_secret, message_bytes);
        input.pq_signature = Some(pq_sig);

        // --- Apply tick ---
        apply_tick(&mut state, &input, tick_index)
            .expect("Deterministic transition failure");

        let new_state_hash = hash_state(&state);

        tick_log.records.push(TickRecord {
            tick_index,
            input,
            state: state.clone(),
            parent_hash: parent_hash.clone(),
            state_hash: new_state_hash.clone(),
        });

        parent_hash = new_state_hash;
    }

    // Write log
    std::fs::write(
        "tick_log.json",
        serde_json::to_string_pretty(&tick_log).unwrap(),
    )
    .unwrap();

    println!("Fluxlock hybrid simulation complete. Generated 20 ticks.");
}

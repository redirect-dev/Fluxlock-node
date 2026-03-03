use ed25519_dalek::{SigningKey, Signer};
use rand::rngs::OsRng;

use fluxlock_core::{EngineCompositeState, TickInput, TickLog, TickRecord};
use fluxlock_engine::{apply_tick, hash_state};

fn main() {
    let mut state = EngineCompositeState::new();
    let mut tick_log = TickLog::new();

    let mut parent_hash = String::from("GENESIS");

    let mut current_signing = SigningKey::generate(&mut OsRng);
    let mut pending_signing: Option<SigningKey> = None;

    for tick_index in 0u64..20u64 {

        let mut input = TickInput {
            commit_pubkey: None,
            reveal_pubkey: None,
            payload: None,
            signature: None,
        };

        // -----------------------------------------
        // Genesis reveal
        // -----------------------------------------
        if tick_index == 0 {
            input.reveal_pubkey =
                Some(current_signing.verifying_key().to_bytes().to_vec());
        }

        // -----------------------------------------
        // ROTATION 1
        // -----------------------------------------
        if tick_index == 4 {
            let new_signing = SigningKey::generate(&mut OsRng);
            input.commit_pubkey =
                Some(new_signing.verifying_key().to_bytes().to_vec());
            pending_signing = Some(new_signing);
        }

        if tick_index == 6 {
            if let Some(new_signing) = pending_signing.take() {

                let reveal_key = new_signing.verifying_key().to_bytes().to_vec();

                let mut message = Vec::new();
                message.extend(&reveal_key);
                message.extend(tick_index.to_le_bytes());

                // SIGN WITH OLD KEY
                let signature = current_signing.sign(&message);

                input.reveal_pubkey = Some(reveal_key);
                input.signature = Some(signature.to_bytes().to_vec());

                current_signing = new_signing;
            }
        }

        // -----------------------------------------
        // ROTATION 2
        // -----------------------------------------
        if tick_index == 8 {
            let new_signing = SigningKey::generate(&mut OsRng);
            input.commit_pubkey =
                Some(new_signing.verifying_key().to_bytes().to_vec());
            pending_signing = Some(new_signing);
        }

        if tick_index == 10 {
            if let Some(new_signing) = pending_signing.take() {

                let reveal_key = new_signing.verifying_key().to_bytes().to_vec();

                let mut message = Vec::new();
                message.extend(&reveal_key);
                message.extend(tick_index.to_le_bytes());

                let signature = current_signing.sign(&message);

                input.reveal_pubkey = Some(reveal_key);
                input.signature = Some(signature.to_bytes().to_vec());

                current_signing = new_signing;
            }
        }

        // -----------------------------------------
        // ROTATION 3
        // -----------------------------------------
        if tick_index == 12 {
            let new_signing = SigningKey::generate(&mut OsRng);
            input.commit_pubkey =
                Some(new_signing.verifying_key().to_bytes().to_vec());
            pending_signing = Some(new_signing);
        }

        if tick_index == 14 {
            if let Some(new_signing) = pending_signing.take() {

                let reveal_key = new_signing.verifying_key().to_bytes().to_vec();

                let mut message = Vec::new();
                message.extend(&reveal_key);
                message.extend(tick_index.to_le_bytes());

                let signature = current_signing.sign(&message);

                input.reveal_pubkey = Some(reveal_key);
                input.signature = Some(signature.to_bytes().to_vec());

                current_signing = new_signing;
            }
        }

        apply_tick(&mut state, &input, tick_index)
            .expect("Deterministic transition failure");

        let state_hash = hash_state(&state);

        tick_log.records.push(TickRecord {
            tick_index,
            input: input.clone(),
            state: state.clone(),
            parent_hash: parent_hash.clone(),
            state_hash: state_hash.clone(),
            signature: None,
        });

        parent_hash = state_hash;
    }

    println!(
        "Fluxlock node simulation complete. Generated {} ticks.",
        tick_log.records.len()
    );

    let json = serde_json::to_string_pretty(&tick_log).unwrap();
    std::fs::write("tick_log.json", json).unwrap();
}

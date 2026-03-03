use fluxlock_core::{
    EngineCompositeState,
    TickLog,
    TickRecord,
    TickInput,
};

use fluxlock_engine::{apply_tick, hash_state};

use ed25519_dalek::{SigningKey, VerifyingKey, Signer};
use rand::rngs::OsRng;

fn main() {

    // -------------------------------------------------------
    // Deterministic Initial State
    // -------------------------------------------------------
    let mut state = EngineCompositeState::new();
    let mut tick_log = TickLog::new();
    let mut parent_hash = String::from("GENESIS");

    // Generate initial keypair
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key: VerifyingKey = signing_key.verifying_key();

    // -------------------------------------------------------
    // Tick Simulation
    // -------------------------------------------------------
    for tick_index in 0u64..20u64 {

        let mut input = TickInput {
            revealed_pubkey: None,
            payload: None,
            signature: None,
        };

        // ---------------------------------------------------
        // GENESIS KEY INJECTION (tick 0 only)
        // ---------------------------------------------------
        if tick_index == 0 {
            input.revealed_pubkey = Some(verifying_key.to_bytes().to_vec());
        }

        // ---------------------------------------------------
        // ROTATION AT TICK 5
        // ---------------------------------------------------
        if tick_index == 5 {

            let new_signing = SigningKey::generate(&mut OsRng);
            let new_verifying = new_signing.verifying_key();

            let revealed = new_verifying.to_bytes().to_vec();

            let mut message = Vec::new();
            message.extend(&revealed);
            message.extend(tick_index.to_le_bytes());

            let signature = signing_key.sign(&message);

            input.revealed_pubkey = Some(revealed);
            input.signature = Some(signature.to_bytes().to_vec());
        }

        // ---------------------------------------------------
        // Apply Deterministic Transition
        // ---------------------------------------------------
        apply_tick(&mut state, &input, tick_index)
            .expect("Deterministic transition failure");

        let state_hash = hash_state(&state);

        // ---------------------------------------------------
        // Record Tick
        // ---------------------------------------------------
        tick_log.records.push(TickRecord {
            tick_index,
            input,
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

    // Persist tick log
    std::fs::write(
        "tick_log.json",
        serde_json::to_string_pretty(&tick_log).unwrap(),
    )
    .unwrap();
}

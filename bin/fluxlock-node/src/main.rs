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

    let mut state = EngineCompositeState::new();
    let mut tick_log = TickLog::new();
    let mut parent_hash = String::from("GENESIS");

    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key: VerifyingKey = signing_key.verifying_key();

    let mut next_signing: Option<SigningKey> = None;
    let mut next_verifying: Option<VerifyingKey> = None;

    for tick_index in 0u64..20u64 {

        let mut input = TickInput {
            commit_pubkey: None,
            reveal_pubkey: None,
            payload: None,
            signature: None,
        };

        // -----------------------------
        // GENESIS REVEAL
        // -----------------------------
        if tick_index == 0 {
            input.reveal_pubkey = Some(verifying_key.to_bytes().to_vec());
        }

        // -----------------------------
        // COMMIT AT TICK 4
        // -----------------------------
        if tick_index == 4 {
            let new_signing = SigningKey::generate(&mut OsRng);
            let new_verifying = new_signing.verifying_key();

            next_signing = Some(new_signing);
            next_verifying = Some(new_verifying);

            input.commit_pubkey =
                Some(new_verifying.to_bytes().to_vec());
        }

        // -----------------------------
        // REVEAL AT TICK 5
        // -----------------------------
        if tick_index == 5 {

            let reveal_key =
                next_verifying
                    .as_ref()
                    .expect("Missing next verifying key")
                    .to_bytes()
                    .to_vec();

            let mut message = Vec::new();
            message.extend(&reveal_key);
            message.extend(tick_index.to_le_bytes());

            let signature =
                signing_key.sign(&message);

            input.reveal_pubkey = Some(reveal_key);
            input.signature = Some(signature.to_bytes().to_vec());
        }

        apply_tick(&mut state, &input, tick_index)
            .expect("Deterministic transition failure");

        let state_hash = hash_state(&state);

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

    std::fs::write(
        "tick_log.json",
        serde_json::to_string_pretty(&tick_log).unwrap(),
    ).unwrap();
}

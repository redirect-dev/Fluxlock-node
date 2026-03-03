use ed25519_dalek::{SigningKey, Signer};
use rand::rngs::OsRng;

use fluxlock_core::{EngineCompositeState, TickInput, TickLog, TickRecord};
use fluxlock_engine::{apply_tick, hash_state};

fn main() {
    let mut state = EngineCompositeState::new();
    let mut tick_log = TickLog::new();
    let mut parent_hash = String::from("GENESIS");

    // Current active keys
    let mut current_classical = SigningKey::generate(&mut OsRng);
    let mut current_pq = SigningKey::generate(&mut OsRng);

    // Pending keys
    let mut pending_classical: Option<SigningKey> = None;
    let mut pending_pq: Option<SigningKey> = None;

    for tick_index in 0u64..20u64 {

        let mut input = TickInput {
            commit_classical: None,
            commit_pq: None,
            reveal_classical: None,
            reveal_pq: None,
            classical_signature: None,
            pq_signature: None,
        };

        // ------------------------------------------------
        // Genesis reveal
        // ------------------------------------------------
        if tick_index == 0 {
            input.reveal_classical =
                Some(current_classical.verifying_key().to_bytes().to_vec());

            input.reveal_pq =
                Some(current_pq.verifying_key().to_bytes().to_vec());
        }

        // ------------------------------------------------
        // ROTATION 1
        // ------------------------------------------------
        if tick_index == 4 {
            let new_classical = SigningKey::generate(&mut OsRng);
            let new_pq = SigningKey::generate(&mut OsRng);

            input.commit_classical =
                Some(new_classical.verifying_key().to_bytes().to_vec());

            input.commit_pq =
                Some(new_pq.verifying_key().to_bytes().to_vec());

            pending_classical = Some(new_classical);
            pending_pq = Some(new_pq);
        }

        if tick_index == 6 {
            if let (Some(new_classical), Some(new_pq)) =
                (pending_classical.take(), pending_pq.take())
            {
                let classical_pub =
                    new_classical.verifying_key().to_bytes().to_vec();

                let pq_pub =
                    new_pq.verifying_key().to_bytes().to_vec();

                // Classical signature
                let mut classical_msg = Vec::new();
                classical_msg.extend(&classical_pub);
                classical_msg.extend(tick_index.to_le_bytes());

                let classical_sig =
                    current_classical.sign(&classical_msg);

                // PQ signature
                let mut pq_msg = Vec::new();
                pq_msg.extend(&pq_pub);
                pq_msg.extend(tick_index.to_le_bytes());

                let pq_sig =
                    current_pq.sign(&pq_msg);

                input.reveal_classical = Some(classical_pub);
                input.reveal_pq = Some(pq_pub);

                input.classical_signature =
                    Some(classical_sig.to_bytes().to_vec());

                input.pq_signature =
                    Some(pq_sig.to_bytes().to_vec());

                current_classical = new_classical;
                current_pq = new_pq;
            }
        }

        // ------------------------------------------------
        // ROTATION 2
        // ------------------------------------------------
        if tick_index == 8 {
            let new_classical = SigningKey::generate(&mut OsRng);
            let new_pq = SigningKey::generate(&mut OsRng);

            input.commit_classical =
                Some(new_classical.verifying_key().to_bytes().to_vec());

            input.commit_pq =
                Some(new_pq.verifying_key().to_bytes().to_vec());

            pending_classical = Some(new_classical);
            pending_pq = Some(new_pq);
        }

        if tick_index == 10 {
            if let (Some(new_classical), Some(new_pq)) =
                (pending_classical.take(), pending_pq.take())
            {
                let classical_pub =
                    new_classical.verifying_key().to_bytes().to_vec();

                let pq_pub =
                    new_pq.verifying_key().to_bytes().to_vec();

                let mut classical_msg = Vec::new();
                classical_msg.extend(&classical_pub);
                classical_msg.extend(tick_index.to_le_bytes());

                let classical_sig =
                    current_classical.sign(&classical_msg);

                let mut pq_msg = Vec::new();
                pq_msg.extend(&pq_pub);
                pq_msg.extend(tick_index.to_le_bytes());

                let pq_sig =
                    current_pq.sign(&pq_msg);

                input.reveal_classical = Some(classical_pub);
                input.reveal_pq = Some(pq_pub);

                input.classical_signature =
                    Some(classical_sig.to_bytes().to_vec());

                input.pq_signature =
                    Some(pq_sig.to_bytes().to_vec());

                current_classical = new_classical;
                current_pq = new_pq;
            }
        }

        // ------------------------------------------------
        // ROTATION 3
        // ------------------------------------------------
        if tick_index == 12 {
            let new_classical = SigningKey::generate(&mut OsRng);
            let new_pq = SigningKey::generate(&mut OsRng);

            input.commit_classical =
                Some(new_classical.verifying_key().to_bytes().to_vec());

            input.commit_pq =
                Some(new_pq.verifying_key().to_bytes().to_vec());

            pending_classical = Some(new_classical);
            pending_pq = Some(new_pq);
        }

        if tick_index == 14 {
            if let (Some(new_classical), Some(new_pq)) =
                (pending_classical.take(), pending_pq.take())
            {
                let classical_pub =
                    new_classical.verifying_key().to_bytes().to_vec();

                let pq_pub =
                    new_pq.verifying_key().to_bytes().to_vec();

                let mut classical_msg = Vec::new();
                classical_msg.extend(&classical_pub);
                classical_msg.extend(tick_index.to_le_bytes());

                let classical_sig =
                    current_classical.sign(&classical_msg);

                let mut pq_msg = Vec::new();
                pq_msg.extend(&pq_pub);
                pq_msg.extend(tick_index.to_le_bytes());

                let pq_sig =
                    current_pq.sign(&pq_msg);

                input.reveal_classical = Some(classical_pub);
                input.reveal_pq = Some(pq_pub);

                input.classical_signature =
                    Some(classical_sig.to_bytes().to_vec());

                input.pq_signature =
                    Some(pq_sig.to_bytes().to_vec());

                current_classical = new_classical;
                current_pq = new_pq;
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
        });

        parent_hash = state_hash;
    }

    println!(
        "Fluxlock hybrid simulation complete. Generated {} ticks.",
        tick_log.records.len()
    );

    let json = serde_json::to_string_pretty(&tick_log).unwrap();
    std::fs::write("tick_log.json", json).unwrap();
}

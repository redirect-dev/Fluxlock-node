use fluxlock_protocol::state::account::Account;
use fluxlock_protocol::state::reveal::apply_rotation_reveal;
use fluxlock_protocol::state::validator::Validator;
use fluxlock_protocol::state::event::Event;
use fluxlock_protocol::tx::transaction::RotationRevealTx;
use fluxlock_protocol::pq;

use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
use std::path::PathBuf;

use serde::{Serialize, Deserialize};

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[derive(Serialize, Deserialize)]
struct ValidatorState {
    name: String,
    stake: u128,
    reputation: i32,
}

pub fn run_simulation() {
    println!("🧪 Fluxlock Phase 8B — PERSISTENCE\n");

    let mut all_events: Vec<Event> = vec![];

    // -----------------------------
    // LOAD EXISTING STATE (IF EXISTS)
    // -----------------------------
    let mut state_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    state_path.push("validator_state.json");

    let saved_states: Vec<ValidatorState> = if state_path.exists() {
        let data = fs::read_to_string(&state_path).unwrap();
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        vec![]
    };

    // -----------------------------
    // INITIALIZE VALIDATORS
    // -----------------------------
    let mut validators = vec![];

    let names = vec!["Validator A", "Validator B", "Validator C"];

    for name in names {
        let saved = saved_states.iter().find(|s| s.name == name);

        let (stake, reputation) = if let Some(s) = saved {
            (s.stake, s.reputation)
        } else {
            (1000, 100)
        };

        validators.push((
            name,
            Validator::new(
                stake,
                name.as_bytes().to_vec(),
                name.as_bytes().to_vec(),
                0,
                reputation,
            ),
            vec![],
        ));
    }

    // -----------------------------
    // ACCOUNT SETUP (FRESH EACH RUN)
    // -----------------------------
    let (pq_pk, pq_sk) = pq::generate_keypair();

    let base_account = Account::new(
        1000,
        b"initial_classical".to_vec(),
        pq_pk.clone(),
    );

    let new_classical = b"new_classical_persist".to_vec();
    let (new_pq, _) = pq::generate_keypair();

    let mut acc_template = base_account.clone();

    acc_template.rotation_commitment = Some({
        let mut hasher = blake3::Hasher::new();
        hasher.update(&new_classical);
        hasher.update(&new_pq);
        hasher.finalize().as_bytes().to_vec()
    });

    let timestamp = now();

    let good_tx = RotationRevealTx {
        from: b"initial_classical".to_vec(),
        new_classical_key: new_classical.clone(),
        new_pq_key: new_pq.clone(),
        nonce: 0,
        epoch: 1,
        timestamp,
        link_signature: pq::sign(&new_pq, &pq_sk),
        classical_signature: vec![],
        pq_signature: vec![],
    };

    let bad_tx = RotationRevealTx {
        from: b"initial_classical".to_vec(),
        new_classical_key: new_classical.clone(),
        new_pq_key: new_pq.clone(),
        nonce: 0,
        epoch: 1,
        timestamp,
        link_signature: vec![0; 64],
        classical_signature: vec![],
        pq_signature: vec![],
    };

    // -----------------------------
    // RUN SIMULATION
    // -----------------------------
    for (name, validator, accounts) in validators.iter_mut() {
        accounts.push(acc_template.clone());

        let (events, _) = match *name {
            "Validator A" => apply_rotation_reveal(accounts, validator, &good_tx, name),
            "Validator B" => apply_rotation_reveal(accounts, validator, &bad_tx, name),
            "Validator C" => {
                let mut tampered = good_tx.clone();
                tampered.nonce = 999;
                apply_rotation_reveal(accounts, validator, &tampered, name)
            }
            _ => unreachable!(),
        };

        for e in events {
            all_events.push(e);
        }
    }

    // -----------------------------
    // SAVE STATE
    // -----------------------------
    let new_states: Vec<ValidatorState> = validators
        .iter()
        .map(|(name, v, _)| ValidatorState {
            name: name.to_string(),
            stake: v.stake,
            reputation: v.reputation,
        })
        .collect();

    fs::write(
        &state_path,
        serde_json::to_string_pretty(&new_states).unwrap(),
    )
    .expect("Failed to save validator state");

    // -----------------------------
    // EXPORT UI DATA
    // -----------------------------
    let mut ui_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    ui_path.push("../../fluxlock-ui/public/events.json");

    let validator_states: Vec<_> = validators
        .iter()
        .map(|(name, v, _)| {
            serde_json::json!({
                "name": name,
                "stake": v.stake,
                "reputation": v.reputation
            })
        })
        .collect();

    let json = serde_json::json!({
        "events": all_events,
        "validators": validator_states
    });

    fs::write(&ui_path, serde_json::to_string_pretty(&json).unwrap())
        .expect("Unable to write UI file");

    println!("📁 State persisted + UI updated");
}
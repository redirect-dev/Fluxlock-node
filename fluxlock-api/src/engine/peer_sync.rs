use std::sync::{
    Arc,
    Mutex,
};

use reqwest;

use crate::network_state::NetworkState;

use fluxlock_core::types::{
    PeerAnnouncement,
    Validator,
};

use serde::{
    Serialize,
    Deserialize,
};

// =========================
// 🌐 REMOTE STATE
// =========================
#[derive(
    Clone,
    Serialize,
    Deserialize,
)]
pub struct RemotePeerState {

    pub validators:
        Vec<Validator>,

    pub gossip:
        Vec<PeerAnnouncement>,
}

// =========================
// 🌐 SYNC LOOP
// =========================
pub async fn synchronize_peers(

    state:
        Arc<
            Mutex<NetworkState>
        >,
) {

    loop {

        let peers = {

            let s =
                state
                    .lock()
                    .unwrap();

            s.peer_state
                .peers
                .values()
                .cloned()
                .collect::<Vec<_>>()
        };

        for peer in peers {

            if !peer.active {
                continue;
            }

            let endpoint =
                format!(
                    "{}/peer/state",
                    peer.address
                );

            let response =
                reqwest::get(endpoint)
                    .await;

            let response =
                match response {

                    Ok(r) => r,

                    Err(_) => continue,
                };

            let remote =
                response
                    .json::<RemotePeerState>()
                    .await;

            let remote =
                match remote {

                    Ok(r) => r,

                    Err(_) => continue,
                };

            merge_remote_state(
                state.clone(),
                remote,
            );
        }

        tokio::time::sleep(
            std::time::Duration::from_secs(8)
        )
        .await;
    }
}

// =========================
// 🌐 MERGE STATE
// =========================
fn merge_remote_state(

    state:
        Arc<
            Mutex<NetworkState>
        >,

    remote:
        RemotePeerState,
) {

    let mut state =
        state
            .lock()
            .unwrap();

    // =========================
    // 🌐 MERGE GOSSIP
    // =========================
    for announcement in
        remote.gossip
    {

        let exists =
            state
                .peer_state
                .gossip
                .announcements
                .iter()
                .any(|a| {

                    a.peer_id
                        == announcement.peer_id

                    &&

                    a.epoch
                        == announcement.epoch

                    &&

                    a.validator_id
                        == announcement.validator_id
                });

        if !exists {

            state
                .peer_state
                .push_announcement(
                    announcement
                );
        }
    }

    // =========================
    // 🌐 MERGE VALIDATORS
    // =========================
    for remote_validator in
        remote.validators
    {

        if let Some(local) =
            state
                .validators
                .iter_mut()
                .find(|v| {

                    v.id
                        == remote_validator.id
                })
        {

            // =========================
            // 🧠 TRUST CONVERGENCE
            // =========================
            local.trust =
                (
                    local.trust
                    + remote_validator.trust
                ) / 2.0;

            local.confidence =
                (
                    local.confidence
                    + remote_validator.confidence
                ) / 2.0;

            local.peer_agreement_ratio =
                (
                    local.peer_agreement_ratio
                    + remote_validator.peer_agreement_ratio
                ) / 2.0;

            // =========================
            // 🌐 RECOVERY SYNC
            // =========================
            if remote_validator
                .consensus_failures
                >
                local.consensus_failures
            {

                local.consensus_failures =
                    remote_validator
                        .consensus_failures;
            }

            // =========================
            // 🔗 LONGEST LINEAGE WINS
            // =========================
            if remote_validator
                .identity_chain
                .len()
                >
                local.identity_chain
                    .len()
            {

                local.identity_chain =
                    remote_validator
                        .identity_chain
                        .clone();
            }

            // =========================
            // 🌐 NETWORK ACCEPTANCE
            // =========================
            local.network_accepted =
                remote_validator
                    .network_accepted
                || local
                    .network_accepted;
        }
    }
}
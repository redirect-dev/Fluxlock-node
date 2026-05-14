use serde::{
    Serialize,
    Deserialize,
};

use std::collections::{
    HashMap,
    HashSet,
};

use fluxlock_core::types::{
    PeerNode,
    PeerAnnouncement,
    GossipState,
};

// =========================
// 🌐 PEER MEMORY
// =========================
#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
)]
pub struct PeerState {

    pub local_peer_id: String,

    pub peers:
        HashMap<
            String,
            PeerNode
        >,

    pub gossip:
        GossipState,

    // =========================
    // 🧠 GOSSIP MEMORY
    // =========================
    pub seen_announcements:
        HashSet<String>,

    // =========================
    // 🔗 PEER CONTINUITY
    // =========================
    pub peer_continuity:
        HashMap<
            u32,
            String
        >,
}

impl PeerState {

    // =========================
    // 🌐 INIT
    // =========================
    pub fn new() -> Self {

        Self {

            local_peer_id:
                format!(
                    "peer-{}",
                    rand::random::<u64>()
                ),

            peers:
                HashMap::new(),

            gossip:
                GossipState {

                    announcements:
                        Vec::new(),
                },

            seen_announcements:
                HashSet::new(),

            peer_continuity:
                HashMap::new(),
        }
    }

    // =========================
    // 📡 REGISTER PEER
    // =========================
    pub fn register_peer(
        &mut self,
        peer: PeerNode,
    ) {

        self.peers.insert(
            peer.peer_id.clone(),
            peer,
        );
    }

    // =========================
    // 💓 HEARTBEAT
    // =========================
    pub fn heartbeat(
        &mut self,
        peer_id: &str,
        epoch: u64,
    ) {

        if let Some(peer) =
            self.peers.get_mut(peer_id)
        {

            peer.last_seen_epoch =
                epoch;

            peer.active = true;
        }
    }

    // =========================
    // 💤 STALE DETECTION
    // =========================
    pub fn detect_stale_peers(
        &mut self,
        current_epoch: u64,
    ) {

        for peer in
            self.peers.values_mut()
        {

            let idle =
                current_epoch
                    .saturating_sub(
                        peer.last_seen_epoch
                    );

            if idle > 300 {

                peer.active = false;
            }
        }
    }

    // =========================
    // 📡 GOSSIP BROADCAST
    // =========================
    pub fn push_announcement(
        &mut self,
        announcement:
            PeerAnnouncement,
    ) {

        let fingerprint =
            format!(
                "{}-{}-{}",
                announcement.validator_id,
                announcement.epoch,
                announcement.continuity_hash
            );

        // =========================
        // 🚫 DUPLICATE FILTER
        // =========================
        if self
            .seen_announcements
            .contains(
                &fingerprint
            )
        {
            return;
        }

        self
            .seen_announcements
            .insert(
                fingerprint
            );

        // =========================
        // 🔗 TRACK CONTINUITY
        // =========================
        self.peer_continuity.insert(
            announcement.validator_id,
            announcement
                .continuity_hash
                .clone(),
        );

        self.gossip
            .announcements
            .push(
                announcement
            );

        // =========================
        // 🧹 MEMORY LIMIT
        // =========================
        if self.gossip
            .announcements
            .len() > 512
        {

            self.gossip
                .announcements
                .remove(0);
        }
    }

    // =========================
    // 🔍 CONTINUITY CHECK
    // =========================
    pub fn continuity_conflict(
        &self,
        validator_id: u32,
        continuity_hash: &str,
    ) -> bool {

        if let Some(existing) =
            self.peer_continuity.get(
                &validator_id
            )
        {

            return existing
                != continuity_hash;
        }

        false
    }
}
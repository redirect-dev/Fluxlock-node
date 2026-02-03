use std::env;

const MAX_TICKS: u32 = 30;

// ─────────────────────────────────────────────
// Network / Security Parameters
// ─────────────────────────────────────────────
const TRUST_DECAY: f64 = 0.04;
const TRUST_FLOOR: f64 = 0.0;

const ACTIVE_THRESHOLD: f64 = 0.55;
const DEGRADED_THRESHOLD: f64 = 0.35;

// ─────────────────────────────────────────────
// Enums
// ─────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq)]
enum NodeState {
    Active,
    Degraded,
    Quarantined,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum LockState {
    Unlocked,
    Restricted,
    Locked,
}

// ─────────────────────────────────────────────
// Node Struct
// ─────────────────────────────────────────────
struct Node {
    id: String,
    trust: f64,
    key_age: u32,
    state: NodeState,
}

impl Node {
    fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            trust: 0.80,
            key_age: 0,
            state: NodeState::Active,
        }
    }

    fn should_rotate(&self) -> bool {
        self.key_age >= 2
    }

    fn rotate_key(&mut self) -> bool {
        if self.should_rotate() {
            self.key_age = 0;
            true
        } else {
            self.key_age += 1;
            false
        }
    }

    fn update_trust(&mut self) {
        self.trust -= TRUST_DECAY;
        if self.trust < TRUST_FLOOR {
            self.trust = TRUST_FLOOR;
        }

        self.state = if self.trust >= ACTIVE_THRESHOLD {
            NodeState::Active
        } else if self.trust >= DEGRADED_THRESHOLD {
            NodeState::Degraded
        } else {
            NodeState::Quarantined
        };
    }
}

// ─────────────────────────────────────────────
// Network Logic
// ─────────────────────────────────────────────
fn evaluate_lock_state(nodes: &[Node]) -> LockState {
    let total = nodes.len() as f64;

    let avg_trust: f64 = nodes.iter().map(|n| n.trust).sum::<f64>() / total;
    let quarantined = nodes
        .iter()
        .filter(|n| n.state == NodeState::Quarantined)
        .count() as f64;

    if avg_trust < 0.35 || quarantined / total >= 0.40 {
        LockState::Locked
    } else if avg_trust < 0.55 {
        LockState::Restricted
    } else {
        LockState::Unlocked
    }
}

// ─────────────────────────────────────────────
// Main Simulation
// ─────────────────────────────────────────────
fn main() {
    let args: Vec<String> = env::args().collect();
    let node_id = args.get(1).map(|s| s.as_str()).unwrap_or("node1");

    println!("Starting Phase 29-A (Soft Lock) for {}", node_id);

    let mut nodes = vec![
        Node::new("node1"),
        Node::new("node2"),
        Node::new("node3"),
        Node::new("node4"),
        Node::new("node5"),
    ];

    let mut lock_state = LockState::Unlocked;

    for tick in 0..MAX_TICKS {
        println!("\n=== TICK {} ===", tick);

        for node in nodes.iter_mut() {
            let rotated = node.rotate_key();
            node.update_trust();

            println!(
                "{} | trust {:.3} | key_age {:2} | rotated {:5} | state {:?}",
                node.id, node.trust, node.key_age, rotated, node.state
            );
        }

        let new_lock_state = evaluate_lock_state(&nodes);

        if new_lock_state != lock_state {
            println!(
                "\n[NETWORK] LOCK STATE CHANGE: {:?} → {:?}",
                lock_state, new_lock_state
            );
            if new_lock_state == LockState::Locked {
                println!("[NETWORK] SOFT LOCK ENGAGED — keys continue rotating");
            }
            lock_state = new_lock_state;
        }
    }

    println!("\nPhase 29-A complete.");
}

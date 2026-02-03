use std::collections::HashMap;
use std::env;

const TRUST_DECAY: f64 = 0.04;
const RECOVERY_INCREMENT: f64 = 0.02;
const LOCKED_RECOVERY_FACTOR: f64 = 0.1;

#[derive(Clone, Copy, PartialEq)]
enum NodeState {
    Active,
    Degraded,
    Quarantined,
}

#[derive(Clone, Copy, PartialEq)]
enum LockState {
    Unlocked,
    Restricted,
    Locked,
}

#[derive(Clone, Copy)]
enum LockPolicy {
    Irreversible,   // Hard security vault
    Probationary,   // Slow recovery allowed
}

struct Node {
    name: String,
    trust: f64,
    key_age: u32,
    state: NodeState,
}

impl Node {
    fn rotate_key(&mut self) -> bool {
        if self.key_age >= 2 {
            self.key_age = 0;
            true
        } else {
            self.key_age += 1;
            false
        }
    }

    fn update_state(&mut self) {
        self.state = if self.trust >= 0.55 {
            NodeState::Active
        } else if self.trust >= 0.35 {
            NodeState::Degraded
        } else {
            NodeState::Quarantined
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let node_name = args.get(1).cloned().unwrap_or_else(|| "node1".to_string());

    // 🔧 CHANGE POLICY HERE
    let lock_policy = LockPolicy::Probationary;
    // let lock_policy = LockPolicy::Irreversible;

    println!(
        "Starting Phase 29-C (Policy-Split Soft Lock) for {}",
        node_name
    );

    let mut nodes: HashMap<String, Node> = (1..=5)
        .map(|i| {
            let name = format!("node{}", i);
            (
                name.clone(),
                Node {
                    name,
                    trust: 0.76,
                    key_age: 0,
                    state: NodeState::Active,
                },
            )
        })
        .collect();

    let mut lock_state = LockState::Unlocked;
    let mut recovery_windows = 0;

    for tick in 0..30 {
        println!("\n=== TICK {} ===", tick);

        // 🔻 Trust decay (always)
        for node in nodes.values_mut() {
            node.trust = (node.trust - TRUST_DECAY).max(0.0);
            let rotated = node.rotate_key();
            node.update_state();

            println!(
                "{} | trust {:.3} | key_age {:2} | rotated {:5} | state {:?}",
                node.name, node.trust, node.key_age, rotated, node.state
            );
        }

        // 🔐 Lock transitions
        let degraded = nodes.values().filter(|n| n.state != NodeState::Active).count();

        match lock_state {
            LockState::Unlocked if degraded >= 3 => {
                lock_state = LockState::Restricted;
                println!("\n[NETWORK] LOCK STATE CHANGE: Unlocked → Restricted");
            }
            LockState::Restricted if degraded >= 4 => {
                lock_state = LockState::Locked;
                println!("\n[NETWORK] LOCK STATE CHANGE: Restricted → Locked");
                println!("[NETWORK] SOFT LOCK ENGAGED — keys continue rotating");
            }
            _ => {}
        }

        // 🩹 Recovery logic (policy dependent)
        if lock_state == LockState::Locked {
            match lock_policy {
                LockPolicy::Irreversible => {
                    // ❌ no recovery allowed
                }
                LockPolicy::Probationary => {
                    for node in nodes.values_mut() {
                        if node.state != NodeState::Active {
                            node.trust = (node.trust + RECOVERY_INCREMENT * LOCKED_RECOVERY_FACTOR)
                                .min(1.0);
                            node.update_state();
                        }
                    }
                }
            }
        } else {
            for node in nodes.values_mut() {
                if node.state != NodeState::Active {
                    node.trust = (node.trust + RECOVERY_INCREMENT).min(1.0);
                    node.update_state();
                }
            }
        }

        // 🔓 Exit lock (only if policy allows)
        let healthy = nodes.values().filter(|n| n.state == NodeState::Active).count();
        if lock_state == LockState::Locked && healthy >= 4 {
            recovery_windows += 1;
            if recovery_windows >= 3 {
                lock_state = LockState::Restricted;
                recovery_windows = 0;
                println!("\n[NETWORK] LOCK RELAXED → Restricted");
            }
        }
    }

    println!("\nPhase 29-C complete.");
}

use std::env;

#[derive(Debug, Clone, PartialEq)]
enum NodeState {
    Active,
    Degraded,
    Quarantined,
}

const MAX_TICKS: u32 = 40;

// Trust thresholds
const DEGRADED_THRESHOLD: f64 = 0.50;
const QUARANTINE_THRESHOLD: f64 = 0.35;
const RECOVERY_THRESHOLD: f64 = 0.55;

// Behavior parameters
const TRUST_DECAY: f64 = 0.04;
const TRUST_REWARD: f64 = 0.02;
const ROTATION_INTERVAL: u32 = 2;

// Proof-of-behavior requirement
const REQUIRED_GOOD_BEHAVIOR: u32 = 10;

#[derive(Debug)]
struct Node {
    name: String,
    trust: f64,
    key_age: u32,
    rotated: bool,
    state: NodeState,
    good_behavior_streak: u32,
}

impl Node {
    fn new(name: &str) -> Self {
        Node {
            name: name.to_string(),
            trust: 0.80,
            key_age: 0,
            rotated: false,
            state: NodeState::Active,
            good_behavior_streak: 0,
        }
    }

    fn should_rotate(&self) -> bool {
        self.key_age >= ROTATION_INTERVAL
    }

    fn rotate_key(&mut self) {
        self.key_age = 0;
        self.rotated = true;
        self.trust = (self.trust + TRUST_REWARD).min(1.0);
    }

    fn decay_trust(&mut self) {
        self.trust = (self.trust - TRUST_DECAY).max(0.0);
    }

    fn update_state(&mut self) {
        self.state = if self.trust < QUARANTINE_THRESHOLD {
            NodeState::Quarantined
        } else if self.trust < DEGRADED_THRESHOLD {
            NodeState::Degraded
        } else {
            NodeState::Active
        };
    }

    fn attempt_recovery(&mut self) {
        if self.state == NodeState::Quarantined {
            println!(
                "{} attempting recovery (good_behavior_streak = {})",
                self.name, self.good_behavior_streak
            );

            if self.good_behavior_streak >= REQUIRED_GOOD_BEHAVIOR {
                self.trust = (self.trust + 0.10).min(1.0);
                self.state = if self.trust >= RECOVERY_THRESHOLD {
                    NodeState::Active
                } else {
                    NodeState::Degraded
                };

                println!(
                    "{} partial recovery → trust {:.3}, state {:?}",
                    self.name, self.trust, self.state
                );
            } else {
                println!("{} recovery denied (insufficient proof)", self.name);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let node_name = args.get(1).unwrap_or(&"node".to_string()).clone();

    let mut node = Node::new(&node_name);

    println!(
        "Starting Phase 28-2B (Balanced Recovery w/ Proof of Good Behavior) for {}",
        node.name
    );

    for tick in 0..MAX_TICKS {
        node.rotated = false;

        // --- Key aging ---
        node.key_age += 1;

        // --- Key rotation ---
        if node.should_rotate() {
            node.rotate_key();
        }

        // --- Trust decay ---
        let previous_trust = node.trust;
        node.decay_trust();

        // --- GOOD BEHAVIOR LOGIC (CRITICAL FIX) ---
        // Good behavior only counts if trust is improving
        if node.trust > previous_trust && node.state != NodeState::Quarantined {
            node.good_behavior_streak += 1;
        } else {
            node.good_behavior_streak = 0;
        }

        // --- Update node state ---
        node.update_state();

        // --- Attempt recovery if quarantined ---
        if node.state == NodeState::Quarantined {
            println!("{} QUARANTINED at tick {}", node.name, tick);
            node.good_behavior_streak = 0; // force fresh proof
            node.attempt_recovery();
        }

        println!(
            "{} | tick {:>2} | trust {:.3} | key_age {:>2} | rotated {:<5} | good {:>2} | state {:?}",
            node.name,
            tick,
            node.trust,
            node.key_age,
            node.rotated,
            node.good_behavior_streak,
            node.state
        );
    }

    println!("{} FINISHED", node.name);
}

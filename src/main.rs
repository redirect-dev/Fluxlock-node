use std::env;

#[derive(Debug, Clone, PartialEq)]
enum NodeState {
    Active,
    Quarantined,
}

struct Node {
    id: String,
    trust: f64,
    key_age: u32,
    state: NodeState,
    late_rotation_streak: u32,
}

impl Node {
    fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            trust: 0.80,
            key_age: 0,
            state: NodeState::Active,
            late_rotation_streak: 0,
        }
    }

    /// Honest nodes rotate every 2 ticks
    /// Malicious nodes delay rotation
    fn should_rotate(&self, tick: u32) -> bool {
        if self.id.contains("3") {
            // attacker rotates very late
            self.key_age >= 10
        } else {
            // honest behavior
            tick % 2 == 1
        }
    }

    fn rotate_key(&mut self) {
        self.key_age = 0;
        self.late_rotation_streak = 0;
        self.trust = (self.trust + 0.05).min(1.0);
    }

    fn apply_trust_dynamics(&mut self) {
        if self.key_age <= 2 {
            // compliant behavior → protect trust
            self.trust = (self.trust + 0.01).min(1.0);
        } else {
            // late rotation → increasing penalty
            self.late_rotation_streak += 1;
            let decay = 0.04 + (self.late_rotation_streak as f64 * 0.02);
            self.trust = (self.trust - decay).max(0.0);
        }
    }

    fn evaluate_state(&mut self) {
        // Quarantine requires BOTH:
        // 1. Low trust
        // 2. Sustained late rotation
        if self.trust < 0.25 && self.late_rotation_streak >= 3 {
            self.state = NodeState::Quarantined;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let node_id = args.get(1).unwrap_or(&"node1".to_string()).clone();

    let mut node = Node::new(&node_id);

    for tick in 0..20 {
        if node.state == NodeState::Quarantined {
            println!(
                "{} QUARANTINED at tick {}",
                node.id, tick
            );
            break;
        }

        node.key_age += 1;

        let rotated = if node.should_rotate(tick) {
            node.rotate_key();
            true
        } else {
            false
        };

        node.apply_trust_dynamics();
        node.evaluate_state();

        println!(
            "{} | tick {} | trust {:.3} | key_age {} | rotated {} | state {:?}",
            node.id,
            tick,
            node.trust,
            node.key_age,
            rotated,
            node.state
        );
    }

    println!("{} FINISHED", node.id);
}

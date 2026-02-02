use std::env;

#[derive(Debug, Clone, PartialEq)]
enum NodeState {
    Active,
    Degraded,
    Quarantined,
}

struct Node {
    name: String,
    trust: f64,
    key_age: u32,
    alignment_streak: u32,
    state: NodeState,
}

impl Node {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            trust: 0.80,
            key_age: 0,
            alignment_streak: 0,
            state: NodeState::Active,
        }
    }

    fn should_rotate(&self) -> bool {
        self.key_age >= 2
    }

    fn decide(&self, tick: u32) -> u8 {
        // Honest nodes alternate predictably
        if self.name != "node3" {
            (tick % 2) as u8
        } else {
            // Node3 is adversarial / unstable
            if tick % 3 == 0 { 1 } else { 0 }
        }
    }

    fn update_state(&mut self) {
        self.state = if self.trust >= 0.6 {
            NodeState::Active
        } else if self.trust >= 0.3 {
            NodeState::Degraded
        } else {
            NodeState::Quarantined
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let node_name = args.get(1).expect("Provide node name");

    let mut node = Node::new(node_name);

    println!("Starting Phase 15B for {}", node.name);

    let recovery_threshold: u32 = 5;
    let recovery_rate: f64 = 0.05;
    let penalty: f64 = 0.08;

    for tick in 0..20 {
        if node.state == NodeState::Quarantined {
            println!(
                "{} QUARANTINED at tick {}",
                node.name, tick
            );
            break;
        }

        let decision = node.decide(tick);
        let consensus = (tick % 2) as u8;

        // Alignment logic
        if decision == consensus {
            node.alignment_streak += 1;
        } else {
            node.alignment_streak = 0;
            node.trust -= penalty;
        }

        // Recovery gate
        if node.alignment_streak >= recovery_threshold {
            node.trust += recovery_rate;
            node.alignment_streak = 0;
        }

        // Clamp trust
        if node.trust > 1.0 {
            node.trust = 1.0;
        }
        if node.trust < 0.0 {
            node.trust = 0.0;
        }

        // Key rotation
        let rotated = node.should_rotate();
        if rotated {
            node.key_age = 0;
        } else {
            node.key_age += 1;
        }

        node.update_state();

        println!(
            "{} | tick {} | trust {:.3} | streak {} | key_age {} | rotated {} | state {:?}",
            node.name,
            tick,
            node.trust,
            node.alignment_streak,
            node.key_age,
            rotated,
            node.state
        );
    }

    println!("{} FINISHED", node.name);
}

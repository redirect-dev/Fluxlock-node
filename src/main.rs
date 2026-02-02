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
    state: NodeState,
}

impl Node {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            trust: 0.80,
            key_age: 0,
            state: NodeState::Active,
        }
    }

    fn should_rotate(&self) -> bool {
        // Honest nodes rotate frequently, malicious ones don't
        !self.name.contains("node3") && self.key_age >= 2
    }

    fn apply_rotation(&mut self) -> bool {
        if self.should_rotate() {
            self.key_age = 0;
            self.trust = (self.trust + 0.05).min(1.0);
            true
        } else {
            false
        }
    }

    fn apply_trust_decay(&mut self) {
        // Progressive penalties
        match self.key_age {
            0..=2 => {} // no penalty
            3..=5 => self.trust -= 0.05,
            6..=8 => self.trust -= 0.08,
            _ => self.trust -= 0.12,
        }

        if self.trust < 0.0 {
            self.trust = 0.0;
        }
    }

    fn update_state(&mut self) {
        self.state = if self.trust < 0.20 {
            NodeState::Quarantined
        } else if self.trust < 0.50 {
            NodeState::Degraded
        } else {
            NodeState::Active
        };
    }

    fn tick(&mut self, tick: u32) -> bool {
        self.key_age += 1;

        let rotated = self.apply_rotation();
        if !rotated {
            self.apply_trust_decay();
        }

        self.update_state();

        println!(
            "{} | tick {} | trust {:.3} | key_age {} | rotated {} | state {:?}",
            self.name, tick, self.trust, self.key_age, rotated, self.state
        );

        self.state == NodeState::Quarantined
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let node_name = args.get(1).unwrap_or(&"node1".to_string()).clone();

    let mut node = Node::new(&node_name);

    for tick in 0..20 {
        let quarantined = node.tick(tick);

        if quarantined {
            println!("{} QUARANTINED at tick {}", node.name, tick);
            break;
        }
    }

    println!("{} FINISHED", node.name);
}

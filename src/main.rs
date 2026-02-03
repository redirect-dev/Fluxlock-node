use std::env;

#[derive(Debug, Clone, Copy, PartialEq)]
enum NodeState {
    Active,
    Degraded,
    Quarantined,
}

struct Node {
    name: String,
    trust: f64,
    key_age: u32,
    effort: f64,

    // Conservative recovery controls
    proof_counter: u32,          // proof-of-good-behavior counter
    max_recoverable_trust: f64,  // recovery cap by state
    state: NodeState,
}

impl Node {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            trust: 0.80,
            key_age: 0,
            effort: 1.0,
            proof_counter: 0,
            max_recoverable_trust: 1.0,
            state: NodeState::Active,
        }
    }

    fn should_rotate(&self) -> bool {
        self.key_age >= 2
    }

    fn evaluate_state(&mut self) {
        self.state = if self.trust < 0.25 {
            NodeState::Quarantined
        } else if self.trust < 0.50 {
            NodeState::Degraded
        } else {
            NodeState::Active
        };

        // Conservative recovery ceilings
        self.max_recoverable_trust = match self.state {
            NodeState::Quarantined => 0.60,
            NodeState::Degraded => 0.80,
            NodeState::Active => 1.00,
        };
    }

    fn tick(&mut self, tick: u32) -> bool {
        // Simulated behavior:
        // even ticks = good behavior, odd ticks = noisy environment
        let good_behavior = tick % 2 == 0;

        // Adversary effort always rises over time
        self.effort *= 1.12;

        let rotated = if self.should_rotate() {
            self.key_age = 0;
            true
        } else {
            self.key_age += 1;
            false
        };

        if good_behavior && rotated {
            // Conservative recovery path
            self.proof_counter += 1;

            if self.proof_counter >= 5 {
                // Slow recovery
                self.trust += 0.025;
            }
        } else {
            // Fast decay on any failure
            self.proof_counter = 0;
            self.trust -= 0.08;
        }

        self.evaluate_state();

        // Clamp trust conservatively
        if self.trust > self.max_recoverable_trust {
            self.trust = self.max_recoverable_trust;
        }
        if self.trust < 0.0 {
            self.trust = 0.0;
        }

        println!(
            "{} | tick {:>2} | trust {:.3} | key_age {:>2} | effort {:.2} | rotated {:<5} | state {:?}",
            self.name, tick, self.trust, self.key_age, self.effort, rotated, self.state
        );

        self.state == NodeState::Quarantined
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let node_name = args.get(1).map(String::as_str).unwrap_or("node");

    println!("Starting Phase 27-2 (Conservative Trust Recovery) for {}", node_name);

    let mut node = Node::new(node_name);

    for tick in 0..30 {
        let quarantined = node.tick(tick);
        if quarantined {
            println!("{} QUARANTINED at tick {}", node.name, tick);
            break;
        }
    }

    println!("{} FINISHED", node.name);
}

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
    attacker_effort: f64,
    state: NodeState,
}

impl Node {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            trust: 0.80,
            key_age: 0,
            attacker_effort: 1.0,
            state: NodeState::Active,
        }
    }

    fn should_rotate_key(&self) -> bool {
        self.key_age >= 2
    }

    fn rotate_key(&mut self) {
        self.key_age = 0;
        // Rotation slightly increases attack cost
        self.attacker_effort *= 1.15;
    }

    fn apply_attack(&mut self, base_attack: f64) {
        // Adversary fatigue grows with key age
        let fatigue_factor = 0.12;
        self.attacker_effort += self.key_age as f64 * fatigue_factor;

        // Diminishing returns on repeated attacks
        let effective_attack = base_attack / self.attacker_effort;

        // Non-linear trust damage (super-linear)
        let damage = effective_attack.powf(1.3);

        self.trust -= damage;
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

        let mut rotated = false;
        if self.should_rotate_key() {
            self.rotate_key();
            rotated = true;
        }

        // Simulated adversarial pressure
        let base_attack = if tick % 2 == 0 { 0.08 } else { 0.10 };
        self.apply_attack(base_attack);

        self.update_state();

        println!(
            "{} | tick {} | trust {:.3} | key_age {} | effort {:.2} | rotated {} | state {:?}",
            self.name,
            tick,
            self.trust,
            self.key_age,
            self.attacker_effort,
            rotated,
            self.state
        );

        self.state != NodeState::Quarantined
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let node_name = args.get(1).cloned().unwrap_or("node".to_string());

    let mut node = Node::new(&node_name);

    println!("Starting Phase 26-3 (Adversary Fatigue) for {}", node.name);

    for tick in 0..30 {
        if !node.tick(tick) {
            println!("{} QUARANTINED at tick {}", node.name, tick);
            break;
        }
    }

    println!("{} FINISHED", node.name);
}

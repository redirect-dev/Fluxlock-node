use std::env;

#[derive(Clone, Debug)]
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

struct Attacker {
    cost: f64,
    efficiency: f64, // decays toward zero
}

impl Node {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            trust: 0.8,
            key_age: 0,
            state: NodeState::Active,
        }
    }

    fn should_rotate(&self) -> bool {
        self.key_age >= 2 || self.trust < 0.6
    }

    fn rotate_key(&mut self) {
        self.key_age = 0;
        self.trust = (self.trust + 0.05).min(1.0);
    }

    fn degrade(&mut self) {
        self.trust -= 0.08;
    }

    fn update_state(&mut self) {
        self.state = if self.trust < 0.2 {
            NodeState::Quarantined
        } else if self.trust < 0.5 {
            NodeState::Degraded
        } else {
            NodeState::Active
        };
    }
}

impl Attacker {
    fn new() -> Self {
        Self {
            cost: 0.0,
            efficiency: 1.0,
        }
    }

    fn attempt_attack(&mut self, node: &Node) -> bool {
        let base_cost = 1.0;
        let age_factor = 1.0 + (node.key_age as f64 * 0.3);
        let efficiency_penalty = 1.0 / self.efficiency.max(0.1);

        self.cost += base_cost * age_factor * efficiency_penalty;
        self.efficiency -= 0.05;

        // Attack success probability
        let success_chance = self.efficiency * (1.0 - node.trust);
        success_chance > 0.3
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let node_name = args.get(1).expect("Provide node name");

    let mut node = Node::new(node_name);
    let mut attacker = Attacker::new();

    println!("Starting Phase 26-2 for {}", node.name);

    for tick in 0..30 {
        if let NodeState::Quarantined = node.state {
            println!("{} already quarantined — attacker wasting resources", node.name);
            attacker.cost += 2.0;
            continue;
        }

        node.key_age += 1;

        let rotated = if node.should_rotate() {
            node.rotate_key();
            true
        } else {
            false
        };

        let attack_success = attacker.attempt_attack(&node);

        if attack_success {
            node.degrade();
        }

        node.update_state();

        println!(
            "{} | tick {} | trust {:.3} | key_age {} | rotated {} | state {:?} | attacker_cost {:.2} | attacker_eff {:.2} | attack_success {}",
            node.name,
            tick,
            node.trust,
            node.key_age,
            rotated,
            node.state,
            attacker.cost,
            attacker.efficiency.max(0.0),
            attack_success
        );

        if let NodeState::Quarantined = node.state {
            println!(
                "{} QUARANTINED — attacker cost {:.2}, efficiency {:.2}",
                node.name,
                attacker.cost,
                attacker.efficiency.max(0.0)
            );
            break;
        }
    }

    println!(
        "{} FINISHED — total attacker cost {:.2}, final efficiency {:.2}",
        node.name,
        attacker.cost,
        attacker.efficiency.max(0.0)
    );
}

fn main() {
    println!("🧪 CLEAN SIMULATION ENTRY\n");
    run_simulation();
}

#[derive(Clone, Debug)]
struct Validator {
    name: String,
    stake: u128,
    reputation: i32,
    trust_penalty: i32,
    valid_identity: bool,
}

impl Validator {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            stake: 1000,
            reputation: 100,
            trust_penalty: 0,
            valid_identity: true,
        }
    }

    fn effective_reputation(&self) -> i32 {
        self.reputation - self.trust_penalty
    }

    fn influence(&self) -> f64 {
        if !self.valid_identity {
            return 0.0;
        }
        let stake_weight = self.stake as f64 / 1000.0;
        (self.effective_reputation() as f64) * stake_weight
    }
}

fn run_simulation() {
    println!("🧪 Fluxlock Phase 20 — Fork Resolution\n");

    let mut validators = vec![
        Validator::new("Validator A (Honest)"),
        Validator::new("Validator B (Broken)"),
        Validator::new("Validator C (Aggressive)"),
    ];

    // Identity validation
    println!("⚡ Identity Validation\n");
    for v in validators.iter_mut() {
        if v.name.contains("Broken") {
            v.valid_identity = false;
            println!("❌ {} INVALID", v.name);
        } else {
            println!("✅ {} VALID", v.name);
        }
    }

    // Behavior shaping
    println!("\n🌱 Behavior Phase\n");

    for round in 0..3 {
        println!("--- Round {} ---", round + 1);

        for v in validators.iter_mut() {
            if !v.valid_identity {
                continue;
            }

            if v.name.contains("Honest") {
                v.reputation += 5;
            }

            if v.name.contains("Aggressive") && round < 2 {
                v.trust_penalty += 10;
                v.reputation -= 5;
            }

            println!(
                "{} → eff_rep: {} | influence: {:.2}",
                v.name,
                v.effective_reputation(),
                v.influence()
            );
        }
    }

    // Fork simulation
    println!("\n🌐 Fork Simulation\n");

    let mut chain_valid_weight = 0.0;
    let mut chain_invalid_weight = 0.0;

    for v in validators.iter() {
        if !v.valid_identity {
            continue;
        }

        if v.name.contains("Honest") {
            println!("{} builds VALID chain", v.name);
            chain_valid_weight += v.influence();
        } else if v.name.contains("Aggressive") {
            println!("{} builds INVALID chain", v.name);
            chain_invalid_weight += v.influence();
        }
    }

    println!("\n⚖️ Chain Weights:");
    println!("VALID chain weight: {:.2}", chain_valid_weight);
    println!("INVALID chain weight: {:.2}", chain_invalid_weight);

    println!("\n🏆 FINAL CHAIN:");

    if chain_valid_weight > chain_invalid_weight {
        println!("✔ VALID chain survives");
    } else {
        println!("❌ INVALID chain survives");
    }

    println!("\n✅ Phase 20 Complete");
}
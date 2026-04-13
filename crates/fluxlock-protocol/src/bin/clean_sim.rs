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
    println!("🧪 Fluxlock Phase 23 — Slow Corruption Attack\n");

    let mut validators = vec![
        Validator::new("Validator A (Honest)"),
        Validator::new("Validator C (Slow Corruptor)"),
    ];

    println!("⚡ Identity Validation\n");
    for v in validators.iter() {
        println!("✅ {} VALID", v.name);
    }

    println!("\n🌱 Trust Building Phase\n");

    // Build trust equally first
    for round in 0..3 {
        println!("--- Build Round {} ---", round + 1);

        for v in validators.iter_mut() {
            v.reputation += 5;

            println!(
                "{} → eff_rep: {} | influence: {:.2}",
                v.name,
                v.effective_reputation(),
                v.influence()
            );
        }
    }

    println!("\n🐍 Slow Corruption Phase\n");

    // Subtle degradation (no big penalties)
    for round in 0..5 {
        println!("--- Corruption Round {} ---", round + 1);

        for v in validators.iter_mut() {
            if v.name.contains("Slow Corruptor") {
                // small degradation each round
                v.trust_penalty += 3;
                v.reputation -= 2;
            } else {
                v.reputation += 3;
            }

            println!(
                "{} → eff_rep: {} | influence: {:.2}",
                v.name,
                v.effective_reputation(),
                v.influence()
            );
        }
    }

    println!("\n⚔️ Conflict After Slow Corruption\n");

    let mut honest_weight = 0.0;
    let mut corruptor_weight = 0.0;

    for v in validators.iter() {
        if v.name.contains("Honest") {
            honest_weight += v.influence();
        } else {
            corruptor_weight += v.influence();
        }
    }

    println!("Honest weight: {:.2}", honest_weight);
    println!("Corruptor weight: {:.2}", corruptor_weight);

    println!("\n🏆 FINAL RESULT:");

    if honest_weight > corruptor_weight {
        println!("✔ Honest validator resists slow corruption");
    } else {
        println!("❌ Slow corruption attack succeeds");
    }

    println!("\n✅ Phase 23 Complete");
}
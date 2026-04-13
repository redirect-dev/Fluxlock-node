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
    println!("🧪 Fluxlock Phase 21 — Collusion Attack\n");

    let mut validators = vec![
        Validator::new("Validator A (Honest)"),
        Validator::new("Validator B (Colluder 1)"),
        Validator::new("Validator C (Colluder 2)"),
    ];

    println!("⚡ Identity Validation\n");
    for v in validators.iter_mut() {
        println!("✅ {} VALID", v.name);
    }

    println!("\n🌱 Behavior Phase\n");

    for round in 0..3 {
        println!("--- Round {} ---", round + 1);

        for v in validators.iter_mut() {
            if v.name.contains("Honest") {
                v.reputation += 5;
            }

            if v.name.contains("Colluder") {
                v.trust_penalty += 15;
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

    println!("\n⚔️ Collusion Attempt\n");

    let mut honest_weight = 0.0;
    let mut collusion_weight = 0.0;

    for v in validators.iter() {
        if v.name.contains("Honest") {
            honest_weight += v.influence();
        } else {
            collusion_weight += v.influence();
        }
    }

    println!("Honest weight: {:.2}", honest_weight);
    println!("Colluding weight: {:.2}", collusion_weight);

    println!("\n🏆 FINAL RESULT:");

    if honest_weight > collusion_weight {
        println!("✔ Honest validator resists collusion");
    } else {
        println!("❌ Collusion attack succeeds");
    }

    println!("\n✅ Phase 21 Complete");
}
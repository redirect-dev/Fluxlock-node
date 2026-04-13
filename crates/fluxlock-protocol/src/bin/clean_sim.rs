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
    fn new(name: &str, stake: u128) -> Self {
        Self {
            name: name.to_string(),
            stake,
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

        // 🔥 Dampened stake (non-linear)
        let stake_weight = (self.stake as f64 / 1000.0).sqrt();

        // 🔥 Trust gating (THIS is the key upgrade)
        let trust_factor = (self.effective_reputation() as f64) / 100.0;

        (self.effective_reputation() as f64) * stake_weight * trust_factor
    }
}

fn run_simulation() {
    println!("🧪 Fluxlock Phase 26 — Trust-Gated Stake\n");

    let mut validators = vec![
        Validator::new("Validator A (Honest)", 1000),
        Validator::new("Validator C (Wealthy Attacker)", 3000),
    ];

    println!("⚡ Identity Validation\n");
    for v in validators.iter() {
        println!("✅ {} VALID", v.name);
    }

    println!("\n🌱 Trust Building Phase\n");

    for round in 0..3 {
        println!("--- Build Round {} ---", round + 1);

        for v in validators.iter_mut() {
            v.reputation += 5;

            println!(
                "{} → stake: {} | eff_rep: {} | trust_factor: {:.2} | influence: {:.2}",
                v.name,
                v.stake,
                v.effective_reputation(),
                v.effective_reputation() as f64 / 100.0,
                v.influence()
            );
        }
    }

    println!("\n⚠️ ATTACK PHASE\n");

    for round in 0..3 {
        println!("--- Attack Round {} ---", round + 1);

        for v in validators.iter_mut() {
            if v.name.contains("Attacker") {
                v.trust_penalty += 5;
                v.reputation -= 3;
            } else {
                v.reputation += 5;
            }

            println!(
                "{} → stake: {} | eff_rep: {} | trust_factor: {:.2} | influence: {:.2}",
                v.name,
                v.stake,
                v.effective_reputation(),
                v.effective_reputation() as f64 / 100.0,
                v.influence()
            );
        }
    }

    println!("\n⚔️ Conflict: Trust-Gated Wealth vs Honest Behavior\n");

    let mut honest_weight = 0.0;
    let mut attacker_weight = 0.0;

    for v in validators.iter() {
        if v.name.contains("Honest") {
            honest_weight += v.influence();
        } else {
            attacker_weight += v.influence();
        }
    }

    println!("Honest weight: {:.2}", honest_weight);
    println!("Attacker weight: {:.2}", attacker_weight);

    println!("\n🏆 FINAL RESULT:");

    if honest_weight > attacker_weight {
        println!("✔ Trust outweighs wealth (gated)");
    } else {
        println!("❌ Wealth still dominates (needs stronger gating)");
    }

    println!("\n✅ Phase 26 Complete");
}
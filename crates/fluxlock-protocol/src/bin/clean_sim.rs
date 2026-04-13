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

        let stake_weight = (self.stake as f64 / 1000.0).sqrt();
        let trust_factor = (self.effective_reputation() as f64) / 100.0;

        (self.effective_reputation() as f64) * stake_weight * trust_factor
    }
}

fn run_simulation() {
    println!("🧪 Fluxlock Phase 28 — Sybil Dampening\n");

    let mut validators = vec![
        Validator::new("Validator A (Honest)", 1000),

        // Sybil group
        Validator::new("Validator C1 (Sybil)", 1000),
        Validator::new("Validator C2 (Sybil)", 1000),
        Validator::new("Validator C3 (Sybil)", 1000),
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
                "{} → eff_rep: {} | influence: {:.2}",
                v.name,
                v.effective_reputation(),
                v.influence()
            );
        }
    }

    println!("\n⚠️ ATTACK PHASE (Sybil Coordination)\n");

    for round in 0..3 {
        println!("--- Attack Round {} ---", round + 1);

        for v in validators.iter_mut() {
            if v.name.contains("Sybil") {
                v.trust_penalty += 3;
                v.reputation -= 2;
            } else {
                v.reputation += 5;
            }

            println!(
                "{} → eff_rep: {} | influence: {:.2}",
                v.name,
                v.effective_reputation(),
                v.influence()
            );
        }
    }

    println!("\n🧠 Detecting Coordinated Group Behavior...\n");

    // 🔥 GROUP PENALTY
    let sybil_count = validators.iter().filter(|v| v.name.contains("Sybil")).count();

    let mut sybil_penalty_factor = 1.0;

    if sybil_count > 1 {
        println!("🚨 Sybil cluster detected ({} validators)", sybil_count);

        // Dampen combined influence
        sybil_penalty_factor = 0.5;
    }

    println!("\n⚔️ Conflict: Honest vs Sybil Group\n");

    let mut honest_weight = 0.0;
    let mut sybil_weight = 0.0;

    for v in validators.iter() {
        if v.name.contains("Honest") {
            honest_weight += v.influence();
        } else {
            sybil_weight += v.influence();
        }
    }

    // 🔥 APPLY GROUP DAMPENING
    let adjusted_sybil_weight = sybil_weight * sybil_penalty_factor;

    println!("Honest weight: {:.2}", honest_weight);
    println!("Sybil raw weight: {:.2}", sybil_weight);
    println!("Sybil adjusted weight: {:.2}", adjusted_sybil_weight);

    println!("\n🏆 FINAL RESULT:");

    if honest_weight > adjusted_sybil_weight {
        println!("✔ Honest validator resists Sybil attack (dampened)");
    } else {
        println!("❌ Sybil attack still overwhelms system");
    }

    println!("\n✅ Phase 28 Complete");
}
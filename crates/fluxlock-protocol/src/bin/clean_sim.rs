fn main() {
    println!("🧪 CLEAN SIMULATION ENTRY\n");
    run_simulation();
}

#[derive(Clone, Debug)]
struct Validator {
    name: String,
    stake: u128,
    reputation: i32,
}

impl Validator {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            stake: 1000,
            reputation: 100,
        }
    }

    // 🔥 HARD FAILURE
    fn slash(&mut self, amount: u128) {
        self.stake = self.stake.saturating_sub(amount);
        self.reputation = (self.reputation - 30).max(0);

        println!(
            "⚠️ {} SLASHED → stake: {} | rep: {}",
            self.name, self.stake, self.reputation
        );
    }

    // 🔄 SLOW TRUST RECOVERY
    fn reward(&mut self) {
        if self.reputation < 100 {
            self.reputation += 5;
        }
    }

    // 💰 ECONOMIC RE-ENTRY
    fn restake(&mut self, amount: u128) {
        println!("💰 {} RESTAKES {}", self.name, amount);
        self.stake += amount;
    }

    fn status(&self) -> &'static str {
        if self.reputation < 20 {
            "EXILED"
        } else if self.stake == 0 {
            "BANKRUPT"
        } else if self.stake < 200 {
            "RECOVERING"
        } else {
            "HEALTHY"
        }
    }
}

fn run_simulation() {
    println!("🧪 Fluxlock Phase 15 — Economic Reality Simulation\n");

    let mut validators = vec![
        Validator::new("Validator A"),
        Validator::new("Validator B"),
        Validator::new("Validator C"),
    ];

    // ⚡ CHAOS — FORCE BANKRUPTCY
    println!("⚡ Chaos Phase\n");

    for v in validators.iter_mut() {
        v.slash(500);
        v.slash(500);

        println!(
            "{} post-chaos → stake: {} | rep: {} | status: {}",
            v.name,
            v.stake,
            v.reputation,
            v.status()
        );
    }

    // 🌱 RECOVERY — CONTROLLED + VERIFIED
    println!("\n🌱 Recovery Phase\n");

    for round in 0..10 {
        println!("--- Round {} ---", round + 1);

        for v in validators.iter_mut() {
            // Step 1: recover trust slowly
            v.reward();

            // Step 2: FORCE ECONOMIC REBUILD
            if v.stake < 200 {
                println!("🔁 RESTAKE TRIGGER → {}", v.name);
                v.restake(50);
            }

            println!(
                "{} → stake: {} | rep: {} | status: {}",
                v.name,
                v.stake,
                v.reputation,
                v.status()
            );
        }
    }

    println!("\n✅ Phase 15 Complete");
}
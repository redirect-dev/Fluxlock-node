use fluxlock_protocol::state::account::Account;
use fluxlock_protocol::pq;

fn main() {
    println!("🧪 CLEAN SIMULATION ENTRY\n");
    run_simulation();
}

#[derive(Clone, Debug)]
struct Validator {
    name: String,
    stake: u128,
    reputation: i32,
    suspicion_timer: i32,
    trust_penalty: i32,
    valid_identity: bool,
}

impl Validator {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            stake: 1000,
            reputation: 100,
            suspicion_timer: 0,
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

    fn status(&self) -> &'static str {
        if !self.valid_identity {
            return "INVALID_IDENTITY";
        }

        if self.reputation < 20 {
            return "EXILED";
        }

        if self.stake == 0 {
            return "BANKRUPT";
        }

        if self.suspicion_timer > 0 {
            return "UNDER_REVIEW";
        }

        if self.stake >= 200 && self.effective_reputation() >= 60 {
            return "HEALTHY";
        }

        if self.stake >= 200 {
            return "UNTRUSTED";
        }

        "RECOVERING"
    }
}

fn run_simulation() {
    println!("🧪 Fluxlock Phase 18 — Influence & Consensus\n");

    let mut validators = vec![
        Validator::new("Validator A (Honest)"),
        Validator::new("Validator B (Broken Identity)"),
        Validator::new("Validator C (Aggressive)"),
    ];

    // Identity setup
    let mut accounts: Vec<Account> = vec![];

    for i in 0..3 {
        let (pq_pk, _) = pq::generate_keypair();

        let acc = Account::new(
            1000,
            format!("identity_{}", i).as_bytes().to_vec(),
            pq_pk,
        );

        accounts.push(acc);
    }

    println!("⚡ Identity Validation Phase\n");

    for v in validators.iter_mut() {
        if v.name.contains("Broken") {
            v.valid_identity = false;
            println!("❌ {} FAILED identity chain", v.name);
        } else {
            println!("✅ {} PASSED identity chain", v.name);
        }
    }

    println!("\n🌱 Behavior Phase\n");

    for round in 0..6 {
        println!("--- Round {} ---", round + 1);

        for v in validators.iter_mut() {
            if v.suspicion_timer > 0 {
                v.suspicion_timer -= 1;
            }

            if !v.valid_identity {
                println!("{} → BLOCKED", v.name);
                continue;
            }

            // Honest
            if v.name.contains("Honest") {
                if v.stake < 200 {
                    v.stake += 50;
                }
                v.reputation += 5;
            }

            // Aggressive
            if v.name.contains("Aggressive") {
                if v.stake < 200 {
                    v.stake += 100;
                }

                if round < 2 {
                    v.suspicion_timer = 3;
                    v.trust_penalty += 10;
                    v.reputation -= 5;
                }
            }

            println!(
                "{} → rep: {} | eff_rep: {} | influence: {:.2} | status: {}",
                v.name,
                v.reputation,
                v.effective_reputation(),
                v.influence(),
                v.status()
            );
        }
    }

    // 🔥 CONSENSUS CALCULATION
    println!("\n⚖️ Network Influence\n");

    let mut ranked = validators.clone();

    ranked.sort_by(|a, b| b.influence().partial_cmp(&a.influence()).unwrap());

    for v in ranked.iter() {
        println!(
            "{} → Influence: {:.2} | Status: {}",
            v.name,
            v.influence(),
            v.status()
        );
    }

    println!("\n🏆 Consensus Leader:");

    if let Some(top) = ranked.first() {
        if top.influence() > 0.0 {
            println!("✔ {}", top.name);
        } else {
            println!("❌ No valid leader");
        }
    }

    println!("\n✅ Phase 18 Complete");
}
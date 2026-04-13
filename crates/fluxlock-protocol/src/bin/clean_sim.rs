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
    println!("🧪 Fluxlock Phase 17 — Identity + Behavior Fusion (Fixed)\n");

    let mut validators = vec![
        Validator::new("Validator A (Valid Chain)"),
        Validator::new("Validator B (Broken Chain)"),
        Validator::new("Validator C (Valid but Aggressive)"),
    ];

    // 🔥 Simulated identity creation
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

    println!("\n🌱 Behavior + Identity Combined\n");

    for round in 0..6 {
        println!("--- Round {} ---", round + 1);

        for v in validators.iter_mut() {
            if v.suspicion_timer > 0 {
                v.suspicion_timer -= 1;
            }

            if !v.valid_identity {
                println!("{} → BLOCKED (invalid identity)", v.name);
                continue;
            }

            // Honest
            if v.name.contains("Valid Chain") {
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
                "{} → stake: {} | rep: {} | eff_rep: {} | penalty: {} | status: {}",
                v.name,
                v.stake,
                v.reputation,
                v.effective_reputation(),
                v.trust_penalty,
                v.status()
            );
        }
    }

    println!("\n✅ Phase 17 Complete");
}
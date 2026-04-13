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
    println!("🧪 Fluxlock Phase 19 — Conflict Resolution\n");

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

    for round in 0..4 {
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
                v.reputation += 5;
            }

            // Aggressive
            if v.name.contains("Aggressive") {
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

    // 🔥 CONFLICT SIMULATION
    println!("\n⚔️ Conflict Simulation\n");

    let mut decisions = vec![];

    for v in validators.iter() {
        if !v.valid_identity {
            continue;
        }

        let vote = if v.name.contains("Honest") {
            "VALID"
        } else if v.name.contains("Aggressive") {
            "INVALID"
        } else {
            "ABSTAIN"
        };

        decisions.push((v.name.clone(), vote, v.influence()));
    }

    println!("Votes:");

    let mut valid_weight = 0.0;
    let mut invalid_weight = 0.0;

    for (name, vote, influence) in &decisions {
        println!("{} → {} (weight {:.2})", name, vote, influence);

        match *vote {
            "VALID" => valid_weight += influence,
            "INVALID" => invalid_weight += influence,
            _ => {}
        }
    }

    println!("\n⚖️ Weighted Result:");

    println!("VALID weight: {:.2}", valid_weight);
    println!("INVALID weight: {:.2}", invalid_weight);

    println!("\n🏆 FINAL DECISION:");

    if valid_weight > invalid_weight {
        println!("✔ TRANSACTION ACCEPTED");
    } else if invalid_weight > valid_weight {
        println!("❌ TRANSACTION REJECTED");
    } else {
        println!("⚠️ TIE — NO CONSENSUS");
    }

    println!("\n✅ Phase 19 Complete");
}
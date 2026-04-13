use rand::Rng;

#[derive(Clone, Debug)]
pub struct Validator {
    pub id: usize,
    pub stake: f64,
    pub reputation: f64,
    pub effective_reputation: f64,
    pub penalty: f64,

    pub behavior_score: f64,
}

impl Validator {
    pub fn new(id: usize, is_attacker: bool) -> Self {
        if is_attacker {
            Self {
                id,
                stake: 10.0,
                reputation: 10.0,
                effective_reputation: 10.0,
                penalty: 20.0,
                behavior_score: 30.0,
            }
        } else {
            Self {
                id,
                stake: 100.0,
                reputation: 70.0,
                effective_reputation: 70.0,
                penalty: 0.0,
                behavior_score: 90.0,
            }
        }
    }

    pub fn influence(&self) -> f64 {
        // 🔥 TRUST GATING
        if self.effective_reputation < 30.0 {
            return self.stake * 0.01;
        }

        let trust_factor = (100.0 - self.penalty).max(0.0) / 100.0;

        self.stake * trust_factor * (self.effective_reputation / 100.0)
    }
}

pub fn simulate_saturation_attack() {
    let mut rng = rand::thread_rng();

    println!("\n=== PHASE 30: NETWORK SATURATION ATTACK ===\n");

    let mut validators: Vec<Validator> = Vec::new();

    // ✅ Honest validators
    for i in 0..20 {
        validators.push(Validator::new(i, false));
    }

    // 🚨 Attacker swarm
    for i in 20..120 {
        validators.push(Validator::new(i, true));
    }

    // 🔥 Simulate a few rounds
    for _ in 0..10 {
        for v in validators.iter_mut() {
            let action = if v.reputation < 30.0 {
                rng.gen_range(20.0..50.0) // noisy/bad actors
            } else {
                rng.gen_range(80.0..100.0) // honest
            };

            v.behavior_score =
                (v.behavior_score * 0.9) + (action * 0.1);

            // Slight rep adjustments
            if action > 70.0 {
                v.effective_reputation += 0.5;
            } else {
                v.effective_reputation -= 0.5;
            }

            // Clamp
            if v.effective_reputation > 100.0 {
                v.effective_reputation = 100.0;
            }
            if v.effective_reputation < 0.0 {
                v.effective_reputation = 0.0;
            }
        }
    }

    // 📊 Calculate influence totals
    let mut honest_total = 0.0;
    let mut attacker_total = 0.0;

    for v in validators.iter() {
        if v.reputation >= 30.0 {
            honest_total += v.influence();
        } else {
            attacker_total += v.influence();
        }
    }

    println!("Honest Validators Total Influence: {:.2}", honest_total);
    println!("Attacker Swarm Total Influence: {:.2}", attacker_total);

    println!(
        "\nInfluence Ratio (Honest / Attacker): {:.2}",
        honest_total / attacker_total.max(1.0)
    );

    println!("\n=== SAMPLE VALIDATORS ===\n");

    for v in validators.iter().take(5) {
        println!(
            "HONEST ID {} | Inf: {:.2} | Rep: {:.2}",
            v.id,
            v.influence(),
            v.effective_reputation
        );
    }

    for v in validators.iter().skip(20).take(5) {
        println!(
            "ATTACKER ID {} | Inf: {:.2} | Rep: {:.2}",
            v.id,
            v.influence(),
            v.effective_reputation
        );
    }
}

fn main() {
    simulate_saturation_attack();
}
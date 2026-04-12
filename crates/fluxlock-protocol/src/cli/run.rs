use crate::state::validator::Validator;

pub fn run() {
    println!("🚀 Running Fluxlock CLI...\n");

    let validator = Validator::new("CLI Validator");

    println!("Validator Initialized:");
    println!("Name: {}", validator.name);
    println!("Stake: {}", validator.stake);
    println!("Reputation: {}", validator.reputation);
}
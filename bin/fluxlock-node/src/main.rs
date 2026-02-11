use fluxlock_core::EngineCompositeState;
use fluxlock_engine::FluxlockEngine;

fn main() {
    println!("Fluxlock Node Starting...");

    let mut state = EngineCompositeState::new();

    println!("Initial State: {:?}", state);

    // Run deterministic tick loop
    for tick in 1..=10 {
        FluxlockEngine::execute_tick(&mut state);

        println!("After Tick {}: {:?}", tick, state);
    }

    println!("Fluxlock Node Finished.");
}

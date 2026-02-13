use fluxlock_core::{EngineCompositeState, TickClock};
use fluxlock_engine::FluxlockEngine;

fn main() {
    println!("Fluxlock Node Starting...");

    let mut state = EngineCompositeState::new();
    let mut clock = TickClock::new();

    println!("Initial State: {:?}", state);

   for _ in 0..25 {
        clock.advance();
        FluxlockEngine::execute_tick(&mut state);

        println!("Tick {}: {:?}", clock.current_tick, state);
    }

    println!("Fluxlock Node Finished.");
}

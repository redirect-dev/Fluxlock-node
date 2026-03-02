use fluxlock_core::{
    EngineCompositeState,
    TickClock,
    TickLog,
};
use fluxlock_engine::FluxlockEngine;

use std::fs::File;
use std::io::Write;

fn main() {
    println!("Fluxlock Node Starting...");

    let mut clock = TickClock::new();
    let mut state = EngineCompositeState::new();
    let mut engine = FluxlockEngine::new();

    let mut tick_log = TickLog::new();

    println!("Initial State: {:?}", state);

    for _ in 0..25 {
        clock.advance();
        engine.execute_tick(&mut state);

        tick_log.push(clock.tick, &state);

        println!("Tick {}: {:?}", clock.tick, state);
    }

    // Persist tick log for replay
    let json = serde_json::to_string_pretty(&tick_log)
        .expect("Failed to serialize tick log");

    let mut file = File::create("tick_log.json")
        .expect("Failed to create tick_log.json");

    file.write_all(json.as_bytes())
        .expect("Failed to write tick_log.json");

    println!("Fluxlock Node Finished.");
}

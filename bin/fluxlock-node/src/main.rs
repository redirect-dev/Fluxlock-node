use fluxlock_core::{
    EngineCompositeState,
    TrustState,
    LifecycleState,
    LockState,
    RecoveryState,
    TickLog,
    TickRecord,
};

use fluxlock_engine::FluxlockEngine;

fn main() {
    println!("Fluxlock Node Starting...");

    // =========================================
    // INITIAL STATE
    // =========================================
    let mut state = EngineCompositeState {
        trust: TrustState { trust_score: 100.0 },
        lifecycle: LifecycleState { stage: 0 },
        lock: LockState { level: 0 },
        recovery: RecoveryState {
            is_recovering: false,
            recovery_ticks: 0,
            grace_ticks_remaining: 0,
        },
    };

    // =========================================
    // TICK LOG (NEW)
    // =========================================
    let mut tick_log = TickLog::default();

    println!("Initial State: {:?}", state);

    // =========================================
    // MAIN TICK LOOP
    // =========================================
    let total_ticks = 25;

    for tick in 1..=total_ticks {
        FluxlockEngine::execute_tick(&mut state);

        // -------------------------------------
        // RECORD TICK (DETERMINISTIC LOG)
        // -------------------------------------
        tick_log.record(TickRecord {
            tick,
            trust_score: state.trust.trust_score,
            lifecycle_stage: state.lifecycle.stage,
            lock_level: state.lock.level,
            is_recovering: state.recovery.is_recovering,
        });

        println!("Tick {}: {:?}", tick, state);
    }

    // =========================================
    // END
    // =========================================
    println!("Fluxlock Node Finished.");
    println!("Total ticks logged: {}", tick_log.records.len());
}

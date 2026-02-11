# Fluxlock Rust Trait Interface Specification
Version: v0.1 (Draft)
Status: Phase 2 Implementation Preparation

This document defines canonical Rust trait interfaces for the Fluxlock reference engine.

The purpose of this specification is to provide stable behavioral contracts
for all core engine components, enabling deterministic, replayable,
and testable implementations across independent nodes.

This specification is designed to map directly to Rust code structure.

---

## 1. Design Goals

The trait system MUST ensure:

- Deterministic execution across nodes
- Replay compatibility
- Testability of individual engine modules
- Separation of protocol logic from runtime infrastructure
- Safe interface boundaries between engine components

---

## 2. Trust Engine Trait

Responsible for computing trust decay, pressure modifiers, and recovery gain.

```rust
pub trait TrustEngine {
    fn update_trust(
        &self,
        current_state: &TrustState,
        inputs: &NormalizedInputSet,
        constants: &TrustParameters,
    ) -> TrustState;
}
pub trait LifecycleEngine {
    fn evaluate_lifecycle(
        &self,
        trust_state: &TrustState,
        lifecycle_state: &LifecycleState,
    ) -> LifecycleState;
}
pub trait LockEngine {
    fn evaluate_lock_state(
        &self,
        current_lock: &NetworkLockState,
        inputs: &NormalizedInputSet,
        metrics: &LockMetrics,
    ) -> NetworkLockState;
}
pub trait RecoveryEngine {
    fn evaluate_recovery(
        &self,
        recovery_state: &RecoveryState,
        trust_state: &TrustState,
        inputs: &NormalizedInputSet,
    ) -> RecoveryState;
}
pub trait ResourceEngine {
    fn update_resource(
        &self,
        resource_state: &SecurityResourceState,
        lifecycle_state: &LifecycleState,
        lock_state: &NetworkLockState,
    ) -> SecurityResourceState;
}
pub trait TickExecutor {
    fn execute_tick(
        &self,
        state: &mut EngineCompositeState,
        inputs: &NormalizedInputSet,
    );
}
pub trait PersistenceEngine {
    fn write_snapshot(&self, state: &EngineCompositeState);
    fn append_replay_log(&self, entry: &ReplayEntry);
    fn load_snapshot(&self) -> EngineCompositeState;
}
pub trait ReplayEngine {
    fn load_replay_log(&self) -> Vec<ReplayEntry>;
    fn verify_replay(&self, state: &EngineCompositeState);
}
pub trait InputNormalizer {
    fn normalize_inputs(
        &self,
        network_messages: &[NetworkMessage],
        telemetry_signals: &[TelemetrySignal],
    ) -> NormalizedInputSet;
}

# Fluxlock Engine Scaffolding Layout Specification
Version: v0.1 (Draft)
Status: Phase 2 Implementation Preparation

This document defines the reference repository, crate, and module layout
for the Fluxlock reference protocol engine.

---

## 1. Repository Structure


---

## 2. Core Crate

Purpose:
Shared primitives and state definitions.

Modules:
- trust_state
- lifecycle_state
- lock_state
- recovery_state
- security_resource_state

---

## 3. Engine Crate

Purpose:
Deterministic evaluation pipeline.

Modules:
- input_normalization
- trust_engine
- lifecycle_engine
- lock_engine
- recovery_engine
- resource_engine
- tick_executor

---

## 4. Protocol Crate

Purpose:
Protocol constants and thresholds.

Modules:
- trust_parameters
- lock_thresholds
- recovery_rules
- decay_functions

---

## 5. Replay Crate

Purpose:
Replay injection and verification.

Modules:
- replay_loader
- replay_executor
- state_comparator

---

## 6. Metrics Crate

Purpose:
Security telemetry.

Modules:
- trust_metrics
- lock_metrics
- recovery_metrics
- resource_metrics
- attack_cost_metrics

---

## 7. Node Crate

Purpose:
Runtime node orchestration.

Modules:
- node_runtime
- role_assignment
- network_io
- state_persistence

---

## 8. Testnet Crate

Purpose:
Simulation orchestration.

Modules:
- adversarial_scenarios
- scenario_scheduler
- multi_node_runner

---

## 9. Deterministic Execution Requirement

Engine tick MUST execute in order:

1. Normalize Inputs
2. Update Trust
3. Update Lifecycle
4. Evaluate Locks
5. Evaluate Recovery
6. Update Resource
7. Emit State

---

## 10. Security Goal

Provide an implementation layout that guarantees deterministic,
replayable, adversary-resilient protocol execution.

# Fluxlock State Model Struct Specification
Version: v0.1 (Draft)
Status: Phase 2 Implementation Preparation

This document defines the canonical runtime state objects for the Fluxlock protocol engine.

This specification is designed to map directly to Rust struct definitions.

---

## 1. Trust State

Represents current operational trust confidence.

Fields:

- node_id
- trust_score (float or fixed point)
- decay_rate_modifier
- last_trust_update_tick
- anomaly_flags

---

## 2. Lifecycle State

Represents node defensive status classification.

Enum:

- ACTIVE
- DEGRADED
- QUARANTINED

Fields:

- current_state
- state_entry_tick
- degradation_reason

---

## 3. Network Lock State

Represents global defensive posture.

Enum:

- UNLOCKED
- RESTRICTED
- SOFT_LOCK
- LOCKED

Fields:

- current_lock_state
- lock_entry_tick
- lock_trigger_source

---

## 4. Recovery State

Tracks recovery validation progress.

Fields:

- recovery_attempt_active (bool)
- recovery_start_tick
- required_observation_ticks
- recovery_proof_hash
- recovery_validation_score

---

## 5. Security Resource State

Represents survivability signal accumulation.

Fields:

- accumulated_resource_value
- last_resource_update_tick
- resource_decay_flag

---

## 6. Engine Composite State

Represents full node evaluation state snapshot.

Contains:

- TrustState
- LifecycleState
- RecoveryState
- SecurityResourceState

Network State Injected Separately:
- NetworkLockState

---

## 7. Determinism Requirement

State updates MUST be pure function of:

Previous state  
Normalized inputs  
Protocol constants  

No hidden state allowed.

---

## 8. Security Goal

Ensure state representation supports deterministic replay and auditability.

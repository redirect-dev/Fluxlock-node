# Fluxlock Reference Engine Skeleton Build Plan
Version: v0.1 (Draft)
Status: Phase 3 Implementation Entry Planning

This document defines the initial build sequencing for the Fluxlock reference protocol engine.

---

## 1. Overview

The reference engine MUST be built in deterministic layers,
ensuring replay compatibility and testability from the earliest build stage.

---

## 2. Build Phase Order

---

### Phase A — Core State + Types

Implement:

- TrustState
- LifecycleState
- RecoveryState
- SecurityResourceState
- NetworkLockState
- EngineCompositeState

Goal:
Compileable core state model.

---

### Phase B — Tick Executor Skeleton

Implement:

- TickExecutor trait implementation
- Stub engine components

Goal:
Compileable deterministic tick loop.

---

### Phase C — Engine Module Implementations

Implement:

- TrustEngine
- LifecycleEngine
- LockEngine
- RecoveryEngine
- ResourceEngine

Goal:
Basic deterministic engine behavior.

---

### Phase D — Persistence + Replay Layer

Implement:

- Snapshot writing
- Replay log writing
- Snapshot loading
- Replay re-execution

Goal:
Deterministic crash recovery.

---

### Phase E — Node Runtime Loop

Implement:

- Network input loop
- Tick scheduler
- Persistence integration
- Metrics emission

Goal:
Running single-node deterministic engine.

---

### Phase F — Network Integration

Implement:

- Wire message parsing
- Signal normalization pipeline
- Peer communication layer

Goal:
Multi-node deterministic behavior.

---

### Phase G — Testnet Simulation Harness

Implement:

- Scenario injection
- Multi-node orchestration
- Metrics capture

Goal:
Testnet validation readiness.

---

## 3. Early Compile Goal

Each phase MUST:

Compile  
Run unit tests  
Support deterministic replay  

---

## 4. Security Goal

Ensure deterministic correctness before performance optimization.

# Fluxlock Persistence and Replay Storage Specification
Version: v0.1 (Draft)
Status: Phase 2 Implementation Preparation

This document defines state persistence, snapshotting, and replay log storage
requirements for the Fluxlock protocol engine.

---

## 1. Overview

Fluxlock nodes MUST persist state in a deterministic and replay-compatible manner.

Persistence MUST support:

- Crash recovery
- State verification
- Replay execution
- Audit log generation

---

## 2. Snapshot Storage

Snapshots represent full deterministic engine state at a specific tick.

Snapshot MUST include:

- TrustState
- LifecycleState
- RecoveryState
- SecurityResourceState
- Current NetworkLockState
- Tick counter

Snapshots SHOULD be stored periodically (configurable interval).

---

## 3. Replay Log Storage

Replay logs MUST capture:

- Normalized tick inputs
- State transitions
- Lock state changes
- Recovery validation events

Replay logs MUST be append-only.

---

## 4. Deterministic Replay Requirements

Replay execution MUST:

- Reconstruct state from snapshot + replay log
- Produce identical state transitions
- Produce identical final state

---

## 5. Crash Recovery Requirements

On restart, node MUST:

Load latest valid snapshot  
Replay forward using replay log  
Resume deterministic tick execution  

---

## 6. Storage Integrity

Persistence MUST support:

- Hash validation of snapshot files
- Hash chaining of replay logs
- Tamper detection

---

## 7. Security Goal

Ensure node state and behavior can be reconstructed and verified
under adversarial and failure conditions.

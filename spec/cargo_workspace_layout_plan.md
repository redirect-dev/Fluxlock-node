# Fluxlock Cargo Workspace Layout Plan
Version: v0.1 (Draft)
Status: Phase 3 Implementation Execution Planning

This document defines the Cargo workspace structure for the Fluxlock reference implementation.

---

## 1. Workspace Overview

Fluxlock reference implementation will be structured as a multi-crate Cargo workspace.

---

## 2. Workspace Root Structure

fluxlock-node/
├ Cargo.toml (workspace)
├ crates/
│ ├ core/
│ ├ engine/
│ ├ protocol/
│ ├ replay/
│ ├ persistence/
│ ├ network/
│ ├ node/
│ ├ testnet/
│ └ metrics/
└ bin/
└ fluxlock-node/

---

## 3. Crate Responsibilities

---

### core

State structs and shared types.

---

### engine

Deterministic protocol evaluation logic.

---

### protocol

Constants, thresholds, and parameter sets.

---

### replay

Replay loading, execution, and verification.

---

### persistence

Snapshot storage and replay log storage.

---

### network

Wire protocol handling, normalization, propagation logic.

---

### node

Runtime loop, orchestration, config loading.

---

### testnet

Simulation orchestration and adversarial scenario tooling.

---

### metrics

Security telemetry emission and aggregation.

---

## 4. Binary Target


Responsible for:

- Node startup
- Runtime orchestration
- Config loading
- Network bootstrap

---

## 5. Build Philosophy

Workspace MUST support:

- Deterministic builds
- Independent crate testing
- Replay-compatible state serialization

---

## 6. Security Goal

Ensure clean separation of protocol logic and runtime infrastructure.

# Fluxlock Reference Engine Architecture Specification
Version: v0.1 (Draft)
Status: Planning / Normative for Reference Implementation

This document maps Fluxlock protocol specifications into a concrete
reference engine architecture suitable for implementation (e.g., Rust).

This document does NOT define language-specific code.
It defines module responsibilities and interaction boundaries.

---

## 1. Overview

The reference engine architecture defines:

- State containers
- Evaluation pipelines
- Module boundaries
- Data flow ordering
- Replay injection points

---

## 2. Core Engine State Domains

Implementations MUST maintain deterministic state for:

- Trust State
- Node Lifecycle State
- Network Lock State
- Recovery Observation State
- Security Resource State

---

## 3. Required Engine Modules

### Input Normalization Module
Responsibilities:
- Normalize network observations
- Normalize adversarial signals
- Normalize replay inputs

---

### Trust Engine Module
Responsibilities:
- Apply base trust decay
- Apply pressure decay
- Apply recovery gain

---

### Lifecycle Engine Module
Responsibilities:
- Determine ACTIVE / DEGRADED / QUARANTINED transitions
- Enforce monotonic degradation rules

---

### Lock Engine Module
Responsibilities:
- Evaluate lock escalation
- Evaluate lock de-escalation
- Enforce global lock authority

---

### Recovery Engine Module
Responsibilities:
- Validate recovery proofs
- Track recovery observation windows
- Apply recovery trust gain limits

---

### Security Resource Engine Module
Responsibilities:
- Accumulate survivability contribution
- Apply decay on compromise
- Provide resource weighting outputs

---

## 4. Execution Order (Deterministic)

Each evaluation tick MUST execute in this order:

1. Input normalization
2. Trust update
3. Lifecycle update
4. Lock evaluation
5. Recovery evaluation
6. Security resource update
7. State publication

---

## 5. Replay Integration Points

Replay MUST be injectable at:

- Input normalization stage
- Trust state initialization
- Lock state initialization
- Recovery state initialization

---

## 6. Determinism Requirements

All modules MUST:

- Be pure-function evaluable given state + inputs
- Avoid hidden state mutation
- Produce replay-identical outputs

---

## 7. Security Goal

The architecture ensures:

> Fluxlock can be implemented consistently across independent nodes
> while preserving deterministic defensive behavior.

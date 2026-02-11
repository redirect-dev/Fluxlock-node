# Fluxlock Testnet Build Checklist
Version: v0.1 (Draft)
Status: Execution Planning

This document defines the minimum requirements to begin a Fluxlock testnet deployment.

---

## 1. Reference Engine Implementation

Required:
- Deterministic execution pipeline
- Trust evaluation engine
- Lifecycle state engine
- Lock state engine
- Recovery validation engine
- Security resource accounting engine

---

## 2. Replay Harness Integration

Required:
- Replay input injection
- Deterministic replay verification
- State transition comparison logging

---

## 3. Node Runtime

Required:
- Config-driven role assignment
- Metrics emission support
- Replay compatibility flags

---

## 4. Metrics Infrastructure

Required:
- Trust decay tracking
- Lock escalation tracking
- Recovery tracking
- Resource accumulation tracking
- Attack cost tracking

---

## 5. Adversarial Simulation

Required:
- Controlled adversarial node behavior flags
- Attack scenario injection tooling
- Identity churn simulation tooling

---

## 6. Deployment Readiness

Required:
- Node containerization or binary packaging
- Config distribution mechanism
- Log aggregation pipeline

---

## 7. Go / No-Go Criteria

Testnet launch MAY begin when:

- Deterministic replay matches spec behavior
- Lock escalation behaves correctly under simulated attack
- Recovery behaves correctly under controlled condition

# Fluxlock Testnet Node Role Specification
Version: v0.1 (Draft)
Status: Normative for Testnet Deployments

This document defines node role classifications for Fluxlock testnet environments.

---

## 1. Overview

Fluxlock testnets MUST support multiple node roles to simulate realistic network behavior and adversarial conditions.

---

## 2. Core Node Roles

### Standard Participant Node
Responsibilities:
- Execute full protocol engine
- Participate in trust evaluation
- Participate in lock coordination
- Submit and validate recovery proofs

---

### Sentinel Node (Optional Testnet Role)
Responsibilities:
- Enhanced monitoring
- Early anomaly detection signal contribution
- Lock escalation signal weighting (testnet only)

---

### Adversarial Simulation Node
Responsibilities:
- Execute controlled adversarial behavior
- Inject attack patterns
- Test trust decay and lock escalation behavior

---

### Replay Validator Node
Responsibilities:
- Execute deterministic replay validation
- Compare expected vs observed state transitions

---

## 3. Role Determinism

Role MUST NOT change protocol rules.
Roles only affect:

- Observability
- Simulation behavior
- Metrics generation

---

## 4. Security Goal

Node roles ensure:

> Fluxlock behavior can be validated under realistic distributed conditions.

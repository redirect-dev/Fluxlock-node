# Fluxlock Adversary Fatigue Model Specification
Version: v0.1 (Draft)
Status: Normative

This document defines the adversary fatigue model within the Fluxlock protocol.

This specification is subordinate only to:
- Fluxlock Protocol Axioms
- Fluxlock System Model
- Node Lifecycle Specification
- Network Lock State Specification
- Trust Decay Model Specification
- Recovery Proof Model Specification

---

## 1. Overview

Fluxlock assumes adversaries will attempt sustained pressure attacks.

The adversary fatigue model ensures that:

- Sustained attack becomes increasingly expensive
- Attack success probability decreases over time
- Defensive cost grows slower than offensive cost

---

## 2. Design Intent

Fluxlock is designed to favor defender endurance over attacker persistence.

Attackers should experience:

- Increasing computational cost
- Increasing coordination complexity
- Increasing trust decay resistance
- Increasing recovery difficulty

---

## 3. Fatigue Dimensions

Adversary fatigue operates across multiple dimensions:

### Time Cost
Longer attack duration increases total attack resource cost.

### Lock Amplification
Higher network lock states multiply attacker cost.

### Trust Resistance
Lower trust reduces attacker influence.

### Recovery Friction
Recovered nodes cannot be rapidly re-exploited.

---

## 4. Attack Cost Function (Conceptual)

AttackCost(t) increases as a function of:

- Time under attack
- Lock escalation level
- Node trust resistance
- Verification burden

---

## 5. Lock Interaction

Lock escalation MUST:

- Increase verification difficulty for attackers
- Increase cost of maintaining malicious presence
- Increase resource burn rate for adversaries

---

## 6. Trust Interaction

Lower trust MUST:

- Reduce adversarial influence
- Increase attacker operational complexity
- Increase time required to achieve compromise

---

## 7. Recovery Interaction

Recovery MUST prevent rapid attack re-entry.

Recovered nodes MUST require new trust-building effort.

---

## 8. Security Invariants

The following MUST hold:

- Sustained attack cost increases over time
- Attack cost growth exceeds defensive cost growth
- Repeated attack attempts have increasing marginal cost
- Attack persistence decreases success probability

---

## 9. Security Goal

The adversary fatigue model ensures:

> Sustained attack becomes operationally and economically irrational.

And:

> Time structurally favors defenders.

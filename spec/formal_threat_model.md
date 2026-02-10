# Fluxlock Formal Threat Model Specification
Version: v0.1 (Draft)
Status: Normative

This document defines the adversary capabilities and threat classes
considered in Fluxlock protocol design.

---

## 1. Overview

Fluxlock assumes persistent, adaptive adversaries.

The protocol is designed to remain secure under sustained,
coordinated, and evolving attack pressure.

---

## 2. Adversary Capability Classes

### Passive Observation
- Network traffic observation
- Timing analysis
- Pattern inference

---

### Active Network Manipulation
- Message delay and reordering
- Network partition attempts
- Traffic flooding

---

### Node Compromise
- Key compromise
- Runtime compromise
- Identity takeover attempts

---

### Coordinated Multi-Node Adversaries
- Sybil cluster behavior
- Coordinated lock manipulation attempts
- Trust poisoning campaigns

---

### Economic Adversaries
- Attempt to exploit economic incentives
- Attempt to bypass trust via economic accumulation

---

## 3. Long-Term Adversary Assumption

Fluxlock assumes adversaries will:

- Persist over long time horizons
- Adapt to defensive behavior
- Attempt recovery manipulation
- Attempt repeated reinsertion

---

## 4. Explicit Non-Goals

Fluxlock does NOT attempt to:

- Prevent all compromise events
- Guarantee perfect availability during attack
- Prevent short-term localized compromise

---

## 5. Security Goals

Fluxlock MUST ensure:

- Compromise impact decays over time
- Trust must be re-earned
- Sustained attack becomes ineffective
- Defensive endurance is rewarded

---

## 6. Threat Model Continuity

All future protocol and economic layers MUST remain compatible with this threat model.

# Fluxlock Node Lifecycle Specification
Version: v0.2 (Draft)
Status: Normative

This document defines the node state machine, transition rules,
and invariants governing node participation within the Fluxlock protocol.

This specification is subordinate only to:
- Fluxlock Protocol Axioms
- Fluxlock System Model

---

## 1. Overview

Fluxlock nodes exist in continuously evaluated trust states.

Nodes transition between states based on:
- Trust decay
- Observed behavior
- Network pressure
- Recovery proof validation

Nodes MUST exist in exactly one lifecycle state at any given time.

---

## 2. Design Intent

The Fluxlock node lifecycle is designed under the assumption that
adversarial pressure is continuous, adaptive, and long-lived.

The lifecycle intentionally favors:

- Defender endurance over attacker persistence
- Slow, proof-driven recovery over instant restoration
- Progressive restriction over sudden exclusion
- Identity continuity over key permanence

The lifecycle is not designed to prevent all compromise.

It is designed to ensure that sustained adversarial pressure
becomes increasingly ineffective over time.

Fluxlock assumes that attackers will attempt to apply
continuous pressure rather than single-event attacks.
The lifecycle is structured to make sustained pressure
increasingly expensive and operationally ineffective.

---

## 3. Node Lifecycle States

Fluxlock defines three primary node states:

- ACTIVE
- DEGRADED
- QUARANTINED

State determines:
- Network permissions
- Verification requirements
- Influence on network lock decisions
- Recovery obligations

---

## 4. ACTIVE State

### Description
ACTIVE represents normal operating state with full protocol participation.

### Capabilities
ACTIVE nodes MAY:
- Validate and propagate messages
- Participate in lock state coordination
- Submit and verify trust-relevant protocol actions
- Rotate keys at standard cadence

### Exit Conditions
Nodes MUST transition to DEGRADED if:
- Trust falls below Active Threshold
- Suspicious or non-compliant behavior is detected
- Sustained adversarial pressure is detected
- Network

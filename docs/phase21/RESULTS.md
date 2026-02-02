# Phase 21 — Attack Cost Modeling

## Purpose

This phase models the economic cost of sustained attacks against FluxLock.

Rather than claiming absolute security, FluxLock increases the **cost, complexity, and duration** required to compromise the system.

---

## Threat Model

Attacker assumptions:
- Can compromise a node or key at time T
- Has sufficient compute resources
- Attempts persistence rather than single-shot attacks

Defender assumptions:
- Keys rotate
- Trust decays on anomalous behavior
- Nodes can be degraded or quarantined

---

## Baseline Comparison

### Traditional Static-Key System

| Property | Result |
|--------|-------|
| Key lifetime | Long |
| Attack persistence | High |
| Marginal attack cost | Low |
| Recovery | Manual |

Once compromised, the attacker maintains access until detection.

---

### FluxLock System

| Property | Result |
|--------|-------|
| Key lifetime | Short |
| Attack persistence | Low |
| Marginal attack cost | Increases over time |
| Recovery | Automatic |

Attackers must continuously re-compromise the system.

---

## Cost Curve Model

Let:
- `C₀` = cost to compromise a key once
- `R` = key rotation interval
- `D` = trust decay rate
- `P` = penalty multiplier (stake, trust loss, quarantine)

Total attacker cost over time `T`:


Where `Pᵢ` increases with:
- Failed rotations
- Trust decay
- Quarantine events

This creates a **superlinear cost curve**.

---

## Simulation Evidence

Observed behaviors:
- Honest nodes stabilize near trust = 1.0
- Malicious or unstable nodes:
  - Accumulate key age
  - Lose trust
  - Enter degraded or quarantined states
- Recovery requires sustained good behavior

Attacks are no longer “one-and-done”.

---

## Economic Implication

FluxLock transforms security from:
> “Can you break this once?”

Into:
> “Can you afford to keep breaking this forever?”

For most adversaries, the answer is no.

---

## Summary

FluxLock does not promise invulnerability.

It promises:
- Rising attacker costs
- Reduced persistence
- Automatic containment
- Measurable economic deterrence

Security becomes a **losing game for attackers**.

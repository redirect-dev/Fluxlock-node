# FluxLock Key Rotation Mechanism

## Overview

FluxLock does not rely on long-lived cryptographic keys.  
Instead, it enforces **continuous key rotation tied to node behavior and trust state**.

Keys in FluxLock are:
- Short-lived
- Behavior-dependent
- Invalidated by time, entropy drift, or trust decay

This design minimizes the window in which any compromised key is useful.

---

## Why Key Rotation Is Mandatory

Traditional systems assume:
> “A key is secure until it is broken.”

FluxLock assumes:
> “A key is already obsolete by the time it is targeted.”

Key rotation ensures:
- No key remains valid long enough for offline quantum attacks
- Stolen keys decay naturally
- Trust erosion forces cryptographic renewal

---

## Rotation Triggers

A node **must rotate its key** when *any* of the following occur:

1. **Key Age Threshold**
   - Keys expire after a fixed number of ticks
   - Prevents long-term cryptanalysis

2. **Trust Degradation**
   - Falling trust accelerates rotation
   - Low-trust nodes rotate more frequently

3. **State Transitions**
   - Degraded → Active
   - Active → Degraded
   - Any recovery from quarantine

4. **Behavioral Anomalies**
   - Erratic decisions
   - Weighted consensus divergence

---

## Time as a Defensive Layer

FluxLock treats **time itself as a security primitive**.

An attacker must:
- Observe the key
- Exploit the key
- Maintain trust
- Act before rotation

This compresses the attack window beyond practical feasibility, even with quantum resources.

---

## Interaction with Trust & State

| Node State     | Rotation Frequency |
|---------------|--------------------|
| Active        | Normal             |
| Degraded      | Accelerated        |
| Quarantined   | Forced / Reset     |

Nodes cannot opt out of rotation.  
Rotation is enforced by protocol logic, not node preference.

---

## Security Implications

- No static attack surface
- No “harvest now, decrypt later”
- No reliance on a single cryptographic assumption
- Breach impact is localized and temporary

FluxLock security emerges from **movement**, not secrecy.

---

## Summary

FluxLock keys are not assets to be protected indefinitely.  
They are **ephemeral signals** bound to time, trust, and behavior.

In a quantum era, permanence is a liability.  
FluxLock survives by never standing still.

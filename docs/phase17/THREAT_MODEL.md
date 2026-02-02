# FluxLock Threat Model

## Purpose

This document defines the adversarial assumptions under which FluxLock operates
and explains why those adversaries fail to compromise the system in a sustained way.

FluxLock is designed for a future where **quantum and classical attacks coexist**.

---

## Adversary Classes

FluxLock considers five primary attacker categories.

---

### 1. Passive Observer (Classical or Quantum)

**Capabilities**
- Observes network traffic
- Collects keys, messages, and state transitions
- Performs offline analysis

**Why FluxLock Holds**
- Keys are short-lived
- Observed keys expire before exploitation
- No long-term secrets exist

**Outcome**
- Data collected becomes obsolete
- No retroactive compromise

---

### 2. Active Network Attacker

**Capabilities**
- Injects messages
- Replays old states
- Attempts consensus manipulation

**Why FluxLock Holds**
- Trust scoring penalizes inconsistent behavior
- Weighted consensus reduces single-node impact
- Replay attacks fail due to time-bound state

**Outcome**
- Attacker is degraded and quarantined

---

### 3. Key Compromise Attacker

**Capabilities**
- Temporarily steals a node’s private key
- Attempts impersonation

**Why FluxLock Holds**
- Key usefulness decays rapidly
- Rotation invalidates stolen keys
- Trust loss accelerates rotation

**Outcome**
- Breach window is narrow and localized

---

### 4. Quantum Adversary (Cryptanalytic)

**Capabilities**
- Breaks asymmetric cryptography
- Stores encrypted traffic for later decryption
- Uses Grover/Shor-class attacks

**Why FluxLock Holds**
- No static keys to harvest
- No “decrypt later” value
- Security relies on time, not hardness

**Outcome**
- Quantum advantage is neutralized by key expiration

---

### 5. Byzantine / Insider Node

**Capabilities**
- Behaves maliciously while appearing valid
- Attempts slow trust erosion or split behavior

**Why FluxLock Holds**
- Behavior is continuously evaluated
- Long-term dishonesty is statistically detectable
- Trust decay leads to degraded state and quarantine

**Outcome**
- Insider influence is capped and temporary

---

## Time as a Security Primitive

FluxLock explicitly treats **time** as an attack surface and defense layer.

An attacker must:
1. Acquire a key
2. Maintain trust
3. Act within a shrinking window
4. Avoid detection

Failure at any step invalidates the attack.

---

## What FluxLock Does NOT Protect Against

FluxLock does not claim:
- Absolute secrecy
- Perfect anonymity
- Instant attack detection

Instead, it guarantees:
- **Attack non-persistence**
- **Automatic recovery**
- **Bounded damage**

---

## Security Philosophy

Traditional systems aim to prevent compromise.

FluxLock assumes compromise is inevitable and ensures:
- It cannot persist
- It cannot scale
- It cannot be reused

---

## Summary

FluxLock is not secure because its keys are unbreakable.

FluxLock is secure because **keys do not matter for long enough to break them**.

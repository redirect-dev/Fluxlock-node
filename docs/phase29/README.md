# Fluxlock

**Fluxlock** is an experimental security protocol designed to minimize catastrophic breaches by embracing *continuous key rotation*, *dynamic trust scoring*, and *self-locking network behavior*.

Rather than attempting to make breaches impossible, Fluxlock assumes adversarial pressure is constant and focuses on **containment, degradation, and recovery** instead of brittle prevention.

---

## Core Idea

> A lock whose keys are always changing — and which locks itself when trust collapses.

Fluxlock treats security as a **living system**:
- Keys rotate continuously
- Trust is evaluated over time
- Nodes degrade, quarantine, or recover based on behavior
- The network can enter a *soft lock* state without halting key rotation

This design increases attacker cost over time and limits blast radius when failures occur.

---

## What Fluxlock Is (and Is Not)

### Fluxlock **is**
- A protocol simulation exploring adversarial fatigue
- A trust-driven security model
- A self-regulating system that degrades gracefully
- A foundation for future distributed security research

### Fluxlock **is not**
- A finished cryptocurrency
- A production-ready blockchain
- A claim of absolute or “quantum-proof” security

---

## Key Concepts

### Continuous Key Rotation
Keys are never static. Even in degraded or locked states, rotation continues to prevent long-term key exposure.

### Dynamic Trust
Each node maintains a trust score influenced by behavior over time. Trust directly impacts:
- Node state
- Network permissions
- Recovery eligibility

### Node States
- **Active** – Normal operation
- **Degraded** – Limited participation
- **Quarantined** – Isolated from the network

### Network Lock States
- **Unlocked** – Normal operation
- **Restricted** – Partial access
- **Locked (Soft Lock)** – Network halts participation, *but keys still rotate*

Soft Lock is a defining feature: the system freezes access without freezing cryptographic motion.

---

## Phase 29: Soft Lock (Current Milestone)

Phase 29 introduces **network-wide soft locking**:

- The network transitions to `Locked` when trust collapses
- All nodes are quarantined
- **Key rotation continues**
- No automatic recovery occurs

This models a real-world response to sustained attack pressure: freeze access, deny utility to adversaries, and preserve cryptographic freshness.

---

## Why This Matters

Most systems fail catastrophically when breached.

Fluxlock asks a different question:

> What if failure was expected — and designed for?

By increasing attacker cost over time and preventing static secrets, Fluxlock explores a path toward *resilient security systems* instead of fragile ones.

---

## Project Status

- **Stage:** Research / Simulation
- **Language:** Rust
- **Focus:** Protocol behavior, not performance
- **Stability:** Evolving

This repository represents an ongoing exploration. Interfaces, models, and parameters are expected to change.

---

## Running the Simulation

```bash
cargo build --release
cargo run --release -- node1
cargo run --release -- node3

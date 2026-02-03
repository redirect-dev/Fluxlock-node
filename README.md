# Fluxlock

Fluxlock is an experimental security protocol exploring **continuous key rotation**, **dynamic trust scoring**, and **self-locking network behavior** under sustained adversarial pressure.

Rather than attempting to prevent breaches absolutely, Fluxlock assumes compromise is possible and focuses on **containment, degradation, and recovery**.

---

## Core Concept

> A lock whose keys are always changing — and which locks itself when trust collapses.

Fluxlock models security as a living system:
- Keys rotate continuously
- Trust decays and recovers over time
- Nodes self-isolate when behavior degrades
- The network can enter a *soft lock* state while keys continue rotating

---

## Current Status

- **Stage:** Research / Simulation
- **Language:** Rust
- **Latest Milestone:** Phase 29 — Network Soft Lock
- **Focus:** Protocol behavior, not production deployment

---

## Documentation

- [Phase 29 — Soft Lock](docs/phase29/README.md)

Earlier phases document trust decay, quarantine, recovery, and adversary fatigue models.

---

## Disclaimer

Fluxlock is experimental research software.  
Do not use in production systems.

---

## License

MIT

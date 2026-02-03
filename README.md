# Fluxlock

Fluxlock is a crypto-native security protocol designed to resist
*sustained, adaptive, and asymmetric attacks* through time-based defense,
continuous key rotation, and adversary fatigue.

Rather than preventing failure, Fluxlock assumes failure is inevitable
and designs recovery, trust, and liveness around that assumption.

---

## Core Design Principles

Fluxlock is built on the following invariants:

- **Continuous key rotation** — keys are disposable, identity continuity is not
- **Trust decays over time** — trust must be actively maintained
- **Failure-first security** — degradation is expected and controlled
- **Adversary fatigue** — attack cost grows super-linearly over time
- **Lock-based defense** — the network hardens under pressure, not after breach
- **Recovery is earned** — naive or instantaneous recovery is denied

Security is always prioritized over liveness during sustained attack.

---

## Project Status

### Phase 0 — Simulation (Complete, Frozen)

Phase 0 validated Fluxlock’s core mechanics via a Rust-based simulation:

- Node lifecycle: **Active → Degraded → Quarantined**
- Network-wide lock escalation:
  **Unlocked → Restricted → Soft Lock → Locked**
- Continuous key rotation in all states
- Trust decay under sustained pressure
- Adversary cost escalation
- Denial of naive recovery
- Failure-first behavior under attack

📌 **Phase 0 is now frozen** and exists as a historical reference only.

All Phase 0 code and artifacts live under:


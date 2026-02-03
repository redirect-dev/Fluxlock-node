# Fluxlock Protocol Axioms
Version: v0.1 (Draft)
Status: Normative

This document defines the immutable axioms of the Fluxlock protocol.
All protocol rules, implementations, and extensions MUST conform to
these axioms.

If any behavior contradicts an axiom, the behavior is invalid,
regardless of implementation convenience or performance concerns.

---

## Axiom 1 — Failure Is Inevitable

Fluxlock assumes that all nodes, keys, and communication channels
will eventually fail, degrade, or be compromised.

The protocol MUST NOT rely on:
- Permanent correctness
- Long-lived secrets
- Assumed honesty
- Single-point defenses

Failure is treated as an expected state, not an exception.

---

## Axiom 2 — Security Supersedes Liveness Under Pressure

During sustained or coordinated attack, Fluxlock prioritizes
system integrity over availability.

The protocol MAY:
- Reduce functionality
- Restrict participation
- Temporarily deny service
- Quarantine honest nodes

If liveness and security are in conflict, security MUST prevail.

---

## Axiom 3 — Trust Is Continuous and Decaying

Trust within Fluxlock is:
- Non-binary
- Time-dependent
- Continuously decaying

Trust MUST decrease over time unless actively maintained.
Trust MUST NOT increase instantaneously.

Any recovery of trust MUST require:
- Time
- Observable behavior
- Cryptographic proof

---

## Axiom 4 — Keys Are Disposable, Identity Is Not

Cryptographic keys are treated as short-lived, replaceable artifacts.

The protocol MUST:
- Support continuous key rotation
- Rotate keys in all operational states, including lock and quarantine
- Assume eventual key compromise

Identity continuity MUST NOT depend on long-lived keys.

---

## Axiom 5 — Adversaries Are Adaptive and Persistent

Fluxlock assumes adversaries who:
- Learn from prior failures
- Sustain attacks over long durations
- Adapt strategies dynamically
- Possess asymmetric resources

Security mechanisms MUST remain effective under repeated,
long-horizon pressure, not just single-event attacks.

---

## Axiom 6 — Sustained Attack Must Become Uneconomical

The cost of sustained attack MUST increase faster than the cost of defense.

Fluxlock MUST enforce:
- Rising marginal cost for repeated attack attempts
- Adversary fatigue through time-based escalation
- Defensive mechanisms whose cost grows sub-linearly relative to attack cost

An attacker who persists MUST pay an increasing price.

---

## Axiom 7 — Locking Is Defensive, Not Punitive

Network locks exist to preserve system integrity, not to punish nodes.

Locks MAY:
- Restrict capabilities
- Limit participation
- Enforce stricter validation

Locks MUST:
- Escalate automatically under pressure
- De-escalate only through collective recovery conditions
- Apply uniformly, including to honest participants

---

## Axiom 8 — Recovery Must Be Earned

Fluxlock explicitly denies naive recovery.

Nodes MUST NOT recover trust, permissions, or status solely through:
- Key rotation
- Identity reset
- Timeouts without behavior
- Self-asserted claims

Recovery MUST require:
- Sustained compliant behavior
- Cryptographic evidence
- Protocol-defined proof conditions

---

## Axiom 9 — No Actor Is Above Protocol Pressure

All participants, including:
- Validators
- Coordinators
- Founders
- Governance entities

are subject to:
- Trust decay
- Lock restrictions
- Recovery requirements

No role is exempt from protocol enforcement.

---

## Axiom 10 — Protocol Rules Are Deterministic and Observable

Given identical state and inputs, Fluxlock MUST produce identical outcomes.

All critical protocol decisions MUST be:
- Deterministic
- Verifiable
- Auditable

Security MUST NOT depend on obscurity, randomness without justification,
or off-protocol discretion.

---

## Axiom 11 — Economic Incentives Are Security Mechanisms

Economic primitives in Fluxlock exist to enforce security outcomes,
not speculation or wealth extraction.

Any economic mechanism MUST:
- Reinforce honest behavior over time
- Penalize sustained attack
- Align long-term incentives with protocol health

Economic design is a security layer.

---

## Axiom 12 — Fluxlock Is Hostile to Time-Based Exploits

The protocol MUST resist:
- Replay attacks
- Grinding attacks
- Patience-based exploitation
- Low-and-slow adversarial strategies

Time MUST favor the defender.

---

## Axiom 13 — Minimal Trust, Maximal Assumption of Hostility

Fluxlock assumes:
- Partial compromise
- Byzantine behavior
- Delayed or missing messages
- Coordinated adversaries

The protocol MUST remain safe under these conditions.

---

## Axiom 14 — Specification Supersedes Implementation

The protocol specification is authoritative.

If an implementation deviates from the specification,
the implementation is wrong.

---

## Closing Statement

These axioms define Fluxlock’s security posture and design constraints.

They are intentionally conservative, adversarial, and restrictive.
Any future extension, optimization, or economic system MUST
demonstrably preserve these axioms.

Fluxlock does not seek to eliminate attacks.
It seeks to make sustained attack irrational.

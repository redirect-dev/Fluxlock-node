# Fluxlock System Model
Version: v0.1 (Draft)
Status: Normative

This document defines the system model for the Fluxlock protocol.
It specifies the entities, assumptions, and boundaries within which
Fluxlock operates.

Anything not explicitly defined in this document MUST NOT be assumed
by the protocol.

This document is subordinate only to the Fluxlock Protocol Axioms.

---

## 1. System Overview

Fluxlock is a distributed security protocol composed of autonomous
nodes that coordinate defensive behavior in response to sustained,
adaptive adversarial pressure.

Fluxlock does not assume:
- a specific blockchain
- a specific consensus mechanism
- synchronous communication
- an honest majority
- trusted coordinators

---

## 2. Entities

### 2.1 Node

A **Node** is an autonomous participant that:
- maintains protocol state
- rotates cryptographic keys
- evaluates trust locally
- participates in network locks

Nodes may be:
- honest
- faulty
- compromised
- adversarial

The protocol MUST remain safe under any mixture of the above.

---

### 2.2 Network

The **Network** is the logical aggregation of nodes participating
in Fluxlock.

The network:
- has no central authority
- has no trusted leader
- may be partitioned
- may experience message delay or loss

Network-wide behavior emerges from node-local rules.

---

### 2.3 Adversary

An **Adversary** is any actor attempting to:
- degrade trust
- exhaust resources
- bypass locks
- force premature recovery
- maintain long-term pressure

Adversaries are assumed to be:
- adaptive
- persistent
- economically motivated
- capable of partial compromise

---

## 3. Time Model

Fluxlock operates over discrete logical time steps.

The protocol assumes:
- time may be skewed across nodes
- no global clock
- eventual progression of local time

All time-based behavior MUST tolerate drift and delay.

---

## 4. Communication Model

Fluxlock assumes an unreliable communication substrate.

Messages may be:
- delayed
- duplicated
- reordered
- dropped

The protocol MUST NOT depend on timely delivery for safety.

---

## 5. Trust Model

Trust is:
- local to each node
- continuously valued
- monotonically decreasing without reinforcement

Trust is never binary and never absolute.

Trust values influence:
- node state
- lock participation
- recovery eligibility

---

## 6. Key Model

Nodes possess cryptographic keys used for:
- authentication
- message integrity
- participation proofs

Keys are:
- short-lived
- continuously rotated
- assumed compromiseable

Key possession alone MUST NOT confer trust.

---

## 7. Lock Model

The network may enter defensive lock states.

Locks:
- restrict behavior
- reduce attack surface
- increase adversary cost

Locks apply uniformly and are not punitive.

---

## 8. Recovery Model

Nodes may attempt recovery from degraded or quarantined states.

Recovery:
- is slow
- is behavior-based
- requires cryptographic evidence

Recovery without proof is invalid.

---

## 9. Out-of-Scope Explicitly

Fluxlock does NOT define:
- consensus rules
- token issuance
- governance voting
- economic pricing
- user interfaces

These may exist externally but MUST NOT violate Fluxlock axioms.

---

## 10. Security Goal

Fluxlock’s primary security goal is to ensure that:

> Sustained attack becomes irrational before sustained defense does.

All protocol behavior is evaluated against this goal.

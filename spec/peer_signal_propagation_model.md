# Fluxlock Peer Signal Propagation Model Specification
Version: v0.1 (Draft)
Status: Phase 2 Implementation Preparation

This document defines how Fluxlock protocol signals propagate across the network.

---

## 1. Overview

Fluxlock uses controlled signal propagation to balance:

- Fast network awareness
- Resistance to adversarial amplification
- Deterministic signal visibility across nodes

---

## 2. Propagation Goals

Propagation MUST:

- Ensure high signal visibility
- Limit adversarial signal flooding
- Preserve replay determinism
- Avoid network-level oscillation

---

## 3. Signal Classes

Signals are classified as:

Critical:
- Lock state transitions
- Recovery validation completion

High Priority:
- Recovery proof submission
- High confidence adversarial pressure signals

Standard:
- Trust telemetry
- Low confidence anomaly signals

---

## 4. Propagation Strategy

---

### Critical Signals

Propagation Method:
- Broadcast to all peers
- Mandatory forwarding

---

### High Priority Signals

Propagation Method:
- Gossip broadcast
- Limited hop TTL

---

### Standard Signals

Propagation Method:
- Gossip sampling
- Rate limited forwarding

---

## 5. Anti-Amplification Controls

Nodes MUST:

- Deduplicate signal hashes
- Enforce per-peer signal rate limits
- Drop signals exceeding TTL
- Reject invalid signatures

---

## 6. Deterministic Visibility Windows

Signals MUST be:

- Propagated within defined tick windows
- Logged for replay
- Normalized before engine ingestion

---

## 7. Adversarial Resistance

Propagation model MUST resist:

- Signal flooding attacks
- Echo amplification loops
- Delayed replay injection
- Sybil broadcast storms

---

## 8. Security Goal

Ensure network signal propagation supports fast awareness
without allowing adversarial signal manipulation or flooding.

# Fluxlock Network Topology and Peer Discovery Specification
Version: v0.1 (Draft)
Status: Phase 2 Implementation Preparation

This document defines network topology assumptions and peer discovery mechanisms
for Fluxlock nodes.

---

## 1. Overview

Fluxlock assumes adversaries may attempt to manipulate network topology
to isolate nodes, amplify signals, or degrade network visibility.

Topology and peer discovery mechanisms MUST therefore be adversary-aware.

---

## 2. Topology Goals

The network MUST:

- Maintain sufficient peer diversity
- Resist Sybil clustering
- Resist targeted isolation
- Maintain signal visibility across network partitions

---

## 3. Peer Discovery Sources

Nodes MAY discover peers via:

- Bootstrap peer lists
- Peer exchange gossip
- Testnet registry services (if applicable)

Nodes MUST validate peers before long-term connection.

---

## 4. Peer Set Diversity Requirements

Nodes SHOULD maintain peer sets that are:

- Geographically distributed
- Network-AS distributed (when possible)
- Node-ID entropy distributed

---

## 5. Connection Rotation

Nodes SHOULD:

- Periodically rotate non-critical peers
- Maintain stable connections to trusted long-term peers
- Limit rapid peer churn to prevent topology oscillation

---

## 6. Sybil Resistance Measures

Nodes MUST:

- Limit connections from identical network subnets
- Limit connections from correlated node identity patterns
- Prefer long-lived verified peers

---

## 7. Partition Awareness

Nodes MUST detect potential partitions via:

- Signal visibility metrics
- Peer reachability metrics
- Lock state divergence detection

Nodes SHOULD attempt controlled peer expansion when partition suspected.

---

## 8. Peer Reputation Influence

Peer reliability MAY influence:

- Signal weighting
- Connection persistence preference

Peer reputation MUST NOT override core protocol security decisions.

---

## 9. Security Goal

Ensure network topology remains stable, diverse, and adversary resistant,
even under sustained targeted network attacks.

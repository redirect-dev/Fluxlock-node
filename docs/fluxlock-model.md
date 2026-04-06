# Fluxlock — Time-Bound Identity Model

## 🧠 Overview

Fluxlock is a protocol that enforces **time-bound cryptographic identity**.

Unlike traditional systems where identities persist indefinitely, Fluxlock requires identities to **evolve over time** or become invalid.

> Identity is not permanent — it must be continuously maintained.

---

## ⚠️ The Problem

Most systems assume:

- a valid key = valid identity
- identity persists indefinitely
- compromise results in long-term access

Even with post-quantum cryptography, this assumption remains unchanged.

> Stronger keys do not solve persistent identity.

---

## 🔐 Core Concept

Fluxlock defines identity as:

identity = cryptographic validity + temporal validity

A valid signature alone is not sufficient.

An identity must also be **current within the network’s time model**.

---

## ⏱ Time-Bound Identity

Each identity exists within a defined lifecycle:

1. Active — identity is valid and synchronized  
2. Rotating — new identity is being committed/revealed  
3. Expired — identity is no longer accepted by the network  

If an identity is not updated within its allowed window:

> it becomes invalid — regardless of cryptographic correctness

---

## 🔁 Identity Rotation

Fluxlock enforces rotation through a two-phase process:

### Commit Phase
- A new identity is cryptographically committed  
- The network acknowledges an upcoming change  

### Reveal Phase
- The new identity is revealed and activated  
- The previous identity is deprecated  

This ensures:

- continuity of identity  
- resistance to replay and preimage attacks  
- deterministic transitions  

---

## ⚔️ Enforcement Model

Fluxlock enforces identity validity at the protocol level:

- Transactions from expired identities are rejected  
- Valid signatures can still fail if identity is outdated  
- Validators that accept invalid transactions are penalized (slashed)  

> Enforcement is automatic and deterministic

---

## 🔐 Cryptographic Model

Fluxlock uses hybrid signatures:

- Ed25519 (classical)  
- Dilithium (post-quantum)  

This provides:

- immediate compatibility  
- future resilience against quantum attacks  

---

## 🧠 Security Implications

Fluxlock changes the threat model:

### Traditional Systems
key compromised → persistent access  

### Fluxlock
key compromised → temporary access → rotation required → access lost  

---

## 📉 Risk Reduction

By enforcing time-bound identity, Fluxlock reduces:

- long-term credential abuse  
- delayed breach impact  
- replay and stale identity attacks  

---

## 🧭 Positioning

Fluxlock is not:

- just a blockchain  
- just a post-quantum upgrade  

It is:

> a protocol that enforces expiring identity  

---

## 🚀 Current Status

- Core protocol: implemented  
- Identity rotation: functional  
- Enforcement + slashing: active  
- Deterministic execution: verified  
- Demo: available  

---

## 🔮 Future Work

- persistent state layer  
- networked consensus  
- developer interface  
- identity lifecycle tooling  

---

## 💬 Closing Thought

Most systems verify identity.

Fluxlock verifies:

> whether that identity still has the right to exist.
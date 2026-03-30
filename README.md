# 🔐 Fluxlock

> Time-bound, self-healing identity for secure systems

---

## 🚀 What is Fluxlock?

Fluxlock is a protocol that enforces **continuous identity evolution**.

Unlike traditional systems where a stolen key grants permanent access, Fluxlock ensures:

- identities **expire over time**
- keys must be **rotated regularly**
- outdated identities are **automatically rejected**

---

## 🧠 Core Idea

identity = evolving + time-bound + enforced

If a key is compromised:

→ it only works for a limited time  
→ the system forces rotation  
→ the attacker loses access  

---

## 🔥 Key Features

- 🔐 Hybrid cryptography (Ed25519 + Dilithium)
- 🔁 Atomic key rotation (commit → reveal)
- ⏱ Epoch-based identity enforcement
- ⚠️ Automatic rejection of outdated identities
- 🛡 Validator slashing for invalid blocks

---

## 🎬 Live Demo

Run the system and watch identity evolve in real time:

```bash
cargo run --bin fluxlock-protocol run
🚀 FLUXLOCK LIVE SECURITY DEMO

⏱ Tick 3
🔐 EVENT: Identity Commit Initiated

⏱ Tick 4
🔁 EVENT: Identity Rotation Executed

⏱ Tick 10
⚠️ EVENT: Attempting transaction with outdated identity
❌ Block rejected: Account not rotated for current epoch
# Run demo
cargo run --bin fluxlock-protocol demo

# Generate new account
cargo run --bin fluxlock-protocol new-account

# Run live chain
cargo run --bin fluxlock-protocol run
# 🔐 Fluxlock
## ⚡ Try It

Run the demo:

```bash
cargo run -p fluxlock-protocol --bin fluxlock-demo
Watch what happens:

Identity rotates
Identity expires
Expired identity is rejected
The network enforces validity over time
### ⚔ Attack Mode

Simulate repeated attempts to reuse expired identity:

```bash
cargo run -p fluxlock-protocol --bin fluxlock-demo -- --attack
### ⚔ Attack Mode

Simulate repeated attempts to reuse expired identity:

```bash
cargo run -p fluxlock-protocol --bin fluxlock-demo -- --attack

**Time-bound cryptographic identity enforced at the protocol level**

---

## 🧠 The Problem

Most systems treat identity as permanent.

If a key is compromised, access often persists indefinitely.

Even with stronger cryptography, this assumption remains unchanged.

> Identity itself does not expire.

---

## 🔐 The Idea

Fluxlock introduces a different model:

> **Identity is not permanent — it must evolve or becomes invalid**

---

## ⚙️ What Fluxlock Does

- Enforces **mandatory key rotation**
- Uses **hybrid cryptography**
  - Ed25519 (classical)
  - Dilithium (post-quantum)
- Rejects **expired or out-of-sync identities**
- Slashes validators that accept invalid transactions
- Executes deterministically across all nodes

---

## 🚀 Why It Matters

In most systems:
key compromised → permanent access


In Fluxlock:
key compromised → temporary access → forced rotation → access lost


---

## 🎬 Live Demo

Run the protocol and observe identity enforcement in real time:

```bash
cargo run --bin fluxlock-protocol demo
What happens:
Identity rotates (commit → reveal)
Epoch advances
An outdated identity attempts to act
Transaction is rejected
Validator is slashed
🧪 Example Output
🚨 THREAT: Attempt to reuse expired credentials

🚨 PROTOCOL VIOLATION DETECTED
⚔ VALIDATOR SLASHED
🪓 New stake: 950000

❌ Transaction rejected: identity no longer valid
🔥 What Makes It Different
🔐 Hybrid classical + post-quantum signatures
🔁 Atomic identity rotation (commit → reveal)
⏱ Epoch-based identity enforcement
⚠️ Rejection of valid signatures if identity is outdated
⚔ Economic penalties for invalid execution

Fluxlock doesn’t just verify identity — it verifies when that identity is valid

⚠️ Status

Fluxlock is currently a prototype / research system

Core protocol: ✅ working
Identity rotation: ✅
Enforcement + slashing: ✅
Demo: ✅

Not yet production-ready.

🧭 Vision

Identity is not something you possess.

It is something you continuously maintain

📌 Impact

Fluxlock reduces the impact of:

key theft
credential leaks
delayed breach detection

By removing permanent access entirely.

🤝 Feedback

If you're working in:

cryptography
security engineering
identity systems
distributed systems

I’d value your perspective.

📜 License

MIT

key compromised → temporary access → forced rotation → access lost
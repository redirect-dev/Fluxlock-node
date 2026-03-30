# 🔐 Fluxlock

![Fluxlock Demo](assets/demo.gif)

> A protocol for time-bound, self-healing identity

Fluxlock introduces a new security model where identity is **not permanent** — it must evolve with time or becomes invalid.

---

## 🚀 Why Fluxlock?

In most systems:


key compromised → permanent access


In Fluxlock:


key compromised → temporary access → forced rotation → access lost


Identity is no longer static — it is **continuously enforced by time**.

---

## 🧠 Core Concept

Fluxlock defines identity as:


identity = evolving + time-bound + enforced


This means:

- Keys must be rotated regularly  
- Identities fall out of sync with the network  
- Outdated identities are automatically rejected  

Even valid cryptographic signatures can fail if the identity is no longer current.

---

## 🔥 What Makes It Different

- 🔐 Hybrid cryptography (Ed25519 + Dilithium)
- 🔁 Atomic key rotation (commit → reveal)
- ⏱ Epoch-based identity enforcement
- ⚠️ Automatic rejection of outdated identities
- 🛡 Validator slashing for invalid execution

Fluxlock doesn't just verify identity — it verifies **when that identity exists**.

---

## 🎬 Live Demo

Run the protocol and watch identity evolve in real time:

```bash
cargo run --bin fluxlock-protocol run
What happens in the demo:
Identity rotates (commit → reveal)
Epoch advances over time
A valid transaction is rejected because the identity is outdated
🧪 Example Output
🚀 FLUXLOCK LIVE SECURITY DEMO

⏱ Tick 3
🔐 EVENT: Identity Commit Initiated

⏱ Tick 4
🔁 EVENT: Identity Rotation Executed

⏱ Tick 10
⚠️ EVENT: Attempting transaction with outdated identity
❌ Block rejected: Account not rotated for current epoch
🛠 CLI Usage
# Run full demo
cargo run --bin fluxlock-protocol demo

# Generate a new identity
cargo run --bin fluxlock-protocol new-account

# Run live chain simulation
cargo run --bin fluxlock-protocol run
⚠️ Project Status

Fluxlock is currently a prototype / research system.

It demonstrates a new identity model but is not yet production-ready.

🧭 Vision

Fluxlock explores a future where:

Identity is not something you have —
it is something you continuously maintain.

📌 Why It Matters

Most systems assume:

secure key = long-term access

Fluxlock enforces:

secure key = temporary access

This reduces the impact of:

Key theft
Long-term credential leaks
Delayed detection breaches
🤝 Contributing

Ideas, feedback, and discussion are welcome.

📜 License

MIT
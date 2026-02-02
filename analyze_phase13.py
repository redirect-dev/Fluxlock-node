import pandas as pd
import matplotlib.pyplot as plt

# Files to analyze
nodes = {
    "Honest Node (node1)": "node1_log.csv",
    "Delayed Rotator (node3)": "node3_log.csv",
}

plt.figure(figsize=(10, 6))

# ---- TRUST OVER TIME ----
for label, file in nodes.items():
    df = pd.read_csv(file)
    plt.plot(df["tick"], df["trust"], marker="o", label=label)

plt.title("Phase 13: Trust vs Time (Delayed Rotation Attack)")
plt.xlabel("Tick")
plt.ylabel("Trust")
plt.ylim(0, 1.05)
plt.grid(True)
plt.legend()
plt.tight_layout()
plt.savefig("docs/phase13/trust_vs_time.png")
plt.close()

# ---- KEY AGE OVER TIME ----
plt.figure(figsize=(10, 6))

for label, file in nodes.items():
    df = pd.read_csv(file)
    plt.plot(df["tick"], df["key_age"], marker="o", label=label)

plt.title("Phase 13: Key Age vs Time")
plt.xlabel("Tick")
plt.ylabel("Key Age")
plt.grid(True)
plt.legend()
plt.tight_layout()
plt.savefig("docs/phase13/key_age_vs_time.png")
plt.close()

print("Phase 13 analysis complete. Plots saved to docs/phase13/")

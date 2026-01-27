import pandas as pd
import matplotlib.pyplot as plt
import glob
import os

# Create docs/phase8 directory if it doesn't exist
os.makedirs("docs/phase8", exist_ok=True)

# Find all node log CSVs
node_logs = glob.glob("*_log.csv")

# Plot Reputation over Time
plt.figure(figsize=(12, 8))
plt.subplot(3, 1, 1)
for file in node_logs:
    df = pd.read_csv(file)
    plt.plot(df['tick'], df['reputation'], label=df['node'][0])
plt.title("Node Reputation Over Time")
plt.xlabel("Tick")
plt.ylabel("Reputation")
plt.legend()
plt.grid(True)
plt.tight_layout()
plt.savefig("docs/phase8/reputation_plot.png")

# Plot Entropy over Time
plt.subplot(3, 1, 2)
for file in node_logs:
    df = pd.read_csv(file)
    plt.plot(df['tick'], df['entropy'], label=df['node'][0])
plt.title("Node Entropy Over Time")
plt.xlabel("Tick")
plt.ylabel("Entropy")
plt.legend()
plt.grid(True)
plt.tight_layout()
plt.savefig("docs/phase8/entropy_plot.png")

# Plot Weighted Consensus over Time
plt.subplot(3, 1, 3)
for file in node_logs:
    df = pd.read_csv(file)
    plt.plot(df['tick'], df['weighted_consensus'], label=df['node'][0])
plt.title("Weighted Consensus Over Time")
plt.xlabel("Tick")
plt.ylabel("Weighted Consensus")
plt.legend()
plt.grid(True)
plt.tight_layout()
plt.savefig("docs/phase8/weighted_consensus_plot.png")

plt.show()
print("Phase 8 plots saved in docs/phase8/")

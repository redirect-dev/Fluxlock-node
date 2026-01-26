import pandas as pd
import matplotlib.pyplot as plt

# Load summaries
entropy_file = "entropy_summary.csv"
reputation_file = "reputation_summary.csv"

entropy_df = pd.read_csv(entropy_file, index_col=0)
reputation_df = pd.read_csv(reputation_file, index_col=0)

# Define nodes
nodes = entropy_df.columns.tolist()
adversarial_nodes = ["node3"]  # change if different

# --- Plot Entropy ---
plt.figure(figsize=(12, 6))
for node in nodes:
    if node in adversarial_nodes:
        plt.plot(entropy_df.index, entropy_df[node], label=f"{node} (adversarial)", linestyle="--", linewidth=2, color="red")
    else:
        plt.plot(entropy_df.index, entropy_df[node], label=node, linewidth=2)
plt.title("Node Entropy over Time")
plt.xlabel("Tick")
plt.ylabel("Entropy")
plt.legend()
plt.grid(True)
plt.tight_layout()
plt.savefig("entropy_plot.png")
plt.show()

# --- Plot Reputation ---
plt.figure(figsize=(12, 6))
for node in nodes:
    if node in adversarial_nodes:
        plt.plot(reputation_df.index, reputation_df[node], label=f"{node} (adversarial)", linestyle="--", linewidth=2, color="red")
    else:
        plt.plot(reputation_df.index, reputation_df[node], label=node, linewidth=2)
plt.title("Node Reputation over Time")
plt.xlabel("Tick")
plt.ylabel("Reputation")
plt.legend()
plt.grid(True)
plt.tight_layout()
plt.savefig("reputation_plot.png")
plt.show()

print("Plots created: entropy_plot.png and reputation_plot.png")

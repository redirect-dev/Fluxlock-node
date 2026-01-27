import pandas as pd
import matplotlib.pyplot as plt
import glob
import os

# -------------------------------
# Load all node CSVs
# -------------------------------
csv_files = glob.glob("node*_log.csv")  # Adjust if your CSVs are named differently

# Dictionary to hold each node's DataFrame
node_data = {}

for file in csv_files:
    node_name = os.path.splitext(os.path.basename(file))[0]  # e.g., 'node1_log'
    df = pd.read_csv(file)
    node_data[node_name] = df

# -------------------------------
# Plot Reputation for all nodes
# -------------------------------
plt.figure(figsize=(12, 6))
for node_name, df in node_data.items():
    plt.plot(df['tick'], df['reputation'], label=node_name)

plt.xlabel("Tick")
plt.ylabel("Reputation")
plt.title("Node Reputation Over Time - Phase 9")
plt.legend()
plt.grid(True)
plt.tight_layout()
plt.savefig("phase9_reputation_plot.png", dpi=300)
print("Reputation plot saved as phase9_reputation_plot.png")
plt.show()

# -------------------------------
# Optional: Plot other metrics (entropy, weighted_consensus, stake)
# -------------------------------
metrics = ['entropy', 'weighted_consensus', 'stake']
for metric in metrics:
    plt.figure(figsize=(12, 6))
    for node_name, df in node_data.items():
        plt.plot(df['tick'], df[metric], label=node_name)
    plt.xlabel("Tick")
    plt.ylabel(metric.replace("_", " ").title())
    plt.title(f"{metric.replace('_', ' ').title()} Over Time - Phase 9")
    plt.legend()
    plt.grid(True)
    plt.tight_layout()
    plt.savefig(f"phase9_{metric}_plot.png", dpi=300)
    print(f"{metric} plot saved as phase9_{metric}_plot.png")
    plt.show()

# -------------------------------
# Create Summary CSV
# -------------------------------
summary_list = []

ticks = node_data[list(node_data.keys())[0]]['tick']  # assume all nodes share the same ticks
for i, tick in enumerate(ticks):
    row = {'tick': tick}
    reputations = [df['reputation'].iloc[i] for df in node_data.values()]
    stakes = [df['stake'].iloc[i] for df in node_data.values()]
    
    row.update({
        'avg_reputation': sum(reputations)/len(reputations),
        'min_reputation': min(reputations),
        'max_reputation': max(reputations),
        'avg_stake': sum(stakes)/len(stakes)
    })
    
    summary_list.append(row)

summary_df = pd.DataFrame(summary_list)
summary_df.to_csv("phase9_summary.csv", index=False)
print("Summary CSV saved as phase9_summary.csv")

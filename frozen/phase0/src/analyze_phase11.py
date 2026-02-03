import pandas as pd
import matplotlib.pyplot as plt
import glob
import os

# Directory where your CSV logs are
csv_folder = "./"  # adjust if needed

# Find all node CSVs
csv_files = sorted(glob.glob(os.path.join(csv_folder, "node*_log.csv")))

# Initialize a figure
fig, axes = plt.subplots(2, 2, figsize=(12, 10))
axes = axes.flatten()

metrics = ["entropy", "weighted_consensus", "reputation", "stake"]
colors = ["blue", "green", "red", "purple"]

# Plot each metric
for i, metric in enumerate(metrics):
    ax = axes[i]
    for csv_file in csv_files:
        df = pd.read_csv(csv_file)
        node_name = os.path.basename(csv_file).split("_")[0]  # e.g., 'node1'
        if metric in df.columns:
            ax.plot(df['tick'], df[metric], label=node_name)
    ax.set_title(f"{metric.capitalize()} over time")
    ax.set_xlabel("Tick")
    ax.set_ylabel(metric.capitalize())
    ax.legend()
    ax.grid(True)

plt.tight_layout()

# Save figures
plt.savefig("phase11_metrics_plot.png")
plt.show()

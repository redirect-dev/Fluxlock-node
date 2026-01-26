import glob
import csv
import pandas as pd

# Find all node log CSVs
log_files = glob.glob("node*_log.csv")

all_data = []

for file in log_files:
    node = file.split("_")[0]  # node1_log.csv -> node1
    with open(file, "r") as f:
        reader = csv.DictReader(f)
        for row in reader:
            if row['tick'].isdigit():  # skip headers or FINISHED
                all_data.append({
                    "tick": int(row['tick']),
                    "node": row['node'],
                    "entropy": int(row['entropy']),
                    "consensus": int(row['consensus']),
                    "delta": int(row['delta']),
                    "reputation": float(row['reputation'])
                })

# Convert to pandas DataFrame
df = pd.DataFrame(all_data)

# Pivot table for easier analysis
entropy_table = df.pivot(index="tick", columns="node", values="entropy")
reputation_table = df.pivot(index="tick", columns="node", values="reputation")

# Save tables for reference
entropy_table.to_csv("entropy_summary.csv")
reputation_table.to_csv("reputation_summary.csv")

print("Analysis complete!")
print("Entropy summary: entropy_summary.csv")
print("Reputation summary: reputation_summary.csv")


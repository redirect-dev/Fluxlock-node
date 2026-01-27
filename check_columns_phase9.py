import pandas as pd
import glob
import os

# Change this to where your logs are
path = "docs/phase9"

# Find all CSVs starting with "node" in that folder
csv_files = glob.glob(os.path.join(path, "node*_log.csv"))

for f in csv_files:
    df = pd.read_csv(f)
    print(f"Columns in {os.path.basename(f)}: {df.columns.tolist()}")

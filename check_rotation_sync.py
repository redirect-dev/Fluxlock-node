# check_rotation_sync.py
import glob
import pandas as pd
from collections import defaultdict

files = sorted(glob.glob("node*_log.csv"))
if not files:
    print("No node log files found.")
    raise SystemExit(1)

# read key_hashes by tick for each node
data = {}
for f in files:
    df = pd.read_csv(f)
    node = df['node'].iloc[0]
    data[node] = df[['tick', 'key_hash', 'key_age']].set_index('tick')

# check ticks in common
ticks = sorted(set.union(*(set(df.index) for df in data.values())))

# report
drift_found = False
for t in ticks:
    hashes = defaultdict(list)
    ages = {}
    for node, df in data.items():
        if t in df.index:
            kh = int(df.at[t, 'key_hash'])
            ka = int(df.at[t, 'key_age'])
            hashes[kh].append(node)
            ages[node] = ka
        else:
            print(f"Missing tick {t} for {node}")
            drift_found = True

    if len(hashes) > 1:
        drift_found = True
        print(f"[Tick {t}] Key hash mismatch between nodes:")
        for kh, nodes in hashes.items():
            print(f"  hash {kh}: {nodes}")
    # key_age check
    # key_age should be identical across nodes if rotation synced
    if len(set(ages.values())) > 1:
        drift_found = True
        print(f"[Tick {t}] key_age differs across nodes: {ages}")

if not drift_found:
    print("Rotation sync check: PASS — all nodes rotated in sync and key hashes match where expected.")
else:
    print("Rotation sync check: FAIL — see messages above.")

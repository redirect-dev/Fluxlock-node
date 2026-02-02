import pandas as pd
import matplotlib.pyplot as plt
import os

nodes = ['node1', 'node2', 'node3', 'node4', 'node5']
metrics = ['decision', 'weighted_decision', 'trust']

os.makedirs('docs/phase11', exist_ok=True)

for metric in metrics:
    plt.figure(figsize=(10, 6))
    for node in nodes:
        df = pd.read_csv(f'{node}_log.csv')
        if metric not in df.columns:
            print(f"Warning: {metric} not found in {node}_log.csv")
            continue
        plt.plot(df['tick'], df[metric], label=node)
    
    plt.title(f'Phase 11: {metric} over time')
    plt.xlabel('Tick')
    plt.ylabel(metric)
    plt.legend()
    plt.grid(True)
    plt.tight_layout()
    plt.savefig(f'docs/phase11/phase11_{metric}_plot.png')
    plt.close()
    print(f"Saved plot for {metric}")

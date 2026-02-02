import pandas as pd

nodes = ['node1', 'node2', 'node3', 'node4', 'node5']
metrics = ['entropy', 'weighted_consensus', 'reputation', 'stake']

for node in nodes:
    df = pd.read_csv(f'{node}_log.csv')
    print(f"{node} columns: {df.columns.tolist()}")  # Check actual headers

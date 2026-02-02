import pandas as pd
import matplotlib.pyplot as plt
import glob
import os

os.makedirs("docs/phase12", exist_ok=True)

files = glob.glob("node*_log.csv")

metrics = {
    "trust": "Trust Over Time",
    "key_age": "Key Age Over Time"
}

for metric, title in metrics.items():
    plt.figure(figsize=(10, 6))

    plotted = False
    for file in files:
        df = pd.read_csv(file)
        if metric not in df.columns:
            print(f"Skipping {file}, missing {metric}")
            continue

        node = df["node"].iloc[0]
        plt.plot(df["tick"], df[metric], label=node)
        plotted = True

    if plotted:
        plt.title(title)
        plt.xlabel("Tick")
        plt.ylabel(metric)
        plt.legend()
        plt.grid(True)

        output = f"docs/phase12/{metric}_plot.png"
        plt.savefig(output)
        print(f"Saved {output}")
    else:
        print(f"No data plotted for {metric}")

    plt.close()

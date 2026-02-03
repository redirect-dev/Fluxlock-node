import matplotlib.pyplot as plt
import numpy as np
import os

# Create output directory
os.makedirs("docs/phase21", exist_ok=True)

# Time axis (attack duration)
time = np.arange(1, 21)

# Static key attack cost (flat persistence)
static_cost = np.ones_like(time) * 10

# FluxLock attack cost (increasing due to rotation + trust decay)
rotation_penalty = 1.2
fluxlock_cost = np.array([10 * (rotation_penalty ** t) for t in time])

# Plot
plt.figure(figsize=(10, 6))
plt.plot(time, static_cost, label="Static-Key System", linestyle="--")
plt.plot(time, fluxlock_cost, label="FluxLock Protocol", linewidth=2)

plt.xlabel("Attack Duration (Time Units)")
plt.ylabel("Cumulative Attacker Cost")
plt.title("Attack Cost Over Time: Static Keys vs FluxLock")
plt.legend()
plt.grid(True)

# Save plot
output_path = "docs/phase21/attack_cost_comparison.png"
plt.savefig(output_path)
plt.close()

print(f"Phase 21 attack cost plot saved to {output_path}")

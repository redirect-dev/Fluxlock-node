# Phase 12 — Key Rotation Results

## Summary
Phase 12 introduces deterministic per-node key rotation (interval = 3 ticks) and records `key_hash` and `key_age` per tick. The goals were to (1) verify consistent rotation across nodes, and (2) confirm that key rotation does not destabilize the reputation/trust model.

## Key Findings
- **Synchronized rotation:** All nodes rotate keys in lockstep at ticks 3, 6, 9... (see `key_age` plot). No rotation drift observed in the standard run.
- **Trust stable across rotation:** Trust trajectories for all nodes show no abrupt changes aligned with rotation events. Nodes with decreasing trust (e.g. node4) reflect decision behavior, not rotation artifacts.
- **Auditability:** Each rotation event is logged with `key_hash` and `key_age`, enabling post-facto verification and forensic analysis.

## Artifacts
- `docs/phase12/key_age_plot.png` — visual confirmation of rotation schedule
- `docs/phase12/trust_plot.png` — trust vs tick (no rotation-induced drops)
- Node logs: `node1_log.csv` ... `node5_log.csv` (contain `key_hash` and `key_age`)

## Next steps
1. Add an automated rotation-sync validator that checks whether nodes rotate at the same ticks and whether `key_hash` actually changed.  
2. Simulate partial/failed rotations (lagging node or malicious node refusing rotation) and measure consensus impact.  
3. Integrate a post-quantum key generation routine and repeat tests.


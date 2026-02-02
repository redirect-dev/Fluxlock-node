# Phase 11 – Node Architecture and Roles

## Step 1: Define Node Roles

Fluxlock operates with three primary node types:

### 1. Proposer Nodes
- Responsible for suggesting new states, key rotations, or protocol updates.
- Must adhere to protocol constraints to ensure network consistency.
- Voting power is proportional to the node's reputation and stake.

### 2. Validator Nodes
- Confirm and vote on proposals submitted by Proposer Nodes.
- Weighted consensus determines whether proposals are accepted.
- Reputation and stake influence voting weight, ensuring trust and accountability.

### 3. Observer Nodes
- Monitor system health, entropy, and reputation metrics.
- Report anomalies or irregular behavior to the network.
- Do not participate in voting but provide oversight for network integrity.

This structure ensures that Fluxlock maintains both **resilience** and **quantum-safe integrity** by distributing roles and responsibilities among nodes while providing a clear mechanism for proposal validation and monitoring.

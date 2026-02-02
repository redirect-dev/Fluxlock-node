#!/bin/bash

# Make sure the Rust project is built in release mode
echo "Building Rust nodes..."
cargo build --release

# Array of node names
nodes=("node1" "node2" "node3" "node4" "node5")

# Run each node one by one
for node in "${nodes[@]}"; do
    echo "Starting $node..."
    ./target/release/fluxlock-node "$node"
    echo "$node FINISHED"
done

echo "All nodes finished Phase 9 simulation."

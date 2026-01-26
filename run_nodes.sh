#!/bin/bash

# Run only 3 nodes to test fault tolerance
cargo run --release -- node1 &
cargo run --release -- node3 &
cargo run --release -- node5 &

# Wait for all processes
wait

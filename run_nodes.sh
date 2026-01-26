#!/bin/bash

# Remove old network state
rm -f network_state.txt

# Number of ticks for simulation
TICKS=50

# Launch nodes
for i in 1 2 3 4 5
do
    echo "Starting node$i"
    ./target/release/fluxlock-node.exe node$i $TICKS &
done

# Wait for all nodes to finish
wait

echo "All nodes finished simulation."

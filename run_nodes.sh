#!/bin/bash
# Run 5 FluxLock nodes in parallel and log their states

for i in 1 2 3 4 5
do
    cargo run --release -- node$i > node${i}_log.txt 2>&1 &
    echo "Node $i started..."
done

wait
echo "All nodes finished."

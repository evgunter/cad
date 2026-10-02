#!/bin/bash
export CARGO_TARGET_DIR=/home/user/cad/target CARGO_INCREMENTAL=0 CAD_TOLERANCE_EPS=1e-9
out=$(timeout 1500 cargo test -p sweep --features probe --test all -- recorded::r1_ring 2>&1)
echo "$out" | grep -q "r1_ring_clearance_decisions_per_carve ... ok" && exit 0
echo "$out" | grep -q "r1_ring_clearance_decisions_per_carve ... FAILED" && exit 1
exit 125

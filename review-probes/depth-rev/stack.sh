#!/bin/bash
# usage: stack.sh <bytes-expr>  runs part_depth_bound rows with WASM_STACK replaced
set -u
W=/home/user/cad/.claude/worktrees/agent-af0c53382b4790f55
T=$W/crates/editor-core/tests/part_depth_bound.rs
cp $T /home/user/depth-rev-scratch/pdb.rs.bak
sed -i "s/^const WASM_STACK: usize = 1 << 20;/const WASM_STACK: usize = $1;/" $T
grep -n "^const WASM_STACK" $T
cd $W
export CARGO_TARGET_DIR=/home/user/depth-rev-target CARGO_INCREMENTAL=0
timeout 400 cargo test -p editor-core --test all --no-run 2>&1 | grep -E "^error|Executable"
cd $W/crates/editor-core
BIN=$(ls -t /home/user/depth-rev-target/debug/deps/all-* | grep -v '\.d$' | head -1)
timeout 120 $BIN part_depth_bound::a_chain_one_past --test-threads 1 2>&1 | grep -E "^test |test result|overflow" | cut -c1-160
echo "one-past exit=${PIPESTATUS[0]}"
timeout 120 $BIN part_depth_bound::the_refused --test-threads 1 2>&1 | grep -E "^test |test result|overflow" | cut -c1-160
echo "at-bound exit=${PIPESTATUS[0]}"
cp /home/user/depth-rev-scratch/pdb.rs.bak $T

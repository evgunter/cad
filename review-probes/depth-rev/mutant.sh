#!/bin/bash
# usage: mutant.sh <name>   applies mutant to a fresh copy of head parts.rs, builds, runs part_depth_bound rows
set -u
W=/home/user/cad/.claude/worktrees/agent-af0c53382b4790f55
P=$W/crates/editor-core/src/eval/parts.rs
cp /home/user/depth-rev-scratch/head-parts.rs $P
case "$1" in
  recursion) python3 - "$P" <<'EOF'
import sys; p=sys.argv[1]; s=open(p).read()
old="if let Some(child) = current.next_unreached(self.eps_bits) {"; assert s.count(old)==1
s=s.replace(old,"if let Some(child) = None::<DocRef>.or_else(|| { let _ = current.next; None }) {"); open(p,"w").write(s)
EOF
  ;;
  up) sed -i 's/pub(crate) const MAX_DEPTH: usize = 1024;/pub(crate) const MAX_DEPTH: usize = 1025;/' $P ;;
  down) sed -i 's/pub(crate) const MAX_DEPTH: usize = 1024;/pub(crate) const MAX_DEPTH: usize = 1023;/' $P ;;
  cmp) sed -i 's/if path.len() >= MAX_DEPTH {/if path.len() > MAX_DEPTH {/' $P ;;
  none) ;;
esac
grep -c "MAX_DEPTH: usize = 1024" $P
cd $W
export CARGO_TARGET_DIR=/home/user/depth-rev-target CARGO_INCREMENTAL=0
timeout 400 cargo test -p editor-core --test all --no-run 2>&1 | grep -E "^error|Executable"
cd $W/crates/editor-core
BIN=$(ls -t /home/user/depth-rev-target/debug/deps/all-* | grep -v '\.d$' | head -1)
timeout 180 $BIN part_depth_bound 2>&1 | grep -E "^test |test result|panicked at|overflow|signal" | cut -c1-200
echo "exit=${PIPESTATUS[0]}"
cp /home/user/depth-rev-scratch/head-parts.rs $P

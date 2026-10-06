#!/bin/bash
cd /tmp/claude-0/probe-wt
export CARGO_TARGET_DIR=/tmp/claude-0/tgt/probe
for m in baseline first last other norefuse nobound twice trimmed_off; do
  git checkout -q -- crates/mesh/src/planar.rs crates/mesh/src/trimmed.rs
  python3 - <<'P'
p='/tmp/claude-0/probe-wt/crates/mesh/src/planar.rs'
s=open(p).read()
s=s.replace("#[cfg(test)]\nmod tests {","#[cfg(test)]\n#[path = \"planar_pinch_probes.rs\"]\nmod pinch_probes;\n\n#[cfg(test)]\nmod tests {",1)
open(p,'w').write(s)
P
  [ $m != baseline ] && python3 /tmp/claude-0/scratch/mutate.py $m
  echo "=== $m"
  cargo test -p mesh --lib pinch 2>&1 | grep -E "^test |test result"
  cargo test -p sweep --test all pinch_faces_tessellate 2>&1 | grep -E "panicked|test result|^test " | head -6
done
git checkout -q -- crates/mesh/src/trimmed.rs

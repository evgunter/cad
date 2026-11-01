#!/usr/bin/env bash
# Temporary investigation script (NOT for merge): per-crate LLVM IR census.
set -u
cd "$(dirname "$0")/.."
OUT=/tmp/bt-llvm
mkdir -p $OUT

# Crates that DEFINE or INSTANTIATE the Real generics, plus editor-core
# (the workspace's largest codegen unit) as the control.
CRATES="geom-core geom geom-brep topo profile sweep editor-core"

for c in $CRATES; do
  echo "=== llvm-lines -p $c --lib --release (default features) ==="
  cargo llvm-lines -p "$c" --lib --release > "$OUT/$c.rel.txt" 2> "$OUT/$c.rel.err"
  head -1 "$OUT/$c.rel.txt"; sed -n '2,18p' "$OUT/$c.rel.txt"
  echo
done

echo "=== TOTAL rows ==="
grep -H "(TOTAL)" $OUT/*.txt

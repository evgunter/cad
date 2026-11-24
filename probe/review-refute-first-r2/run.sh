#!/bin/bash
. ~/.local/share/cad-work/review-refute-first/env.sh
export CARGO_TARGET_DIR=/home/user/review-refute-target
cd /home/user/cad
for m in "$@"; do
  python3 ~/.local/share/cad-work/review-refute-first/mut/mutants.py $m apply || { echo "$m APPLY-FAIL"; continue; }
  out=$(timeout 1200 cargo nextest run -p geom-core --lib --no-fail-fast -E 'test(/sym::/)' 2>&1)
  echo "== $m: $(echo "$out" | grep -E '^\s+Summary' )"
  echo "$out" | grep -E '^\s+FAIL \[' | sed 's/^/   /' | sort -u | head -20
  echo "$out" | grep -E '^error' | head -5
  git checkout -q -- crates/geom-core/src/sym.rs crates/geom-core/src/sym/point.rs crates/geom-core/src/sym/question.rs
done

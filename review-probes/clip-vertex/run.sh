#!/bin/bash
S=$(cd "$(dirname "$0")" && pwd)
cd /home/user/cad
export CARGO_TARGET_DIR=/home/user/review-target
unset CAD_TOLERANCE_EPS
for n in $(python3 -c "import sys; sys.path.insert(0,'$S'); from mutants import M; print(' '.join(M))"); do
  git checkout -q 5421491599 -- crates/topo/src/injective.rs
  python3 $S/apply.py crates/topo/src/injective.rs $n $S || { echo "$n APPLYFAIL"; continue; }
  start=$(date +%s)
  u=$(cargo nextest run -p topo --lib -E 'test(/^injective::/)' --no-fail-fast 2>&1)
  if echo "$u" | grep -q "^error\[\|could not compile"; then echo "$n COMPILEFAIL"; echo "$u" | grep -A5 "^error" | head -20; continue; fi
  s=$(cargo nextest run -p sweep --test all -E 'test(/self_intersection_proves_injectivity::/) & !test(/probe_/)' --no-fail-fast 2>&1)
  echo "== $n ($(( $(date +%s) - start ))s)"
  echo "$u" | grep -E "^\s+Summary"; echo "$u" | grep -E "^\s+FAIL" | sort -u | sed 's/.*injective::/  unit: /'
  echo "$s" | grep -E "^\s+Summary"; echo "$s" | grep -E "^\s+FAIL" | sort -u | sed 's/.*proves_injectivity::/  suite: /'
done
git checkout -q 5421491599 -- crates/topo/src/injective.rs
echo ALLDONE

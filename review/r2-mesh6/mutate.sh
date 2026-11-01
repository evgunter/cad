#!/usr/bin/env bash
set -u
LANE=/root/lanes/mesh-6r2
run() {
  cd $LANE
  out=$(timeout 1200 local-scripts/with-build-slot.sh -- cargo test -p mesh --lib -- $2 2>&1)
  if echo "$out" | grep -qE "^error\[|^error: could not compile|cannot find|evaluation of .* failed"; then
    echo "  [$1] RED BY COMPILE ERROR"
    echo "$out" | grep -E "^error" | head -4 | sed 's/^/     /'
  fi
  echo "$out" | grep -E "^(test result|failures:$)" | sed "s/^/  [$1] /"
  echo "$out" | grep -E "^    (curved|tessellate)::" | sed 's/^/     FAILED: /'
}
revert() {
  cp $LANE/review/r2-mesh6/curved.orig $LANE/crates/mesh/src/curved.rs
  cp $LANE/review/r2-mesh6/tessellate.orig $LANE/crates/mesh/src/tessellate.rs
}

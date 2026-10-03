#!/usr/bin/env bash
# Run the suites for one tree state, writing raw outputs into a raw dir.
# Uses --show-output so harness rows stay clean and test stdout lands in
# the "successes:" section. Usage: r1m4-runs.sh <tree> <rawdir>
set -euo pipefail
TREE="$1"; RAW="$2"
mkdir -p "$RAW"
cd "$TREE"
SLOT=/root/lanes/mesh-4r1/local-scripts/with-build-slot.sh
run() { # run <outfile> <cargo args...>
  local out="$1"; shift
  [ -s "$out" ] && grep -q 'test result:' "$out" && { echo "skip $out (done)"; return 0; }
  "$SLOT" --express 580 -- cargo "$@" > "$out" 2>&1 \
    || { echo "FAILED: $out"; tail -30 "$out"; exit 1; }
}
for BAND in default 1e-6 1e-12; do
  if [ "$BAND" = default ]; then unset CAD_TOLERANCE_EPS; else export CAD_TOLERANCE_EPS="$BAND"; fi
  run "$RAW/mesh.$BAND.txt" test -p mesh --test all -- --test-threads=1 --show-output
  run "$RAW/pg.$BAND.txt" test -p step-import --test all poleguard -- --test-threads=1 --show-output
  run "$RAW/budget.$BAND.txt" test -p mesh --test all --features budget -- --test-threads=1
done
unset CAD_TOLERANCE_EPS || true
echo "runs done: $RAW"

#!/bin/bash
# Lever B: the SSI suites under three levers at three ε; per-test verdict lines.
BIN=/home/user/tgt-B/debug/deps/all-39c92c0ced009b1d
OUTDIR=${1:-/home/user/lever-b-suites}
mkdir -p "$OUTDIR"
SUITES="m5_pr7_ssi review_m5_pr7b_ssi ssi_limb3_one_arc review_m5_pr7_adversarial review_m5_pr7_enclosure"
run() {
  eps=$1; lever=$2
  CAD_TOLERANCE_EPS=$eps LEVER_PROBE=$lever "$BIN" $SUITES --test-threads=2 > "$OUTDIR/$eps-$lever.log" 2>&1
  grep -E "^test .* \.\.\. (ok|FAILED|ignored)" "$OUTDIR/$eps-$lever.log" | sort > "$OUTDIR/$eps-$lever.verdicts"
  grep -E "^test result" "$OUTDIR/$eps-$lever.log" > "$OUTDIR/$eps-$lever.summary"
}
for eps in 1e-9 1e-12 1e-6; do
  run $eps chart &
  run $eps normal &
  wait
  run $eps extent
done
echo DONE > "$OUTDIR/DONE"

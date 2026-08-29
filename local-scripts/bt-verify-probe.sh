#!/usr/bin/env bash
# Temporary investigation script (NOT for merge): probe-lane verification.
#
# CI covers the default and interval lanes on every PR. NOTHING covers the
# `probe` lane except the k-lint job's K sweep, so that sweep is the thing
# most worth exercising locally before pushing: if the feature plumbing is
# wrong, it does not fail loudly — it lints an EMPTY CSV and passes.
set -u
cd "$(dirname "$0")/.."
OUT=/tmp/bt-ksweep
rm -rf $OUT && mkdir -p $OUT

echo "### 1. the K sweep end to end (the k-lint gate's input)"
s=$(date +%s)
scripts/k_probe_sweep.sh $OUT > /tmp/bt-ksweep.log 2>&1
rc=$?
echo "k_probe_sweep.sh rc=$rc in $(( $(date +%s) - s ))s"
for f in $OUT/k-eps-*.csv; do
  [ -e "$f" ] || { echo "MISSING: no CSVs produced"; break; }
  rows=$(( $(wc -l < "$f") - 1 ))
  preds=$(tail -n +2 "$f" | cut -d, -f2 | sort -u | wc -l)
  printf '  %-28s rows=%-8d distinct predicates=%d\n' "$(basename "$f")" "$rows" "$preds"
done
echo "  (an empty or tiny CSV here is the silent-failure mode this check exists for)"

echo
echo "### 2. the lint that consumes them, exactly as CI runs it"
(cd tools/k-lint && cargo run -q -- $OUT/k-eps-1e-6.csv $OUT/k-eps-1e-9.csv $OUT/k-eps-1e-12.csv 2>&1 | tail -5)
echo "k-lint rc=$?"

echo
echo "### 3. full suite on the probe lane (opt-2 for speed)"
export CARGO_PROFILE_DEV_OPT_LEVEL=2 CARGO_PROFILE_TEST_OPT_LEVEL=2
cargo nextest run --workspace --features probe --no-fail-fast 2>&1 | tail -4

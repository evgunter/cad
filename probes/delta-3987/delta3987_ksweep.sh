#!/usr/bin/env bash
# The nightly's dev-probe dumps + k-lint, minus the plain probe-suite
# section (which stops on main at tilted_sphere_pair_k_rows), at 1e-6 and
# 1e-9 (the lane reports 1e-12 stops at lily_leaf_b on both trees).
set -uo pipefail
tree=$1; out=$2; export CARGO_TARGET_DIR=$3
cd "$tree"; mkdir -p "$out/m2" "$out/driver"
for eps in 1e-6 1e-9; do
  CAD_TOLERANCE_EPS=$eps CAD_K_REPORT_OUT="$out/.corpus-$eps.csv" cargo test -q -p editor-core --features probe --test all -- --ignored m4_pr8_k_probe:: > "$out/log-corpus-$eps.txt" 2>&1 || echo "corpus $eps FAILED"
  (cd demos/tour && CAD_TOLERANCE_EPS=$eps cargo run -q --features probe -- k-probe "$out/.demos-$eps.csv" > "$out/log-demos-$eps.txt" 2>&1) || echo "demos $eps FAILED"
  cat "$out/.corpus-$eps.csv" > "$out/k-eps-$eps.csv"; tail -n +2 "$out/.demos-$eps.csv" >> "$out/k-eps-$eps.csv"
  wc -l "$out/k-eps-$eps.csv"
done
(cd tools/k-lint && cargo run -q -- "$out/k-eps-1e-6.csv" "$out/k-eps-1e-9.csv") > "$out/k-lint.txt" 2>&1; echo "k-lint exit $?"
tail -5 "$out/k-lint.txt"

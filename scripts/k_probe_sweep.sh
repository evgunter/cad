#!/usr/bin/env bash
# K-telemetry probe sweep (M4 PR 8b, spec D3): collects the complete
# per-predicate margin distribution of the Band 4 corpus AND the demo
# tour scenes at the recording scalar `Probe`, one process per ε
# (`Tolerance` is a OnceLock), and merges the two dumps into one CSV
# per ε row (M2 file convention: shape,predicate,margin,band_zero,
# band_escalate,outcome; shapes namespaced corpus/<doc> and
# demo/<scene>).
#
# Usage:
#   scripts/k_probe_sweep.sh <outdir> [--gzip]
#
# Writes <outdir>/m4-eps-<ε>.csv (plus .gz and no plain CSV when
# --gzip). The committed baseline in docs/k-report-data/ was generated
# by exactly this script with --gzip (raw rows are ~160 MB/row; the
# gzipped baseline is the durable record — same rows, same columns).
# The ci `k-lint` job runs the same sweep into a scratch dir and lints
# the fresh rows against the thresholds pinned in tools/k-lint.
set -euo pipefail
cd "$(dirname "$0")/.."
root=$(pwd)

out_arg=${1:?usage: k_probe_sweep.sh <outdir> [--gzip]}
mkdir -p "$out_arg"
outdir=$(cd "$out_arg" && pwd)
gz=${2:-}

for eps in 1e-6 1e-9 1e-12; do
  corpus="$outdir/.corpus-$eps.csv"
  demos="$outdir/.demos-$eps.csv"
  merged="$outdir/m4-eps-$eps.csv"
  echo "=== k-probe sweep @ eps=$eps (corpus) ==="
  CAD_TOLERANCE_EPS=$eps CAD_K_REPORT_OUT="$corpus" \
    cargo test -p editor-core --test m4_pr8_k_probe -- --ignored --nocapture
  echo "=== k-probe sweep @ eps=$eps (demo scenes) ==="
  (cd "$root/demos/tour" && CAD_TOLERANCE_EPS=$eps cargo run -- k-probe "$demos")
  # One header, then both bodies — corpus first, demos second.
  cat "$corpus" > "$merged"
  tail -n +2 "$demos" >> "$merged"
  rm "$corpus" "$demos"
  if [ "$gz" = "--gzip" ]; then
    gzip -9 -f "$merged"
    echo "wrote $merged.gz"
  else
    echo "wrote $merged"
  fi
done

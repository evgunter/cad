#!/usr/bin/env bash
# Runs the acceptance suites with the scratch topo hook applied, one eps
# row per call: run_suites.sh <eps> <outdir>
set -u
eps=$1; out=$2
mkdir -p "$out"
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/home/user/cf-target}
CENSUS_FOLD_DIR="$out" CAD_TOLERANCE_EPS=$eps \
  cargo nextest run --release --no-fail-fast \
    -p topo -p sweep -p verbs -p step-import -p editor-core -p pncad \
    --test-threads 3 >"$out/nextest.log" 2>&1
echo "exit $?" >>"$out/nextest.log"

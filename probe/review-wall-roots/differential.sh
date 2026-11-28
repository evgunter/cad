#!/usr/bin/env bash
# The two-tree differential: geom-brep and topo whole (slow set included)
# at the default eps, geom-brep at 1e-6 and 1e-12, on the tree given.
# Usage: differential.sh <tree> <target> <label>
set -u
tree=$1; target=$2; label=$3; out=$HOME/.local/share/cad-work/review-wall-roots/diff-$label
mkdir -p "$out"; cd "$tree"
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=$target CARGO_PROFILE_DEV_OPT_LEVEL=1 CARGO_PROFILE_TEST_OPT_LEVEL=1 CARGO_PROFILE_DEV_DEBUG=line-tables-only CARGO_PROFILE_TEST_DEBUG=line-tables-only
run() { # name, extra env, packages...
  local name=$1; shift; local envs=$1; shift
  env $envs cargo nextest run "$@" --no-fail-fast --success-output never --failure-output final 2>&1 \
    | grep -E "^\s+(PASS|FAIL|SIGABRT|SIGSEGV|TIMEOUT|LEAK)|Summary" | sed -E 's/\[ *[0-9.]+s\] //' > "$out/$name.txt"
  echo "$name: $(tail -1 "$out/$name.txt")"
}
run gb-default "X=1" -p geom-brep
run gb-1e-6 "CAD_TOLERANCE_EPS=1e-6" -p geom-brep
run gb-1e-12 "CAD_TOLERANCE_EPS=1e-12" -p geom-brep
run topo-default "X=1" -p topo
echo "=== done $label"

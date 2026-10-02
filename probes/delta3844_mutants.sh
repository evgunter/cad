#!/usr/bin/env bash
# Run each mutant of probes/delta3844_mutate.py over the backstop's rows.
# Usage: probes/delta3844_mutants.sh <eps> <mutant>...
set -u
eps=$1; shift
export CARGO_INCREMENTAL=0 CAD_TOLERANCE_EPS=$eps
F='test(/volume_backstop/) | test(/door_backstop_settled_residue/) | test(/declared_rounded_continuations_inside_a_wall/) | test(/contact9_side_codes/) | test(/at_rest_policy_tests/) | test(/declaration_order_rows/)'
for m in "$@"; do
  git checkout -q crates/topo/src
  python3 probes/delta3844_mutate.py "$m" || { echo "MUTANT $m: did not apply"; continue; }
  echo "=== MUTANT $m eps=$eps"
  cargo nextest run -p topo -p sweep --no-fail-fast -E "$F" 2>&1 | grep -E "^\s+FAIL \[|Summary|^error\[|^error:" | sort -u
done
git checkout -q crates/topo/src

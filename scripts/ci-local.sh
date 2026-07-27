#!/usr/bin/env bash
# Local mirror of .github/workflows/ci.yml — the merge gate while hosted
# Actions is unavailable (GitHub free-plan minutes exhausted, 2026-07-22),
# and a pre-push check any time. Keep the two IN SYNC: a job added to
# ci.yml gets a row here, same commands, same env. Rows run sequentially
# (they share one target/ dir — cargo can't safely share it concurrently);
# all rows run even after a failure (ci.yml's fail-fast: false), summary
# at the end, nonzero exit if any row failed.
#
# Prereqs beyond the Rust toolchain: m4 (interval row's GMP/MPFR C build),
# admesh (watertight row; apt or built from source — 0.98.4+).
#
# Merge-gate runs go through scripts/gate.sh (serialized, warm runner —
# see its header for the caching guidance and RUSTFLAGS hazard).
set -u
cd "$(dirname "$0")/.."

declare -a NAMES RESULTS
run_row() {
  local name="$1"; shift
  echo
  echo "=== [$name] $*"
  local t0=$SECONDS
  if "$@"; then RESULTS+=("PASS $((SECONDS - t0))s"); else RESULTS+=("FAIL $((SECONDS - t0))s"); fi
  NAMES+=("$name")
}

# --- discipline (evaluation-code): the three tripwire greps, verbatim ---
discipline() {
  local rc=0
  if grep -rnE '\bReal\s*\+' crates/*/src; then
    echo "ERROR: found 'Real +' bound(s) above — evaluation-code discipline forbids extra bounds on scalar type parameters"
    rc=1
  fi
  # Production-consumer allowlist EMPTY since M4 PR 5 (N6 retirement):
  # remaining rows are non-consumers (the seam itself; interval.rs
  # scalar plumbing; memo.rs bit-hashing; source.rs debug assertion).
  if grep -rnE 'bit_identity::|repr_bits|eq_bits' crates/*/src \
    | grep -vE '^crates/geom-core/src/bit_identity\.rs:' \
    | grep -vE '^crates/geom-core/src/interval\.rs:' \
    | grep -vE '^crates/topo/src/source\.rs:' \
    | grep -vE '^crates/editor-core/src/eval/memo\.rs:' \
    | grep -vE ':[0-9]+:\s*//'; then
    echo "ERROR: bit-identity channel use above — RETIRED from production (M4 PR 5, N6); use GeomSource, or revise DESIGN.md first"
    rc=1
  fi
  uses=$(grep -cE 'bit_identity::|eq_bits' crates/topo/src/source.rs || true)
  gates=$(grep -c 'cfg(debug_assertions)' crates/topo/src/source.rs || true)
  if [ "$uses" -gt 0 ] && [ "$gates" -eq 0 ]; then
    echo "ERROR: topo/src/source.rs uses the bit channel without cfg(debug_assertions) gating"
    rc=1
  fi
  if grep -rnE 'downcast_ref|downcast_mut|TypeId|core::any|std::any' crates/*/src \
    | grep -vE '^crates/geom-core/src/bit_identity\.rs:' \
    | grep -vE ':[0-9]+:\s*//'; then
    echo "ERROR: bit-identity punning outside the sanctioned seam (geom-core/src/bit_identity.rs)"
    rc=1
  fi
  return $rc
}

watertight() {
  command -v admesh >/dev/null || { echo "ERROR: admesh not installed (apt admesh, or build 0.98.4+ from source)"; return 1; }
  cargo run -p stl --example export_acceptance -- target/stl-acceptance && \
    scripts/check_admesh.sh target/stl-acceptance
}

# External STEP import acceptance (M4 PR 7): FreeCAD/OCC imports the
# committed fixtures (kept byte-golden against the writer by the cargo
# test suite), asserting validity + exact counts + volume. The script
# SKIPS LOUDLY (exit 0) when freecadcmd is absent so this row stays
# hermetic on machines without FreeCAD — see its header for FREECADCMD
# discovery and REQUIRE_FREECAD.
step_import() {
  scripts/check_step.sh
}

test_eps() { CAD_TOLERANCE_EPS="$1" cargo test --workspace; }
interval_eps() { CAD_TOLERANCE_EPS=1e-6 cargo test --workspace --features interval; }

# M4 PR 6 spec D6: the three persistence obligations as NAMED rows
# (also covered by the workspace rows; named = attributable).
persist_roundtrip() {
  local e
  for e in 1e-6 1e-9 1e-12; do
    CAD_TOLERANCE_EPS="$e" cargo test -p editor-core --test m4_pr6_roundtrip --test m4_pr6_floats --test m4_pr6_golden || return 1
  done
}
persist_eps_diff() { cargo test -p editor-core --test m4_pr6_eps_diff; }
persist_refusal() { cargo test -p editor-core --test m4_pr6_refusal --test m4_pr6_review_probes --test profile_desc_key; }
persist_interval() { cargo test -p editor-core --features interval --test m4_pr6_roundtrip_interval; }

run_row "discipline (evaluation-code)" discipline
run_row "rustfmt"                      cargo fmt --all --check
run_row "clippy"                       cargo clippy --workspace --all-targets -- -D warnings
run_row "test"                         cargo test --workspace
run_row "test (eps = 1e-6)"            test_eps 1e-6
run_row "test (eps = 1e-9)"            test_eps 1e-9
run_row "test (eps = 1e-12)"           test_eps 1e-12
run_row "clippy (interval)"            cargo clippy --workspace --all-targets --features interval -- -D warnings
run_row "test (interval)"              cargo test --workspace --features interval
run_row "test (interval, eps = 1e-6)"  interval_eps
run_row "persist save/load/replay (D6.1)" persist_roundtrip
run_row "persist eps-diff golden (D6.2)"  persist_eps_diff
run_row "persist refusal (D6.3)"          persist_refusal
run_row "persist roundtrip (interval)"    persist_interval
run_row "watertight (admesh)"          watertight
run_row "step import (freecad)"        step_import

echo
echo "=== ci-local summary ==="
fail=0
for i in "${!NAMES[@]}"; do
  printf '%-32s %s\n' "${NAMES[$i]}" "${RESULTS[$i]}"
  [[ "${RESULTS[$i]}" == FAIL* ]] && fail=1
done
exit $fail

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
  if grep -rnE 'bit_identity::|repr_bits|eq_bits' crates/*/src \
    | grep -vE '^crates/geom-core/src/bit_identity\.rs:' \
    | grep -vE '^crates/geom-core/src/interval\.rs:' \
    | grep -vE '^crates/topo/src/merge_faces\.rs:' \
    | grep -vE '^crates/topo/src/boolean/plane_eq\.rs:' \
    | grep -vE ':[0-9]+:\s*//'; then
    echo "ERROR: new bit-identity channel consumer above — retirement-scheduled (DESIGN.md M4); allowlist in ci.yml AND here, plus a retirement note in its docs"
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

test_eps() { CAD_TOLERANCE_EPS="$1" cargo test --workspace; }
interval_eps() { CAD_TOLERANCE_EPS=1e-6 cargo test --workspace --features interval; }

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
run_row "watertight (admesh)"          watertight

echo
echo "=== ci-local summary ==="
fail=0
for i in "${!NAMES[@]}"; do
  printf '%-32s %s\n' "${NAMES[$i]}" "${RESULTS[$i]}"
  [[ "${RESULTS[$i]}" == FAIL* ]] && fail=1
done
exit $fail

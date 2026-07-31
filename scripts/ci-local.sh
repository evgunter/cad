#!/usr/bin/env bash
# CHANGE FILTER (2026-07-28): this script is filtered exactly like hosted
# CI, ON BY DEFAULT, because both call the SAME classifier —
# scripts/ci-filter.py. It reports one of three tiers for the change set
# (see that script's docstring): `docs` runs nothing, `all` runs this whole
# matrix unscoped, `closure` builds the changed workspace members plus every
# member that transitively depends on them and runs only the pipeline rows
# whose root package is in that closure. Filtered rows print SKIP (filter)
# in the summary, so what was skipped is always visible.
#
# Pass --full to force the `all` tier. That is the right call for:
#   * merge-gate runs on a suspect environment (mixed toolchains, a machine
#     you have not gated on before);
#   * post-crash / interrupted-run verification, where the tree state is not
#     trustworthy enough for a diff-derived answer;
#   * torn or poisoned target/ caches — the filter reasons about SOURCE
#     changes, not about what a cache actually contains;
#   * implementer full-battery obligations — the subagent battery prompts
#     ask for the full matrix deliberately, and a filtered run does not
#     discharge them.
# Pass --base <ref> to classify against something other than the merge-base
# with origin/main. Classification fails CLOSED: any uncertainty is `all`.
# Local mirror of .github/workflows/ci.yml — the merge gate while hosted
# Actions is unavailable (GitHub free-plan minutes exhausted, 2026-07-22),
# and a pre-push check any time. Keep the two IN SYNC: a job added to
# ci.yml gets a row here, same commands, same env. Rows run sequentially
# (they share one target/ dir — cargo can't safely share it concurrently);
# all rows run even after a failure (ci.yml's fail-fast: false), summary
# at the end, nonzero exit if any row failed.
#
# Prereqs beyond the Rust toolchain: admesh (watertight row; apt or built
# from source — 0.98.4+). Nothing here needs a C toolchain: the `interval`
# feature's backend is the in-repo, pure-Rust `interval-transcendentals`.
#
# Merge-gate runs go through scripts/gate.sh (serialized, warm runner —
# see its header for the caching guidance and RUSTFLAGS hazard).
set -u
cd "$(dirname "$0")/.."

# --- change filter: one shared implementation with .github/workflows/ci.yml
FULL=0
BASE=""
while [ $# -gt 0 ]; do
  case "$1" in
    --full) FULL=1; shift ;;
    --base) BASE="${2:?--base needs a ref}"; shift 2 ;;
    *) echo "usage: ci-local.sh [--full] [--base <ref>]" >&2; exit 2 ;;
  esac
done

TIER=all
SCOPE=--workspace
RUN_EDITOR_CORE=true
RUN_STL=true
RUN_STEP_EXPORT=true
RUN_INTERVAL_BACKEND=true
RUN_K_LINT=true
if [ "$FULL" -eq 1 ]; then
  echo "=== change filter: --full, forcing tier 'all'"
else
  if [ -z "$BASE" ]; then
    BASE=$(git merge-base HEAD origin/main 2>/dev/null || git merge-base HEAD main 2>/dev/null || echo HEAD~1)
  fi
  echo "=== change filter: classifying against $BASE"
  # No `local`/subshell tricks: read the KEY=value lines straight into the
  # variables above. If the script itself dies, the defaults stand and the
  # run is a full one — fail closed, same as hosted.
  while IFS='=' read -r k v; do
    case "$k" in
      TIER) TIER="$v" ;;
      CARGO_SCOPE) SCOPE="$v" ;;
      RUN_EDITOR_CORE) RUN_EDITOR_CORE="$v" ;;
      RUN_STL) RUN_STL="$v" ;;
      RUN_STEP_EXPORT) RUN_STEP_EXPORT="$v" ;;
      RUN_INTERVAL_BACKEND) RUN_INTERVAL_BACKEND="$v" ;;
      RUN_K_LINT) RUN_K_LINT="$v" ;;
    esac
  done < <(scripts/ci-filter.py --base "$BASE")
fi
echo "=== change filter: tier=$TIER scope='$SCOPE' (--full forces tier 'all')"
if [ "$TIER" = docs ]; then
  echo "=== documentation-only change set: nothing to build."
  echo "=== (hosted CI gates such a PR on the 'docs-only ok' marker job.)"
  echo "=== re-run with --full to force the whole matrix anyway."
  exit 0
fi

declare -a NAMES RESULTS
run_row() {
  local name="$1"; shift
  echo
  echo "=== [$name] $*"
  local t0=$SECONDS
  if "$@"; then RESULTS+=("PASS $((SECONDS - t0))s"); else RESULTS+=("FAIL $((SECONDS - t0))s"); fi
  NAMES+=("$name")
}
# A row whose root package is outside the closure: recorded, not run.
run_row_if() {
  local cond="$1" name="$2"; shift 2
  if [ "$cond" = true ]; then
    run_row "$name" "$@"
  else
    echo; echo "=== [$name] SKIPPED (not in the change closure)"
    NAMES+=("$name"); RESULTS+=("SKIP  0s")
  fi
}

# --- discipline (evaluation-code): the three tripwire greps, verbatim ---
discipline() {
  local rc=0
  if grep -rnE '\bReal\s*\+' crates/*/src; then
    echo "ERROR: found 'Real +' bound(s) above — evaluation-code discipline forbids extra bounds on scalar type parameters"
    rc=1
  fi
  # Compound Bounds allowlist (ratified 2026-07-29; geom-core real.rs
  # Bounds scope rule) — mirror of the hosted step.
  local bhits
  bhits=$(grep -rnE '\+\s*(geom_core::)?Bounds\b' crates/*/src \
    | grep -vE ':[0-9]+:\s*(//|///|//!)' \
    | cut -d: -f1 | sort -u \
    | grep -vE '^crates/topo/src/boolean/(boxes|mod|ops|reduce|rest)\.rs$' \
    | grep -vE '^crates/editor-core/src/eval/(mod|wire)\.rs$' \
    | grep -vE '^crates/profile/src/sugar\.rs$' || true)
  if [ -n "$bhits" ]; then
    echo "$bhits"
    echo "ERROR: compound Bounds bound outside the ratified seams — see geom-core/src/real.rs (Bounds scope rule)"
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

# $SCOPE is the filter's package scope: `--workspace` in tier `all`, an
# explicit `-p <closure>` list in tier `closure`. Unquoted on purpose —
# it must word-split into cargo arguments.
# shellcheck disable=SC2086
test_eps() { CAD_TOLERANCE_EPS="$1" cargo test $SCOPE; }
# shellcheck disable=SC2086
interval_eps() { CAD_TOLERANCE_EPS=1e-6 cargo test $SCOPE --features interval; }

# M4 PR 6 spec D6: the three persistence obligations as NAMED rows
# (also covered by the workspace rows; named = attributable).
# ε battery {1e-6, 1e-12} — see the run_row block below.
persist_roundtrip() {
  local e
  for e in 1e-6 1e-12; do
    CAD_TOLERANCE_EPS="$e" cargo test -p editor-core --test m4_pr6_roundtrip --test m4_pr6_floats --test m4_pr6_golden || return 1
  done
}
persist_eps_diff() { cargo test -p editor-core --test m4_pr6_eps_diff; }
persist_refusal() { cargo test -p editor-core --test m4_pr6_refusal --test m4_pr6_review_probes --test profile_desc_key; }
persist_interval() { cargo test -p editor-core --features interval --test m4_pr6_roundtrip_interval; }

# M4 PR 8a spec D1: the Band 4 corpus as NAMED rows (also covered by
# the workspace rows; named = attributable).
corpus_eps() {
  local e
  for e in 1e-6 1e-12; do
    CAD_TOLERANCE_EPS="$e" cargo test -p editor-core --test m4_pr8_corpus -- --nocapture || return 1
  done
}
corpus_interval() { cargo test -p editor-core --features interval --test m4_pr8_corpus_interval; }

# M4 PR 8a spec D2 (F8): rebuild-latency REPORTING — prints the
# per-document table and diffs the committed baseline. NOT A GATE on any
# timing number (the only assertions are the counted-reuse ones).
# Refresh the baseline with CAD_LATENCY_BASELINE_REFRESH=1.
rebuild_latency() { cargo test -p editor-core --test m4_pr8_latency -- --nocapture; }

# M5 PR 1 (review NOTE-1): the interval backend crate's OWN tripwire, in
# its own workspace, on its DEFAULT feature set — which reaches neither
# inari nor a C toolchain, so it runs in seconds. This is the row that
# catches a dropped outward round; the kernel's lane-agreement tests
# provably cannot (both lanes share the round-to-nearest chain). The full
# differential lane (certify.rs) is behind --features oracle-inari and
# stays a by-hand gate. Hosted mirror: ci.yml's `interval-backend` job.
interval_backend() {
  (cd interval-transcendentals \
    && cargo fmt --check \
    && cargo clippy --all-targets -- -D warnings \
    && cargo test) || return 1
  if (cd interval-transcendentals && cargo tree | grep -iE 'inari|gmp-mpfr-sys|rug'); then
    echo "ERROR: the interval backend's default feature set reaches the gmp stack"
    return 1
  fi
}

# Demos hygiene (M4 PR 8b pickup): demos/tour is workspace-excluded, so
# the workspace fmt/clippy rows above never see it — fmt drift and
# clippy errors accumulated invisibly until 8b. This row keeps them from
# silently returning. Hosted mirror: the ci.yml `k-lint` job runs the
# same two commands before its probe sweep (the tour must build there
# anyway — the demo scenes are half the lint's subject matter).
demos_hygiene() {
  (cd demos/tour && cargo fmt --check && cargo clippy --all-targets -- -D warnings)
}

# M4 PR 8b spec D3: the large-K fragility lint (mirrors ci.yml's
# `k-lint` job). Two rows: the tool's own hygiene + tests (the #99
# litmus MUST fire — that part gates), then the fresh probe sweep +
# the ADVISORY lint (prints flags, exits 0 on findings; fails only on
# harness breakage). Thresholds + provenance: tools/k-lint/src/lib.rs,
# docs/K-REPORT.md M4 addendum.
klint_tool() {
  (cd tools/k-lint && cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test)
}
klint_advisory() {
  scripts/k_probe_sweep.sh target/k-fresh || return 1
  (cd tools/k-lint && cargo run -- \
    ../../target/k-fresh/m4-eps-1e-6.csv \
    ../../target/k-fresh/m4-eps-1e-9.csv \
    ../../target/k-fresh/m4-eps-1e-12.csv)
}

# Rows always run (discipline greps are cheap; rustfmt is --all by design
# and cheap; the cargo rows are already package-scoped by $SCOPE).
# shellcheck disable=SC2086
run_row "discipline (evaluation-code)" discipline
run_row "rustfmt"                      cargo fmt --all --check
run_row "clippy"                       cargo clippy $SCOPE --all-targets -- -D warnings
run_row "test"                         cargo test $SCOPE
# ε battery {1e-6, 1e-12} (Evan's ruling, 2026-07-30). The two rows
# straddle the compiled default three orders either side; the default
# itself — DEFAULT_EPS = 1e-9, geom-core/src/tolerance.rs — is what the
# unparameterized `test` row above runs, so the retired `eps = 1e-9` row
# was a re-run of that row rather than a third band. Mirror of ci.yml's
# `multi-eps` matrix; keep the two in sync.
run_row "test (eps = 1e-6)"            test_eps 1e-6
run_row "test (eps = 1e-12)"           test_eps 1e-12
run_row "clippy (interval)"            cargo clippy $SCOPE --all-targets --features interval -- -D warnings
run_row "test (interval)"              cargo test $SCOPE --features interval
run_row "test (interval, eps = 1e-6)"  interval_eps
# Root package editor-core (persistence D6.*, band 4 corpus D1, latency D2).
run_row_if "$RUN_EDITOR_CORE" "persist save/load/replay (D6.1)" persist_roundtrip
run_row_if "$RUN_EDITOR_CORE" "persist eps-diff golden (D6.2)"  persist_eps_diff
run_row_if "$RUN_EDITOR_CORE" "persist refusal (D6.3)"          persist_refusal
run_row_if "$RUN_EDITOR_CORE" "persist roundtrip (interval)"    persist_interval
run_row_if "$RUN_EDITOR_CORE" "band 4 corpus (2 eps rows)"      corpus_eps
run_row_if "$RUN_EDITOR_CORE" "band 4 corpus (interval)"        corpus_interval
run_row_if "$RUN_EDITOR_CORE" "rebuild latency (reporting)"     rebuild_latency
# interval-transcendentals/ is its own workspace, so tier `closure` can
# never contain a change to it — this row belongs to tier `all` only.
run_row_if "$RUN_INTERVAL_BACKEND" "interval backend crate" interval_backend
# demos/tour and tools/k-lint are excluded workspaces that path-depend on
# nine members between them, and the probe sweep records margins from every
# kernel crate — no minimal root set, so these run whenever anything builds.
run_row_if "$RUN_K_LINT" "demos tour (fmt + clippy)"       demos_hygiene
run_row_if "$RUN_K_LINT" "k-lint tool (fmt+clippy+litmus)" klint_tool
run_row_if "$RUN_K_LINT" "k-lint sweep + advisory lint"    klint_advisory
# Root package stl: the acceptance example and its whole (dev-)dependency
# chain profile -> sweep -> topo -> mesh live under it.
run_row_if "$RUN_STL" "watertight (admesh)"          watertight
# Root package step-export: no cargo build — FreeCAD over the committed
# fixtures, which are byte-golden against that crate's writer.
run_row_if "$RUN_STEP_EXPORT" "step import (freecad)" step_import

echo
echo "=== ci-local summary ==="
fail=0
for i in "${!NAMES[@]}"; do
  printf '%-32s %s\n' "${NAMES[$i]}" "${RESULTS[$i]}"
  [[ "${RESULTS[$i]}" == FAIL* ]] && fail=1
done
exit $fail

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
# from source — 0.98.4+) and cargo-nextest (test rows; pinned 0.9.140 to
# match hosted — `cargo install cargo-nextest --locked --version 0.9.140`
# or the prebuilt from https://get.nexte.st/0.9.140/linux). Nothing here
# needs a C toolchain: the `interval` feature's backend is the in-repo,
# pure-Rust `interval-transcendentals`.
#
# BUILD ONCE PER COMPILE MODE (2026-08-03): hosted CI now compiles the
# test binaries once per feature graph (`build` / `build-interval`, via
# `cargo nextest archive`) and fans the eps rows out over the archived
# binaries — CAD_TOLERANCE_EPS is runtime env, so the eps rows were
# recompiling bit-identical binaries. LOCALLY that build-once property is
# automatic: every row shares the one target/ dir, so the first test row
# compiles and the rest reuse. The mirror below is therefore about the
# FILTER and ROW SEMANTICS staying identical to hosted — same runner
# (nextest, process-per-test), same row set (incl. the explicit doc-test
# rows: nextest does not run doc-tests), same eps env — NOT about
# archives/artifacts, which are hosted plumbing with no local analogue.
# Hosted additionally splits each eps row into two --partition count
# shards for wall-clock fan-out; the shards' union is exactly the row,
# so the unsharded rows here gate the same test set.
#
# NOT MIRRORED, deliberately (2026-08-04): ci.yml's two build jobs set
# RUSTFLAGS=-C link-arg=-fuse-ld=mold and CARGO_PROFILE_{DEV,TEST}_DEBUG=
# line-tables-only. Those are hosted-runner throughput knobs — they cut
# the cost of the 261 test-binary links, they do not change which targets
# compile or which tests run, so the local gate proves the same thing
# without them. Mirroring them here would be actively wrong: it would
# change local developer defaults (a system `mold`, and thinner local
# debuginfo than a debugging session wants), and gate.sh unsets RUSTFLAGS
# on purpose to keep its warm target/ from re-fingerprinting. See the
# LINK/DEBUGINFO note at the top of ci.yml for the measurement.
#
# Merge-gate runs go through scripts/gate.sh (serialized, warm runner —
# see its header for the caching guidance and RUSTFLAGS hazard).
set -u
cd "$(dirname "$0")/.."

# Original args, preserved for the build-slot re-exec below (the parse
# loop consumes "$@").
ORIG_ARGS=("$@")

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
RUN_PNCAD_PY=true
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
      RUN_PNCAD_PY) RUN_PNCAD_PY="$v" ;;
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

# Anything past here builds and runs tests: take the machine-wide build
# slots EXCLUSIVELY (a full battery next to another cargo lane is the
# documented OOM-kill shape — bare "Terminated" rows). Placed after the
# docs early-exit so docs-only runs never wait on locks; the re-exec'd
# script re-runs the (cheap) filter, then passes this guard.
if [ -z "${BUILD_SLOT_HELD:-}" ]; then
  exec scripts/with-build-slot.sh -x -- scripts/ci-local.sh "${ORIG_ARGS[@]}"
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
  # Bounds scope rule) — mirror of the hosted step. topo/props.rs is
  # the M5 PR 11 certified-quadrature seam (Evan's lane-split ruling);
  # sweep/src/fillet/{battery,build}.rs is the M5 PR 12 fillet-battery
  # seam, ratified under that same ruling because its margins are
  # certified metric quantities (sup-κ hulls, setback bounds) and NO
  # dual-scalar path can reach it — `Dual` has no `Bounds` impl, and
  # the only caller sits under editor-core's already-Bounds-bounded
  # `evaluate<T>`. A `PropsQuadLane`-style static split would have had
  # an empty refusing side.
  #
  # geom-brep/src/{ssi.rs,ssi/certify.rs,pcurve_cache.rs} is the M6-2
  # SSI generic-T lift: the rung-3 certificate simultaneously DECIDES
  # (its `ssi_*` funnel margins) and reads brackets into the C9 ring
  # (its hull/tube limbs ARE ring enclosures), so `Decide + Bounds` is
  # its honest signature — the same class as the quadrature seam. The
  # split is NOT empty here and is written: `PcurveFittedLane` has
  # certified impls for f64/Probe/Interval and a refusing one for
  # `Dual`. `ssi/enclose.rs` is deliberately NOT listed — it needs no
  # decision, so it takes the sole-bound `T: Bounds` the rule allows
  # everywhere.
  #
  # geom-brep/src/edge_nurbs.rs is M7-8's plane × NURBS edge lane, the
  # narrowest possible extension of that same seam: it DELEGATES to the
  # already-listed `certify_rung3` door with a declared carrier instead
  # of a marched one, so it inherits the door's signature rather than
  # widening anything. Its split is written in the same shape —
  # `EdgeNurbsLane` has certified impls for f64/Probe/Interval and a
  # refusing one for `Dual` — and it is what keeps `Bounds` out of
  # `topo`'s signatures.
  #
  # profile/src/path/arc_fillet.rs is the LIB-G2 PATHS arc-carrier
  # fillet boundary (ruling LB3, 2026-08-08). The algebra forbids
  # authoring a fillet's corner, so it DERIVES 0/1/2 corners from the
  # two carriers and the S8 choice is over (corner, candidate) pairs —
  # it therefore DECIDES (the carrier-meet and angular advance/reach
  # gates) and reads the selection channel in one function, which is
  # `Decide + Bounds` honestly. It carries sugar.rs's ratified
  # justification verbatim: the pick is a plain deterministic selection
  # rule on the f64 diagnostic channel, a representation-level choice
  # between already-classified constructions, never a re-decision of
  # geometry. The compound bound is confined to this ONE file so
  # `path.rs` itself stays bracket-free; `fillet_select.rs`, which
  # states the ladder, is deliberately NOT listed — sole-bound
  # `T: Bounds`, which the rule allows everywhere.
  local bhits
  bhits=$(grep -rnE '\+\s*(geom_core::)?Bounds\b' crates/*/src \
    | grep -vE ':[0-9]+:\s*(//|///|//!)' \
    | cut -d: -f1 | sort -u \
    | grep -vE '^crates/topo/src/boolean/(boxes|mod|ops|reduce|rest)\.rs$' \
    | grep -vE '^crates/topo/src/props\.rs$' \
    | grep -vE '^crates/editor-core/src/eval/(mod|wire)\.rs$' \
    | grep -vE '^crates/profile/src/sugar\.rs$' \
    | grep -vE '^crates/profile/src/path/arc_fillet\.rs$' \
    | grep -vE '^crates/sweep/src/fillet/(battery|build|surgery)\.rs$' \
    | grep -vE '^crates/geom-brep/src/(pcurve_cache|ssi|ssi/certify|edge_nurbs)\.rs$' || true)
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

# Render provenance (#221 follow-up): every committed per-scene PNG in
# demos/renders{,-freecad}/ must carry FreeCAD's signature tEXt chunks,
# so a matplotlib fallback frame in a committed path fails loud instead
# of riding into a montage cell; demos/renders-wild/ (the wild-corpus
# lane, FreeCAD-free by scope) runs inverted per-lane rules — its
# cells must be matplotlib-drawn AND carry the wild lane's own Author
# stamp. Stdlib-only python3 (no venv, no
# FreeCAD, milliseconds) — hence an always-run row, not a filtered one:
# a guard that a tier selection can skip is not a guard. Runs its own
# self-test first (the guard must be shown to fire). Hosted mirror: the
# `k-lint` job's "demos render provenance" step.
render_provenance() {
  python3 demos/check_render_provenance.py --selftest && \
    python3 demos/check_render_provenance.py
}

# The UV lane's composer (demos/render-uv.sh) has no provenance problem
# — the kernel is the only thing that could have drawn an SVG cell — but
# it does make two claims about what it leaves OFF the sheet (one
# representative per (body, chart); planar charts dropped as a class),
# and a silent drop is precisely what this lane exists to prevent. Its
# self-test pins those, plus the fail-loud on a cell that does not match
# the emitter's root-tag contract. Stdlib-only python3, milliseconds.
uv_composer_selftest() {
  python3 demos/compose_uv_montage.py --selftest
}

# Drift gate for the committed UV sheet: regenerate it and diff. The two
# PNG lanes cannot be gated (they need FreeCAD), so this is the only
# render lane CI can reproduce — and an ungated committed artifact rots.
# The tour is ~3s once built and the sheet is text, so a firing diff is
# readable. Hosted mirror: the `k-lint` job's "uv sheet drift (demos)".
uv_sheet_drift() {
  (cd demos/tour && cargo run --release -- ../out) >/dev/null && \
    demos/render-uv.sh >/dev/null && \
    git diff --exit-code --stat HEAD -- demos/renders-uv/
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

# Mirror of hosted's `python-suite` job (LIB PY-CI). Hosted runs the
# wheel path — maturin build, venv, pip install, unittest discover.
# This box has no pip/ensurepip in the system Python (measured, U9S
# report), so the local row is the staged-cdylib fallback
# run-python-tests.sh exists for: same cargo-built extension module,
# same interpreter contract, same unittest discovery over the same
# tests/ directory — only the install vehicle degrades. The script
# takes the build slot itself; nested under ci-local's exclusive hold
# that acquisition is a no-op (BUILD_SLOT_HELD).
python_suite() {
  crates/pncad-py/run-python-tests.sh
}

# $SCOPE is the filter's package scope: `--workspace` in tier `all`, an
# explicit `-p <closure>` list in tier `closure`. Unquoted on purpose —
# it must word-split into cargo arguments.
#
# The test rows run under nextest to match hosted exactly (process-per-
# test semantics can differ from `cargo test`'s in-process threads); the
# doc-test rows stay on `cargo test --doc` because nextest never runs
# doc-tests. Fail loud, not fall back: a cargo-test fallback would gate
# on different semantics than hosted.
nextest_check() {
  command -v cargo-nextest >/dev/null && return 0
  echo "ERROR: cargo-nextest not installed (hosted CI pins 0.9.140):"
  echo "  cargo install cargo-nextest --locked --version 0.9.140"
  return 1
}
# shellcheck disable=SC2086
test_default() { nextest_check && cargo nextest run $SCOPE; }
# shellcheck disable=SC2086
test_eps() { nextest_check && CAD_TOLERANCE_EPS="$1" cargo nextest run $SCOPE; }
# shellcheck disable=SC2086
doc_tests() { cargo test --doc $SCOPE; }
# shellcheck disable=SC2086
interval_tests() { nextest_check && cargo nextest run $SCOPE --features interval; }
# shellcheck disable=SC2086
interval_eps() { nextest_check && CAD_TOLERANCE_EPS=1e-6 cargo nextest run $SCOPE --features interval; }
# shellcheck disable=SC2086
interval_doc_tests() { cargo test --doc $SCOPE --features interval; }

# M4 PR 6 spec D6: the three persistence obligations as NAMED rows
# (also covered by the workspace rows; named = attributable).
# ε battery {1e-6, 1e-12} — see the run_row block below.
persist_roundtrip() {
  local e
  for e in 1e-6 1e-12; do
    CAD_TOLERANCE_EPS="$e" cargo test -p editor-core --test all -- m4_pr6_roundtrip:: m4_pr6_floats:: m4_pr6_golden:: || return 1
  done
}
persist_eps_diff() { cargo test -p editor-core --test all -- m4_pr6_eps_diff::; }
persist_refusal() { cargo test -p editor-core --test all -- m4_pr6_refusal:: m4_pr6_review_probes:: profile_desc_key::; }
# Mirrors the named step in hosted's test-interval job (which runs it
# out of the interval archive by binary_id).
persist_interval() { nextest_check && cargo nextest run -p editor-core --features interval -E 'binary_id(editor-core::all) & test(/^m4_pr6_roundtrip_interval::/)'; }

# M4 PR 8a spec D1: the Band 4 corpus as NAMED rows (also covered by
# the workspace rows; named = attributable).
corpus_eps() {
  local e
  for e in 1e-6 1e-12; do
    CAD_TOLERANCE_EPS="$e" cargo test -p editor-core --test all -- --nocapture m4_pr8_corpus:: || return 1
  done
}
corpus_interval() { nextest_check && cargo nextest run -p editor-core --features interval -E 'binary_id(editor-core::all) & test(/^m4_pr8_corpus_interval::/)'; }

# M4 PR 8a spec D2 (F8): rebuild-latency REPORTING — prints the
# per-document table and diffs the committed baseline. NOT A GATE on any
# timing number (the only assertions are the counted-reuse ones).
# Refresh the baseline with CAD_LATENCY_BASELINE_REFRESH=1.
rebuild_latency() { cargo test -p editor-core --test all -- --nocapture m4_pr8_latency::; }

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
  (cd demos/tour && cargo fmt --check && cargo clippy --all-targets -- -D warnings) && \
    (cd demos/wild && cargo fmt --check && cargo clippy --all-targets -- -D warnings)
}

# Spec D3: the large-K fragility lint (mirrors ci.yml's `k-lint` job —
# hosted and local must not drift, which is this script's whole point).
# Two rows: the tool's own hygiene + tests (the #99 litmus MUST fire),
# then the fresh probe sweep + the LINT GATE — a flagged margin fails
# the row (exit 2, with the interpretation discipline printed);
# harness breakage fails it in its own voice (exit 1).
#
# On a failure, read the tool's message before touching geometry: a
# fired lint is evidence that the margin DISTRIBUTION moved, and the
# recourse is re-derivation or a recorded demotion, never a geometry
# nudge. Thresholds + provenance: tools/k-lint/src/lib.rs,
# docs/K-REPORT.md ("M7 addendum (2026-08-07): the large-K lint's
# floor refresh").
klint_tool() {
  (cd tools/k-lint && cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test)
}
klint_gate() {
  scripts/k_probe_sweep.sh target/k-fresh || return 1
  (cd tools/k-lint && cargo run -- \
    ../../target/k-fresh/k-eps-1e-6.csv \
    ../../target/k-fresh/k-eps-1e-9.csv \
    ../../target/k-fresh/k-eps-1e-12.csv)
}

# Rows always run (discipline greps are cheap; rustfmt is --all by design
# and cheap; the cargo rows are already package-scoped by $SCOPE).
# shellcheck disable=SC2086
run_row "discipline (evaluation-code)" discipline
run_row "render provenance (demos)"    render_provenance
run_row "uv composer selftest (demos)" uv_composer_selftest
run_row "rustfmt"                      cargo fmt --all --check
run_row "clippy"                       cargo clippy $SCOPE --all-targets -- -D warnings
# ε battery {default, 1e-6, 1e-12} (Evan's ruling, 2026-07-30): the two
# env rows straddle the compiled default — DEFAULT_EPS = 1e-9, geom-core/
# src/tolerance.rs — three orders either side. Mirror of ci.yml's `test`
# matrix over the default archive; the first row compiles, the eps rows
# reuse target/ (build-once is automatic locally — see the header).
run_row "test (eps = default)"         test_default
run_row "test (eps = 1e-6)"            test_eps 1e-6
run_row "test (eps = 1e-12)"           test_eps 1e-12
# Doc-tests: nextest never runs them; hosted keeps them in the build
# jobs, this script as their own rows.
run_row "doc-tests"                    doc_tests
run_row "clippy (interval)"            cargo clippy $SCOPE --all-targets --features interval -- -D warnings
run_row "test (interval)"              interval_tests
run_row "test (interval, eps = 1e-6)"  interval_eps
run_row "doc-tests (interval)"         interval_doc_tests
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
run_row_if "$RUN_K_LINT" "uv sheet drift (demos)"          uv_sheet_drift
run_row_if "$RUN_K_LINT" "k-lint tool (fmt+clippy+litmus)" klint_tool
run_row_if "$RUN_K_LINT" "k-lint sweep + gate"             klint_gate
# Root package stl: the acceptance example and its whole (dev-)dependency
# chain profile -> sweep -> topo -> mesh live under it.
run_row_if "$RUN_STL" "watertight (admesh)"          watertight
# Root package step-export: no cargo build — FreeCAD over the committed
# fixtures, which are byte-golden against that crate's writer.
run_row_if "$RUN_STEP_EXPORT" "step import (freecad)" step_import
# Root package pncad-py: the wheel's build graph is the whole façade
# stack, so this fires exactly when something the suite compiles moved.
run_row_if "$RUN_PNCAD_PY" "python suite (staged cdylib)" python_suite

echo
echo "=== ci-local summary ==="
fail=0
for i in "${!NAMES[@]}"; do
  printf '%-32s %s\n' "${NAMES[$i]}" "${RESULTS[$i]}"
  [[ "${RESULTS[$i]}" == FAIL* ]] && fail=1
done
exit $fail

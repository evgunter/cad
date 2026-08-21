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
# ALSO NOT MIRRORED (2026-08-12): those jobs now set
# CARGO_PROFILE_{DEV,TEST}_OPT_LEVEL=2. This one deserves a sharper note
# than the knobs above, because opt-level is the one setting here that
# COULD in principle change a test outcome rather than just its cost.
# Measured both ways on the same tree the day it landed — opt-0 2791/2791
# and 3001/3001 (interval) green, opt-2 identical counts, all green — so
# the two configurations agree today, and this gate keeps proving the
# cheap-to-compile one. debug-assertions and overflow-checks are ON at
# both levels (cargo defaults them for dev/test), so nothing about
# fail-loud changes. To reproduce the hosted configuration locally, use
# local-scripts/test-fast.sh, which sets exactly these two variables.
#
# Merge-gate runs go through local-scripts/gate.sh (serialized, warm runner —
# see its header for the caching guidance and RUSTFLAGS hazard).
set -u
cd "$(dirname "$0")/.."

# HOSTED CI IS THE GATE. This whole-matrix entry point refuses without an
# explicit certification that hosted CI is unavailable. See
# local-scripts/hosted-ci-guard.sh for the reasoning and the override.
# shellcheck source=local-scripts/hosted-ci-guard.sh
. "$(dirname "$0")/hosted-ci-guard.sh"
require_hosted_ci "local-scripts/ci-local.sh"

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
RUN_TOPO_RELEASE=true
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
      RUN_TOPO_RELEASE) RUN_TOPO_RELEASE="$v" ;;
    esac
  done < <(scripts/ci-filter.py --base "$BASE")
fi
echo "=== change filter: tier=$TIER scope='$SCOPE' (--full forces tier 'all')"

# --- tier-blind rows: A CHECK MUST BE SITED WHERE IT CAN FIRE ON ITS OWN
# INPUTS (Evan, 2026-08-20, on S61). These read prose, documentation and this
# file — inputs whose change sets classify TIER=docs, which is exactly what the
# early exit below returns on. Placed ABOVE it, because a check the tier
# selection can skip is not a check, and because siting them below would
# reproduce in the local half the defect the hosted half was fixed for.
# No cargo, no build slot: greps over three files, milliseconds.
# `gate-roster.sh` runs again inside the discipline loop on a building change
# set; running a grep twice is cheaper than a second hand-written roster.
# HOSTED MIRROR: mirror / gate roster parity (both halves run every gate)
# HOSTED MIRROR: mirror / probe type-check loop citations
# HOSTED MIRROR: mirror / CI half parity (both halves name the same checks)
tier_blind_rows() {
  local rc=0
  scripts/gates/gate-roster.sh --selftest || rc=1
  scripts/gates/gate-roster.sh || rc=1
  scripts/gates/probe-suite-census.sh --citations || rc=1
  python3 scripts/check-ci-mirror-parity.py --selftest || rc=1
  python3 scripts/check-ci-mirror-parity.py || rc=1
  return $rc
}
echo
echo "=== [tier-blind rows] (run on every tier, including docs)"
if tier_blind_rows; then TIER_BLIND_RC=0; else TIER_BLIND_RC=1; fi

if [ "$TIER" = docs ]; then
  echo "=== documentation-only change set: nothing to build."
  echo "=== (hosted CI gates such a PR on the 'docs-only ok' marker job and"
  echo "===  on the 'mirror' job, which carries no if: and runs on every tier.)"
  echo "=== re-run with --full to force the whole matrix anyway."
  [ "$TIER_BLIND_RC" -eq 0 ] || echo "=== TIER-BLIND ROWS FAILED"
  exit "$TIER_BLIND_RC"
fi

# Anything past here builds and runs tests: take the machine-wide build
# slots EXCLUSIVELY (a full battery next to another cargo lane is the
# documented OOM-kill shape — bare "Terminated" rows). Placed after the
# docs early-exit so docs-only runs never wait on locks; the re-exec'd
# script re-runs the (cheap) filter, then passes this guard.
if [ -z "${BUILD_SLOT_HELD:-}" ]; then
  exec local-scripts/with-build-slot.sh -x -- local-scripts/ci-local.sh "${ORIG_ARGS[@]}"
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

# --- discipline (evaluation-code): the mirrored tripwire gates ---
# EVERY gate has exactly ONE home, under scripts/gates/, and this
# function runs the WHOLE DIRECTORY — so there is no local roster to
# drift, and adding a gate here takes nothing but dropping the file in.
# ci.yml cannot do the same (the Actions UI reads failures by step name,
# so each gate needs its own named step), which leaves exactly one
# hand-written roster in the repo; `gate-roster.sh` is the gate that
# checks it against this directory. The ratified allowlist prose lives
# WITH each gate — read the script before touching a membership.
#
# `lib.sh` is sourced, not run, and is skipped BY NAME. It used to be
# skipped by `[ -x "$g" ] || continue`, which made the executable bit
# the registration mechanism: a gate landing mode 0644 was silently
# skipped here and equally silently skipped by `gate-roster.sh`, so it
# ran nowhere while being counted among "all N gates". A member of that
# directory that cannot be executed is a failure now, not a skip.
# Each gate runs its `--selftest` first, as the sibling python gate
# does — a guard that has never been shown to fire is not a guard.
discipline() {
  local rc=0 g
  for g in scripts/gates/*.sh; do
    [ "$(basename "$g")" = lib.sh ] && continue
    if [ ! -x "$g" ]; then
      echo "ERROR: $g is not executable — a gate in scripts/gates/ that no half can run is registered nowhere" >&2
      rc=1
      continue
    fi
    "$g" --selftest || rc=1
    "$g" || rc=1
  done
  # The k-probe sweep's `run_dump` guards, proved against a stub cargo —
  # milliseconds, no build.
  # HOSTED MIRROR: discipline / k-probe sweep guard selftest (run_dump)
  scripts/rundump-guard-selftest.sh || rc=1
  # The `interval` feature must stay purely additive, calling the SAME
  # script the hosted step calls. This is what makes it sound for the
  # interval rows below to run only the tests that feature ADDS; read
  # the script's header before touching either half.
  # HOSTED MIRROR: discipline / interval-feature additivity (gates the interval run legs)
  if ! (python3 scripts/check-interval-cfg-additive.py --selftest \
        && python3 scripts/check-interval-cfg-additive.py); then
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
# self-test first (the guard must be shown to fire).
# HOSTED MIRROR: discipline / render provenance (demos)
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
# HOSTED MIRROR: discipline / uv composer selftest (demos)
uv_composer_selftest() {
  python3 demos/compose_uv_montage.py --selftest
}

# The one reader of demos/out/scenes.json, shared by both renderers,
# the montage composer and render.sh's scene lister. Its self-test
# pins the `view.up` convention against the two spellings it replaced,
# proves the display -> world direction really is the inverse of the
# world -> display one it derives from, and pins that a manifest
# missing `transparency` or `montage` REFUSES rather than defaulting.
# Stdlib-only python3, milliseconds.
# HOSTED MIRROR: discipline / scene manifest reader selftest (demos)
manifest_selftest() {
  python3 demos/manifest.py --selftest
}

# Drift gate for the committed UV sheet: regenerate it and diff. The two
# PNG lanes cannot be gated (they need FreeCAD), so this is the only
# render lane CI can reproduce — and an ungated committed artifact rots.
# The tour is ~3s once built and the sheet is text, so a firing diff is
# readable. NO HOSTED MIRROR ANY MORE (2026-08-17): the `k-lint` job's
# "uv sheet drift (demos)" row was retired when render.yml's uv lane
# started re-baselining itself, so hosted CI now COMMITS a drifting
# sheet and marks the run neutral instead of failing. This row stays a
# failing check because a developer box cannot re-baseline itself —
# locally, being told is the whole point.
#
# The `CAD_RENDER_LOCAL_OVERRIDE` sentence is set HERE, in the file, at
# the one step that renders: the entry points refuse without it
# (demos/hosted-render-guard.sh) and deliberately do not sniff for CI.
# This row is a sanctioned automated render — renderer-free, and
# `git diff --exit-code` un-does the question of drift by failing on it.
uv_sheet_drift() {
  (cd demos/tour && cargo run --release -- ../out) >/dev/null && \
    CAD_RENDER_LOCAL_OVERRIDE=i-accept-local-render-drift \
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

# Mirror of hosted's `release-corruption` job. The ONE row in either
# gate that compiles the release profile: two suites in crates/topo split
# their expectations on `cfg(debug_assertions)`, so the ordinary rows
# above can only ever run one half. Scoped to `-p topo --lib` — the
# compile is the whole cost (93 s on a cold hosted cache). The guards
# after the run are the point: a name filter that matches nothing exits
# 0, so an empty selection must fail rather than pass quietly.
topo_release() {
  local log rc passed
  log=$(mktemp) || return 1
  set -o pipefail
  cargo test --release -p topo --lib -- \
    review_m1_pr2::release_corruption \
    review_m1_pr4::kill_ops_survive_torn_bodies_without_panicking \
    2>&1 | tee "$log"
  rc=$?
  set +o pipefail
  if [ "$rc" -ne 0 ]; then rm -f "$log"; return "$rc"; fi
  passed=$(sed -n 's/^test result: ok\. \([0-9]\{1,\}\) passed.*/\1/p' "$log")
  echo "release-profile rows run: ${passed:-0}"
  rc=0
  [ "${passed:-0}" -gt 0 ] || { echo "ERROR: no tests matched the release-profile filters"; rc=1; }
  for t in 'garbage_in_garbage_out_release' 'large_torn_body_terminates_quickly' \
           'kill_ops_survive_torn_bodies_without_panicking'; do
    grep -q "$t \.\.\. ok" "$log" || { echo "ERROR: $t did not run"; rc=1; }
  done
  grep -q 'corrupt input (release profile)' crates/topo/src/review_m1_pr2/release_corruption.rs ||
    { echo "ERROR: release_corruption.rs no longer names this row"; rc=1; }
  grep -q 'corrupt input (release profile)' crates/topo/src/review_m1_pr4.rs ||
    { echo "ERROR: review_m1_pr4.rs no longer names this row"; rc=1; }
  rm -f "$log"
  return "$rc"
}

# Mirror of hosted's `python-suite` job (LIB PY-CI). Hosted runs the
# wheel path — maturin build, venv, pip install, unittest discover.
# The local row is the staged-cdylib fallback run-python-tests.sh
# exists for: same cargo-built extension module, same interpreter
# contract, same unittest discovery over the same tests/ directory —
# only the install vehicle degrades, which keeps the row runnable even
# on the most degraded box U9S measured (no pip, no ensurepip, no
# maturin). The script takes the build slot itself; nested under
# ci-local's exclusive hold that acquisition is a no-op
# (BUILD_SLOT_HELD).
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
# The interval rows run ONLY the tests the feature adds, exactly as
# hosted's `test-interval` legs do — same script, same set difference, so
# the two gates cannot drift. See that job's header in .github/workflows/
# ci.yml for the measurement (42% of hosted test time was re-execution)
# and for why it is sound (the feature is additive, and
# check-interval-cfg-additive.py above gates that it stays so).
INTERVAL_SEL="target/ci-local/interval-selection.txt"
# HOSTED MIRROR: test-interval / derive the interval-only test selection
# shellcheck disable=SC2086
interval_selection() {
  mkdir -p target/ci-local \
    && cargo nextest list $SCOPE --message-format json \
         > target/ci-local/nextest-list-default.json \
    && cargo nextest list $SCOPE --features interval --message-format json \
         > target/ci-local/nextest-list-interval.json \
    && scripts/interval-only-selection.py \
         target/ci-local/nextest-list-default.json \
         target/ci-local/nextest-list-interval.json > "$INTERVAL_SEL"
}
# `--no-tests` is decided by the SELECTION, mirroring hosted's identical
# conditional: nextest exits 4 on a zero-test run, which is the alarm we
# want when a real filter matches nothing, and exactly wrong for the
# `none()` the selection script emits for a scope carrying no
# interval-gated tests. Any other filter that selects nothing still
# fails the row.
# shellcheck disable=SC2086
interval_tests() {
  nextest_check && interval_selection || return 1
  local sel extra=""
  sel=$(cat "$INTERVAL_SEL")
  [ "$sel" = "none()" ] && extra="--no-tests=pass"
  cargo nextest run $SCOPE --features interval -E "$sel" $extra
}
# shellcheck disable=SC2086
interval_eps() {
  nextest_check && interval_selection || return 1
  local sel extra=""
  sel=$(cat "$INTERVAL_SEL")
  [ "$sel" = "none()" ] && extra="--no-tests=pass"
  CAD_TOLERANCE_EPS=1e-6 cargo nextest run $SCOPE --features interval \
    -E "$sel" $extra
}
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
# per-document table and diffs the newest entry in the timing history,
# docs/perf-data/rebuild-latency/. NOT A GATE on any timing number (the
# assertions are the counted-reuse ones plus the corpus manifest's
# nodes/cone pins — arithmetic, never wall clock).
#
# NOTHING TO REFRESH HERE ANYMORE (2026-08-17). The old
# CAD_LATENCY_BASELINE_REFRESH env var wrote a committed baseline from
# whatever box ran it, and that is precisely what rotted the numbers —
# three workstation refreshes disagreed by 90-98% with contention ruled
# out. Hosted CI is the canonical producer now (ci.yml's `rebuild
# latency (reporting)` job emits and commits); a local run READS the
# history and reports against it, and can no longer write to it. Your
# local milliseconds are not comparable with a runner's, which is the
# point — the `vs base` column here is a sanity check, not a datum.
# `--ignored`: the test is #[ignore]d so the eps matrix stops paying for
# it 5x (its two assertions are a strict subset of the corpus row's —
# see the rustdoc on rebuild_latency_table). THIS row is the one that
# runs it. Mirrors ci.yml's `rebuild latency (reporting)` job.
rebuild_latency() { cargo test -p editor-core --test all -- --ignored --nocapture m4_pr8_latency::; }

# M5 PR 1 (review NOTE-1): the interval backend crate's OWN tripwire, in
# its own workspace, on its DEFAULT feature set — which reaches neither
# inari nor a C toolchain, so it runs in seconds. It catches what the
# kernel's lane-agreement tests provably cannot (both lanes there share
# the round-to-nearest chain).
#
# WHAT IT CATCHES, by direction, because the two are not the same job. A
# pad WIDENED: every operation, via pad_contract.rs. A pad DROPPED:
# division, multiplication and sqrt, via review_fuzz_exact.rs's exact
# u128 rational arithmetic. NOT `+` and `-` (their pads are bounded from
# above here but a dropped one shows only against the oracle), and NOT
# the seven transcendentals (their truth needs a multi-precision
# reference). Hosted mirror: ci.yml's `interval-backend` job.
#
# THIS SCRIPT HAS NO MIRROR OF ci.yml's `oracle-certify` JOB, which is
# what covers the two directions above that this row cannot. So on the
# hosted pipeline a dropped transcendental pad is caught; under
# `gate.sh`, which documents itself as the merge gate when hosted Actions
# is unavailable, it is not. Nothing enforces ci.yml <-> ci-local.sh JOB
# parity (gate-roster.sh enforces gate-SCRIPT parity, which is a
# different set), so this comment is the only thing recording the gap.
# Filed as smell-scan S127 / §D row D71; adding the row here means a
# ~250s GMP+MPFR build in the local gate, which is why it is a decision
# rather than an oversight.
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

# Hosted mirror: ci.yml's "demos tour eps pin (#99)" row. Scoped to the
# integration test for the reason stated there — `--bin demo-tour`'s
# unit tests carry two rows that are red on main (issue #782).
demos_eps_pin() {
  (cd demos/tour && cargo test --release --test eps_regression)
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
# HOSTED MIRROR: k-lint / compile and list every probe-gated test target
# Both commands: the build covers what the feature instantiates
# (`--all-targets` reaches the lib's own `#[cfg(test)]` modules, which
# `--test all` never builds), and the listing asks which counted suites
# were actually built.
# The census half needs no mirror row of its own: it is `scripts/gates/`, so
# the discipline loop above runs it and its --selftest in DEFAULT mode. That
# argument is about the file and not the mode — the loop passes no flags, so
# `--citations` is not covered by it and has its own row above the docs exit.
# `check-ci-mirror-parity.py`'s claim 2 is what keeps that distinction checked.
#
# `crates` is BOUND FIRST, not expanded inside the `for` list: a command
# substitution in a `for` list is not subject to `set -e` and would leave
# this row green over zero crates if the census refused — the same
# silence one level up from the one the census exists to remove.
probe_targets() {
  local crates c listing
  crates=$(scripts/gates/probe-suite-census.sh --crates) || return 1
  [ -n "$crates" ] || { echo "ERROR: the probe census printed no crates"; return 1; }
  for c in $crates; do
    cargo test -p "$c" --features probe --all-targets --no-run || return 1
    # BOUND FIRST for the same reason `crates` is: this file runs without
    # `pipefail`, so `cargo … | gate` would report only the gate's status
    # and a failed build would reach it as an empty listing.
    listing=$(cargo test -p "$c" --features probe --test all -- --list) || return 1
    printf '%s\n' "$listing" \
      | scripts/gates/probe-suite-census.sh --check-listing "$c" || return 1
  done
}
klint_gate() {
  scripts/k_probe_sweep.sh target/k-fresh || return 1
  (cd tools/k-lint && cargo run -- \
    ../../target/k-fresh/k-eps-1e-6.csv \
    ../../target/k-fresh/k-eps-1e-9.csv \
    ../../target/k-fresh/k-eps-1e-12.csv)
}

# The tessellation-budget lint (issue #320; mirrors the two ci.yml
# `k-lint`-job rows of the same names). Two rows again: the tool's own
# hygiene + tests, then the fresh per-face sweep + the GATE.
#
# The gate compares against docs/tess-budget-data/, and it compares
# DIFFERENCES, not absolute slack — a scene whose mesh grew, a face
# whose sizing got wastefuller, or a scene that dropped out of the
# sweep. On a failure read the tool's message: coarsening a demo's
# delta to get the number down is the one forbidden move. What the
# absolute factors currently are, and why: docs/TESS-BUDGET.md.
tesslint_tool() {
  # `cargo doc` too, for the reason given at tessmeter_tool below: this
  # crate is workspace-excluded, so doc-gate.sh never sees the prose
  # that carries the gate's own rules and its report legend.
  (cd tools/tess-lint && cargo fmt --check && \
    cargo clippy --all-targets -- -D warnings && \
    RUSTDOCFLAGS="-D warnings -A rustdoc::private_intra_doc_links" \
      cargo doc --no-deps --document-private-items && \
    cargo test)
}
# The meter's CONSUMER half.
# HOSTED MIRROR: k-lint / tess-meter tool fmt + clippy + doc + tests
# The CSV schema, the counterfactual schedules and the split optimizer
# live here, outside the kernel, and so do their tests.
tessmeter_tool() {
  # `cargo doc` too: scripts/doc-gate.sh is `cargo doc --workspace` and
  # `tools/` is workspace-excluded, so this crate's prose has no other
  # gate (stated at doc-gate.sh; a row covering every excluded root is
  # owed).
  (cd tools/tess-meter && cargo fmt --check && \
    cargo clippy --all-targets -- -D warnings && \
    RUSTDOCFLAGS="-D warnings -A rustdoc::private_intra_doc_links" \
      cargo doc --no-deps --document-private-items && \
    cargo test)
}
# The meter's kernel half.
# HOSTED MIRROR: k-lint / mesh budget meter + certificate falsifier (feature = budget)
# `mesh::budget` is gated at its module boundary, so every row above
# runs the INERT half — the live half needs its own row or it rots.
#
# This row also carries
# `probe_review::z1_per_triangle_certificate_falsification`, which
# needs the deviation pass and so rides the `budget` build. M8-5's
# MIN-1 wanted the gate to run that falsifier unconditionally; this row
# is how that is still true, so it is unconditional here too.
budget_meter() {
  cargo clippy -p mesh --all-targets --features budget -- -D warnings && \
    cargo test -p mesh --features budget
}
# HOSTED MIRROR: k-lint / tessellation-budget sweep (every tour scene, per face)
# HOSTED MIRROR: k-lint / tessellation-budget lint (gate — a grown budget fails this row)
# `--sizing-only` mirrors ci.yml: the gate reads triangle counts and
# the sizing columns, never `worst_dev`, so the default sweep's
# per-triangle resampling (tens of millions of surface evaluations)
# would be paid for nothing. Re-cutting the baseline drops the flag.
tesslint_gate() {
  scripts/tess_budget_sweep.sh target/tess-budget-fresh.csv --sizing-only || return 1
  (cd tools/tess-lint && cargo run -- \
    ../../target/tess-budget-fresh.csv \
    --baseline ../../docs/tess-budget-data/tess-budget-baseline.csv)
}

# Rows always run (discipline greps are cheap; rustfmt is --all by design
# and cheap; the cargo rows are already package-scoped by $SCOPE).
# shellcheck disable=SC2086
run_row "discipline (evaluation-code)" discipline
run_row "render provenance (demos)"    render_provenance
run_row "uv composer selftest (demos)" uv_composer_selftest
run_row "scene manifest reader (demos)" manifest_selftest
run_row "rustfmt"                      cargo fmt --all --check
run_row "clippy"                       cargo clippy $SCOPE --all-targets -- -D warnings
# Rustdoc gate (#465): same script hosted calls, unscoped there and here
# — it is a workspace-wide ratchet over a fixed crate set, not a
# per-closure row. See scripts/doc-gate.sh for the flags and the list.
run_row "rustdoc (gate)"               scripts/doc-gate.sh
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
# demos/tour, tools/k-lint and tools/tess-lint are excluded workspaces
# that path-depend on nine members between them, and the probe sweep
# records margins from every kernel crate — no minimal root set, so
# these run whenever anything builds.
run_row_if "$RUN_K_LINT" "demos tour (fmt + clippy)"       demos_hygiene
run_row_if "$RUN_K_LINT" "demos tour eps pin (#99)"        demos_eps_pin
run_row_if "$RUN_K_LINT" "uv sheet drift (demos)"          uv_sheet_drift
run_row_if "$RUN_K_LINT" "k-lint tool (fmt+clippy+litmus)" klint_tool
run_row_if "$RUN_K_LINT" "probe test targets (type-check)"  probe_targets
run_row_if "$RUN_K_LINT" "k-lint sweep + gate"             klint_gate
run_row_if "$RUN_K_LINT" "tess-lint tool (fmt+clippy+doc+cli)" tesslint_tool
run_row_if "$RUN_K_LINT" "tess-meter tool (fmt+clippy+tests)" tessmeter_tool
run_row_if "$RUN_K_LINT" "mesh budget meter (feature)"     budget_meter
run_row_if "$RUN_K_LINT" "tess-budget sweep + gate"        tesslint_gate
# Root package stl: the acceptance example and its whole (dev-)dependency
# chain profile -> sweep -> topo -> mesh live under it.
run_row_if "$RUN_STL" "watertight (admesh)"          watertight
# Root package step-export: no cargo build — FreeCAD over the committed
# fixtures, which are byte-golden against that crate's writer.
run_row_if "$RUN_STEP_EXPORT" "step import (freecad)" step_import
# Root package topo: the release-profile row's suites live in that crate
# and it compiles `-p topo --lib`, so topo's own closure membership is
# the condition. Fires on 89 of the last 128 first-parent merges.
run_row_if "$RUN_TOPO_RELEASE" "corrupt input (release profile)" topo_release
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

#!/usr/bin/env bash
# scripts/gate.sh <sha-or-ref> — the serialized merge-gate runner.
#
# HOW THE MERGE GATE RUNS (while hosted Actions is down — GitHub
# free-plan minutes exhausted 2026-07-22; resets at Evan's
# billing-month rollover):
#
#   ./scripts/gate.sh <sha-or-ref>     # e.g. origin/main, a branch, a sha
#
# runs the full 11-row matrix (scripts/ci-local.sh, the ci.yml mirror)
# against the resolved commit in a persistent gate-runner checkout, and
# exits with ci-local.sh's status. Two properties make it fast and
# honest (caching investigation, PR #72):
#
#   * SERIALIZED: an flock on a global lock file queues concurrent gate
#     runs. The session-3 70-minute matrix was mostly CPU contention
#     from concurrent agent builds — identical rows measured 3-4x
#     faster uncontended. Uncontended warm matrix: ~3.7 min.
#   * WARM: the runner keeps one persistent target/ at a fixed path.
#     It is a STANDALONE CLONE (origin = GitHub), not a `git worktree`:
#     a worktree whose parent checkout is an ephemeral mngr worktree
#     dies with its parent; a clone survives.
#
# The gate is run on the candidate merge state: push your branch, merge
# (or preview-merge) as usual, and gate the resulting sha/ref here.
#
# CACHING GUIDANCE FOR AGENTS' OWN WORKTREES (not this runner):
#   * sccache v0.16.0 is at ~/.local/bin/sccache. Export
#     RUSTC_WRAPPER=~/.local/bin/sccache from the worktree's FIRST
#     build onward — flipping the wrapper mid-life re-fingerprints
#     everything. Cold worktree + warm sccache: ~8-9 min matrix.
#   * CARGO_INCREMENTAL=0 is fine for fix-pass worktrees (3G target
#     instead of 13-25G).
#
# HAZARDS (investigation cautions — do not "improve" these away):
#   * NEVER export RUSTFLAGS (or any rustc flag env). It silently
#     REPLACES .cargo/config.toml's `-C target-cpu=x86-64-v3`, which
#     the interval feature REQUIRES for inari's directed rounding.
#     This script defensively unsets inherited overrides below.
#   * No shared CARGO_TARGET_DIR across worktrees (fingerprint
#     ping-pong, no concurrency safety — rejected).
#   * Leave ~/.cache/gmp-mpfr-sys alone (machine-wide GMP/MPFR C
#     build cache — why interval libs build in 11s).
set -euo pipefail

[[ $# -eq 1 ]] || { echo "usage: $0 <sha-or-ref>" >&2; exit 2; }
REF="$1"

GATE_HOME="${CAD_GATE_HOME:-$HOME/.local/share/cad-gate}"
RUNNER="$GATE_HOME/repo"
LOCK="$GATE_HOME/lock"
ORIGIN_URL="git@github.com:evgunter/cad.git"
T0=$SECONDS

mkdir -p "$GATE_HOME"
exec 9>"$LOCK"
if ! flock --nonblock 9; then
  echo "[gate] another gate run holds the lock; waiting (serialized on purpose)..."
  flock 9
fi

# Bootstrap the persistent runner (standalone clone, warm target/ persists).
if [[ ! -d "$RUNNER/.git" ]]; then
  echo "[gate] bootstrapping gate-runner clone at $RUNNER (first run is cold)"
  git clone "$ORIGIN_URL" "$RUNNER"
fi

cd "$RUNNER"
git fetch origin --prune

# Resolve the requested ref to a commit: as given, else as origin/<ref>.
SHA=$(git rev-parse --verify --quiet "${REF}^{commit}" \
   || git rev-parse --verify --quiet "origin/${REF}^{commit}" \
   || { echo "[gate] ERROR: cannot resolve '$REF' to a commit" >&2; exit 2; })
echo "[gate] gating $REF -> $SHA"
git checkout --detach --quiet "$SHA"

# Fail loud on a dirty runner: stray files (e.g. untracked junk under
# crates/) would pollute the discipline greps and taint the gate.
if [[ -n "$(git status --porcelain)" ]]; then
  echo "[gate] ERROR: gate-runner checkout is dirty — inspect $RUNNER:" >&2
  git status --porcelain >&2
  exit 2
fi

# Keep the runner's build env constant: inherited overrides would either
# break interval rounding (RUSTFLAGS) or re-fingerprint the warm target
# (RUSTC_WRAPPER flip, CARGO_INCREMENTAL, CARGO_TARGET_DIR).
unset RUSTFLAGS CARGO_ENCODED_RUSTFLAGS CARGO_BUILD_RUSTFLAGS \
      RUSTC_WRAPPER CARGO_TARGET_DIR CARGO_INCREMENTAL CAD_TOLERANCE_EPS

rc=0
./scripts/ci-local.sh || rc=$?
echo
echo "[gate] sha $SHA — total wall time $((SECONDS - T0))s — exit $rc"
exit $rc

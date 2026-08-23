#!/usr/bin/env bash
# local-scripts/gate.sh <sha-or-ref> — the serialized merge-gate runner.
#
# STATUS (2026-07-25): FALLBACK ONLY. Hosted Actions is the merge
# gate (PR checks green = mergeable; same matrix, parallel, ~5-7 min
# on the PR's merge ref). Use this script only when Actions is
# unavailable (billing outage). The persistent runner's target/ is
# no longer kept warm — expect a cold rebuild on first fallback use.
# History (how the gate ran while Actions was down, 2026-07-22..25):
#
#   ./local-scripts/gate.sh <sha-or-ref>     # e.g. origin/main, a branch, a sha
#
# runs the full ci-local.sh matrix (the ci.yml mirror, however many rows)
# against the resolved commit in a persistent gate-runner checkout, and
# exits with ci-local.sh's status. Two properties make it fast and
# honest (caching investigation, PR #72):
#
#   * SERIALIZED: an flock on a global lock file queues concurrent gate
#     runs. (Additionally, ci-local.sh now self-acquires ALL machine
#     build slots — with-build-slot.sh — so a gate run also excludes
#     concurrent agent-lane builds, the source of the 3-4x contention
#     below.) The session-3 70-minute matrix was partly CPU
#     contention from concurrent agent builds (identical rows
#     measured 3-4x faster uncontended) and partly since-fixed
#     laptop settings (Evan, 2026-08-06) — timing numbers from
#     before that fix, including the ~3.7 min uncontended warm
#     matrix, are stale upper bounds.
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
#   * Avoid exporting RUSTFLAGS habitually: it silently REPLACES any
#     .cargo/config.toml rustflags. Since 2026-07-29 the repo sets NONE
#     (the x86-64-v3 floor was dropped after the M5 PR 1 backend swap
#     removed its correctness need — see .cargo/config.toml's history
#     note), so the old clobber hazard is gone; the unsets below remain
#     as cheap defense against inherited environment surprises.
#   * No shared CARGO_TARGET_DIR across worktrees (fingerprint
#     ping-pong, no concurrency safety — rejected).
#   * ~/.cache/gmp-mpfr-sys no longer matters for KERNEL builds (M5 PR 1
#     dropped the gmp stack from the interval feature); it still speeds
#     up interval-transcendentals' own inari dev-oracle lane.
set -euo pipefail

# HOSTED CI IS THE GATE; this runner is the fallback (see STATUS above).
# shellcheck source=local-scripts/hosted-ci-guard.sh
. "$(dirname "$0")/hosted-ci-guard.sh"
require_hosted_ci "local-scripts/gate.sh"

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

# Resolve the requested ref to a commit: origin/<ref> FIRST, then as
# given. Order matters: the runner's local branches (e.g. `main` from
# the bootstrap clone) never advance — only fetch + detach happen here —
# so resolving `main` as-given would silently gate a stale sha.
SHA=$(git rev-parse --verify --quiet "origin/${REF}^{commit}" \
   || git rev-parse --verify --quiet "${REF}^{commit}" \
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
./local-scripts/ci-local.sh || rc=$?
echo
echo "[gate] sha $SHA — total wall time $((SECONDS - T0))s — exit $rc"
exit $rc

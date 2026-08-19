#!/usr/bin/env bash
# lib.sh — shared plumbing for the mirrored discipline gates.
#
# THE INVARIANT: every gate in this directory has exactly ONE home, and
# both halves of CI call it — `.github/workflows/ci.yml`'s `discipline`
# job (one step per gate, keeping the step name the Actions UI shows)
# and `local-scripts/ci-local.sh`'s `discipline` row. A gate implemented
# twice drifts: the dual-maintained allowlists produced live drift in
# BOTH directions (a `separation.rs` entry hosted-only, a
# `test_support.rs` entry stale locally, a `chart_region.rs` entry
# hosted-only before that), and two gates existed hosted-only with no
# local mirror at all.
#
# WHY `scripts/` AND NOT `local-scripts/`: every workflow job runs
# `rm -rf local-scripts` right after checkout, so hosted CI cannot read
# anything there. `scripts/**` is also unscopable to a workspace member,
# so `scripts/ci-filter.py` classifies a change here as TIER=all — a
# gate edit re-runs everything, which is the conservative answer.
#
# Each gate script takes `--root DIR` (the tree to scan; default is this
# repo) and `--selftest` (plant a synthetic violation in a temp fixture
# tree and assert the gate fires, then exit).

# Repo root, derived from this file's location (scripts/gates/lib.sh).
GATE_REPO_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
GATE_ROOT=$GATE_REPO_ROOT
GATE_SELFTEST=false

gate_parse_args() {
  while [ $# -gt 0 ]; do
    case "$1" in
      --selftest) GATE_SELFTEST=true ;;
      --root) GATE_ROOT=$2; shift ;;
      *) printf 'usage: %s [--selftest] [--root DIR]\n' "$0" >&2; exit 2 ;;
    esac
    shift
  done
}

# One message text serves both halves: hosted CI wants the `::error::`
# annotation (it surfaces in the Actions UI against the failing step), a
# local run wants the plain form.
gate_error() {
  if [ -n "${GITHUB_ACTIONS:-}" ]; then
    printf '::error::%s\n' "$*"
  else
    printf 'ERROR: %s\n' "$*"
  fi
}

# gate_selftest WANT PLANTER [ARGS...] — PLANTER writes a synthetic
# violation into a temp tree (passed as its last argument); the gate
# then runs against that tree and must FAIL with a message containing
# WANT. The gate body is unparameterised apart from its root, so this
# exercises the real regex and the real allowlist.
gate_selftest() {
  local want=$1; shift
  local tmp out
  tmp=$(mktemp -d)
  "$@" "$tmp"
  if out=$(cd "$tmp" && gate 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: the gate PASSED on a planted violation\n%s\n' "$out" >&2
    exit 1
  fi
  rm -rf "$tmp"
  case "$out" in
    *"$want"*) ;;
    *) printf 'SELFTEST FAILED: the gate fired with an unexpected message:\n%s\n' "$out" >&2
       exit 1 ;;
  esac
  printf 'selftest OK (%s): the gate fires on a planted violation\n' "$(basename "$0")"
}

# The common tail: --selftest runs the self-test and exits; otherwise
# the gate runs against GATE_ROOT.
gate_main() {
  if [ "$GATE_SELFTEST" = true ]; then
    gate_selftest "$@"
    exit 0
  fi
  cd "$GATE_ROOT"
  gate
}

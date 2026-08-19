#!/usr/bin/env bash
# lib.sh — shared plumbing for the mirrored discipline gates.
#
# THE INVARIANT: every gate in this directory has exactly ONE home, and
# both halves of CI call it — `.github/workflows/ci.yml`'s `discipline`
# job (one step per gate, keeping the step name the Actions UI shows)
# and `local-scripts/ci-local.sh`'s `discipline` row. A gate implemented
# twice drifts: the dual-maintained allowlists produced live drift in
# BOTH directions (a `separation.rs` entry hosted-only, a
# `test_support.rs` paragraph stale locally, a `chart_region.rs` entry
# hosted-only before that), and two gates existed hosted-only with no
# local mirror at all.
#
# Sharing the BODIES is only half of that. The two halves still each
# name which gates to run, so `gate-roster.sh` closes the other half:
# it derives the roster from this directory and fails if either half
# runs a different set. Between them, neither the gate logic nor the
# gate list is maintained twice.
#
# WHY `scripts/` AND NOT `local-scripts/`: every workflow job runs
# `rm -rf local-scripts` right after checkout, so hosted CI cannot read
# anything there. `scripts/**` is also unscopable to a workspace member,
# so `scripts/ci-filter.py` classifies a change here as TIER=all — a
# gate edit re-runs everything, which is the conservative answer.
#
# Each gate script takes `--root DIR` (the tree to scan; default is this
# repo) and `--selftest` (assert the gate passes a clean fixture and
# fires on a planted one, then exit). Both halves run `--selftest`
# before the real pass, the way the sibling python gates do.

# Repo root, derived from this file's location (scripts/gates/lib.sh).
GATE_REPO_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
GATE_ROOT=$GATE_REPO_ROOT
GATE_SELFTEST=false
GATE_SCAN_FILES=0

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

gate_name() { basename "$0" .sh; }

# A GATE THAT SCANNED NOTHING IS NOT A PASS. `crates/*/src` is a glob:
# with no match bash hands the literal to grep, grep finds nothing, and
# the gate reports green for the wrong reason — green because it looked
# at an empty tree, not because the tree is clean. `--root` makes that
# reachable, so the scan target is proven before every scan.
gate_require_crate_sources() {
  local dirs=(crates/*/src)
  if [ ! -d "${dirs[0]}" ]; then
    gate_error "$(gate_name): no crates/*/src under $PWD — the gate scanned nothing, which is not a pass"
    exit 1
  fi
  GATE_SCAN_FILES=$(find crates/*/src -type f -name '*.rs' | wc -l)
  if [ "$GATE_SCAN_FILES" -eq 0 ]; then
    gate_error "$(gate_name): no .rs files under crates/*/src in $PWD — the gate scanned nothing, which is not a pass"
    exit 1
  fi
}

# Same rule for a gate whose subject is one named file. Without this a
# renamed subject makes the gate pass green forever (see
# bit-identity-debug-only.sh's header for the live instance).
gate_require_file() {
  if [ ! -f "$1" ]; then
    gate_error "$(gate_name): $1 does not exist under $PWD — the gate's subject is gone, so it cannot decide; move the gate with the file or retire it deliberately"
    exit 1
  fi
  GATE_SCAN_FILES=1
}

# Gates say what they proved, like their siblings
# (`scripts/check-test-aggregation.sh`, `check-interval-cfg-additive.py`).
gate_ok() {
  local plural=s
  [ "$GATE_SCAN_FILES" = 1 ] && plural=
  printf '%s OK: %s (%s source file%s scanned)\n' \
    "$(gate_name)" "$1" "$GATE_SCAN_FILES" "$plural"
}

# The clean fixture every self-test starts from. A gate whose subject is
# not `crates/*/src` overrides this.
gate_plant_clean() {
  mkdir -p "$1/crates/clean/src"
  printf 'pub fn identity(x: f64) -> f64 { x }\n' > "$1/crates/clean/src/lib.rs"
}

# gate_selftest WANT PLANTER [ARGS...] — two fixtures, both required:
#
#  NEGATIVE CONTROL: the clean fixture must PASS. Without it a positive
#  result proves nothing — a gate that fires on everything would pass a
#  plant-only self-test.
#  POSITIVE: the same fixture plus one planted violation must FAIL with
#  a message containing WANT.
#
# The gate body is unparameterised apart from its root, so both cases
# exercise the real regex, the real allowlist, and the scan-target guard.
gate_selftest() {
  local want=$1; shift
  local tmp out

  tmp=$(mktemp -d)
  gate_plant_clean "$tmp"
  if ! out=$(cd "$tmp" && gate 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: the gate FAILED on a clean fixture\n%s\n' "$out" >&2
    exit 1
  fi
  rm -rf "$tmp"

  tmp=$(mktemp -d)
  gate_plant_clean "$tmp"
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
  printf '%s selftest OK: passes a clean fixture, fires on a planted violation\n' "$(gate_name)"
}

# The common tail: --selftest runs both fixtures and exits; otherwise
# the gate runs against GATE_ROOT.
gate_main() {
  if [ "$GATE_SELFTEST" = true ]; then
    gate_selftest "$@"
    exit 0
  fi
  cd "$GATE_ROOT"
  gate
}

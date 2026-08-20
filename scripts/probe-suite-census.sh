#!/usr/bin/env bash
# probe-suite-census.sh — which crates own `probe`-gated test suites, and
# the guard that the answer is not silently empty.
#
# THE INVARIANT: every `probe`-gated integration-test suite in the
# workspace is compiled by CI. `probe` (`geom_core::k_stats::Probe`) is a
# `Real` instantiation and an opt-in feature, so a `tests/*.rs` file
# behind `#![cfg(feature = "probe")]` is invisible to every default row:
# it can rot into a build error, or out of existence, with the whole
# matrix green.
#
# WHY A DERIVED CENSUS AND NOT A LIST. The set of crates owning such
# suites is not a constant — it has grown four times — so a literal crate
# list in a workflow is a snapshot that goes stale silently, which is the
# same failure one level up. This script derives the set from the tree,
# and the job that type-checks those crates consumes exactly what it
# prints. A crate that gains its first `probe` suite is therefore covered
# by the next run, with nothing to remember.
#
# WHAT MUST STILL BE LOUD. A derived selection that matches NOTHING
# exits 0 — the exact silence this whole mechanism exists to remove. So
# the census refuses an empty answer, and it refuses an answer missing
# any crate on FLOOR below: those are the crates the coverage was argued
# for, so losing one has to be a deliberate edit here rather than a
# quiet return to zero. Growth is free; shrinkage is not.
#
# Usage:
#   scripts/probe-suite-census.sh              # report, one crate per line, to stderr
#   scripts/probe-suite-census.sh --crates     # bare crate names on stdout, for a loop
#   scripts/probe-suite-census.sh --selftest   # prove the guards fire
#   scripts/probe-suite-census.sh --root DIR   # scan DIR instead of this repo
set -euo pipefail

CENSUS_REPO_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
CENSUS_ROOT=$CENSUS_REPO_ROOT
CENSUS_MODE=report

# The crates whose `probe` suites this coverage was argued for. Not the
# job's crate list — that is derived — but the floor beneath it.
CENSUS_FLOOR=(editor-core geom-brep profile sweep topo)

while [ $# -gt 0 ]; do
  case "$1" in
    --crates) CENSUS_MODE=crates ;;
    --selftest) CENSUS_MODE=selftest ;;
    --root) CENSUS_ROOT=$2; shift ;;
    *) printf 'usage: %s [--crates] [--selftest] [--root DIR]\n' "$0" >&2; exit 2 ;;
  esac
  shift
done

census_error() {
  if [ -n "${GITHUB_ACTIONS:-}" ]; then
    printf '::error::probe-suite-census: %s\n' "$*" >&2
  else
    printf 'ERROR: probe-suite-census: %s\n' "$*" >&2
  fi
}

# Every `crates/*/tests/**/*.rs` naming the `probe` feature, whether the
# gate is the file-level `#![cfg(feature = "probe")]` or a per-item
# `#[cfg(feature = "probe")]` inside an otherwise ungated suite — both
# forms exist in the tree and both need the feature to compile.
census_files() {
  local root=$1
  find "$root"/crates/*/tests -type f -name '*.rs' 2>/dev/null | sort |
    xargs -r grep -l 'feature = "probe"' 2>/dev/null || true
}

# Reads a file list on stdin, prints `<crate> <count>` per owning crate.
census_tally() {
  sed 's|.*/crates/\([^/]*\)/tests/.*|\1|' | sort | uniq -c |
    while read -r n c; do printf '%s %s\n' "$c" "$n"; done
}

census() {
  local root=$1 files tally missing=()

  # A CENSUS THAT SCANNED NOTHING IS NOT A PASS. `crates/*/tests` is a
  # glob; with no match `find` is handed the literal, prints nothing, and
  # an empty answer looks exactly like a clean one.
  if ! find "$root"/crates/*/tests -maxdepth 0 -type d >/dev/null 2>&1; then
    census_error "no crates/*/tests under $root — the census scanned nothing, which is not a pass"
    return 1
  fi

  files=$(census_files "$root")
  if [ -z "$files" ]; then
    census_error "no crates/*/tests file names the \`probe\` feature under $root.
Either every probe-gated suite is gone, or the feature was renamed and this
pattern no longer matches it. Both mean the CI job consuming this census now
type-checks NOTHING, so this is a failure and not a clean tree."
    return 1
  fi

  tally=$(printf '%s\n' "$files" | census_tally)

  for want in "${CENSUS_FLOOR[@]}"; do
    printf '%s\n' "$tally" | grep -q "^$want " || missing+=("$want")
  done
  if [ ${#missing[@]} -gt 0 ]; then
    census_error "crates on the census floor own no probe-gated test suite: ${missing[*]}.
Coverage was argued for these crates by name. If a crate genuinely no longer
has probe suites, drop it from CENSUS_FLOOR in this script deliberately; do not
let the job quietly stop covering it."
    return 1
  fi

  if [ "$CENSUS_MODE" = crates ]; then
    printf '%s\n' "$tally" | cut -d' ' -f1
  else
    printf 'probe-suite census OK: %s suites across %s crates (floor: %s)\n' \
      "$(printf '%s\n' "$files" | wc -l | tr -d ' ')" \
      "$(printf '%s\n' "$tally" | wc -l | tr -d ' ')" \
      "${CENSUS_FLOOR[*]}" >&2
    printf '%s\n' "$tally" | while read -r c n; do
      printf '  %-14s %s suite(s)\n' "$c" "$n" >&2
    done
  fi
}

# ---------------------------------------------------------------- selftest
#
# The guards above are the whole value of this script, so they are proved
# rather than asserted: a clean fixture must pass, and each way the
# census can go silently empty must fail. Run by both halves of CI
# immediately before the real pass, the way `scripts/gates/*` do.

plant_clean() {
  local t=$1 c
  for c in "${CENSUS_FLOOR[@]}"; do
    mkdir -p "$t/crates/$c/tests"
    printf '#![cfg(feature = "probe")]\n' > "$t/crates/$c/tests/probe_thing.rs"
  done
  mkdir -p "$t/crates/plain/tests"
  printf '// no feature gate here\n' > "$t/crates/plain/tests/all.rs"
}

selftest_case() {
  local want=$1; shift
  local t out
  t=$(mktemp -d)
  plant_clean "$t"
  "$@" "$t"
  if out=$(census "$t" 2>&1); then
    rm -rf "$t"
    printf 'SELFTEST FAILED: the census PASSED on a planted violation (%s)\n%s\n' "$1" "$out" >&2
    exit 1
  fi
  rm -rf "$t"
  case "$out" in
    *"$want"*) ;;
    *) printf 'SELFTEST FAILED (%s): fired with an unexpected message:\n%s\n' "$1" "$out" >&2
       exit 1 ;;
  esac
}

# The negative control: without it a census that refused everything would
# pass a plant-only selftest.
selftest_clean() {
  local t out
  t=$(mktemp -d)
  plant_clean "$t"
  if ! out=$(census "$t" 2>&1); then
    rm -rf "$t"
    printf 'SELFTEST FAILED: the census FAILED on a clean fixture\n%s\n' "$out" >&2
    exit 1
  fi
  rm -rf "$t"
}

plant_no_tests_dirs() { rm -rf "$1"/crates/*/tests; }
plant_feature_renamed() { sed -i 's/"probe"/"probe2"/' "$1"/crates/*/tests/probe_thing.rs; }
plant_one_crate_dropped() { rm -f "$1/crates/${CENSUS_FLOOR[0]}/tests/probe_thing.rs"; }

selftest() {
  selftest_clean
  selftest_case 'scanned nothing' plant_no_tests_dirs
  selftest_case 'no longer matches it' plant_feature_renamed
  selftest_case "floor own no probe-gated test suite: ${CENSUS_FLOOR[0]}" plant_one_crate_dropped
  printf 'probe-suite-census selftest OK: passes a clean fixture; fires on an absent tests/ tree, on a renamed feature, and on a floor crate losing its suites\n' >&2
}

if [ "$CENSUS_MODE" = selftest ]; then
  selftest
  exit 0
fi
census "$CENSUS_ROOT"

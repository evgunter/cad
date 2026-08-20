#!/usr/bin/env bash
# probe-suite-census.sh — every `probe`-gated test suite is compiled by
# CI, and every sentence claiming so names the step that provides it.
#
# THE INVARIANT. `probe` (`geom_core::k_stats::Probe`) is an opt-in
# feature, so a `tests/*.rs` behind `#![cfg(feature = "probe")]` is
# invisible to every default row: it can rot into a build error, or out
# of existence, with the whole matrix green. This gate derives the census
# of such suites; `k-lint`'s *type-check every probe-gated test target*
# step `cargo check`s exactly the crates it prints.
#
# WHY DERIVED AND NOT LISTED. The set of crates owning such suites is not
# a constant — it has grown four times — so a literal list in a workflow
# is a snapshot that goes stale silently, which is the same failure one
# level up. A crate that gains its first `probe` suite is covered by the
# next run, with nothing to remember.
#
# WHY THE PREDICATE IS THE GATE LINE. An earlier version matched the
# substring `feature = "probe"` anywhere in the file, which is a MENTION,
# not a gate: a doc comment naming the feature satisfied it, so the floor
# below was satisfiable by prose — including prose describing this very
# mechanism. Matching the cfg attribute itself is what makes the census a
# statement about what compiles.
#
# WHAT MUST STILL BE LOUD. A derived selection that matches NOTHING exits
# 0, which is the silence this mechanism exists to remove. So the census
# refuses an empty answer, and refuses any crate on CENSUS_FLOOR falling
# below the suite count its coverage was argued for. Growth is free — a
# new suite or a new crate is simply covered; shrinkage is a deliberate
# edit here. A file re-gated onto a misspelt feature drops out of the
# census, which is exactly what the per-crate floor catches.
#
# Usage: --selftest (prove the guards fire) | --crates (bare crate names
# on stdout, for the type-check loop) | --root DIR | no args (report).
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

GATE_SCAN_NOUN="probe-gated test suite"

# `<crate>:<suites>` — the crates whose probe coverage was argued for,
# and the suite count each had when it was. Not the type-check loop's
# crate list: that is derived. This is the floor beneath it.
CENSUS_FLOOR=(editor-core:2 geom-brep:4 profile:4 sweep:1 topo:5)

# The step whose existence the four sentences below cite. #706's
# `release-corruption` job carries the same defence for the same reason:
# renaming the step would otherwise leave every one of them quietly
# false, and a claim citing a mechanism nobody checks is how the four
# sentences this gate now guards came to be wrong in the first place.
CITED_STEP='type-check every probe-gated test target'
CITING_FILES=(
  crates/topo/tests/probe_s5_sectors.rs
  crates/sweep/tests/k_report.rs
  docs/K-REPORT.md
  docs/SMELL-SCAN-2026-08.md
)

CENSUS_CRATES=false
gate_args=()
for a in "$@"; do
  case "$a" in
    --crates) CENSUS_CRATES=true ;;
    *) gate_args+=("$a") ;;
  esac
done
gate_parse_args ${gate_args[@]+"${gate_args[@]}"}

# The cfg ATTRIBUTE, file-level or per-item — both spellings are live
# (`crates/topo/tests/review_m3_pr2.rs` gates a single test), and both
# need the feature to compile.
PROBE_GATE_RE='^[[:space:]]*#!?\[cfg\(feature = "probe"\)\][[:space:]]*$'

census_files() {
  find crates/*/tests -type f -name '*.rs' 2>/dev/null | sort |
    xargs -r grep -lE "$PROBE_GATE_RE" 2>/dev/null || true
}

census_tally() {
  sed 's|^crates/\([^/]*\)/tests/.*|\1|' | sort | uniq -c |
    while read -r n c; do printf '%s %s\n' "$c" "$n"; done
}

gate() {
  local files tally rc=0 entry want n have

  # A CENSUS THAT SCANNED NOTHING IS NOT A PASS. `crates/*/tests` is a
  # glob; with no match `find` is handed the literal, prints nothing, and
  # an empty answer looks exactly like a clean one.
  if ! find crates/*/tests -maxdepth 0 -type d >/dev/null 2>&1; then
    gate_error "$(gate_name): no crates/*/tests under $PWD — the census scanned nothing, which is not a pass"
    exit 1
  fi

  files=$(census_files)
  if [ -z "$files" ]; then
    gate_error "$(gate_name): no crates/*/tests file carries a \`probe\` cfg gate under $PWD. Either every probe-gated suite is gone, or the gate spelling changed and this predicate no longer matches it. Both mean the type-check loop covers NOTHING, so this is a failure and not a clean tree."
    exit 1
  fi

  tally=$(printf '%s\n' "$files" | census_tally)

  for entry in "${CENSUS_FLOOR[@]}"; do
    want=${entry%%:*}; n=${entry##*:}
    have=$(printf '%s\n' "$tally" | awk -v c="$want" '$1==c {print $2}')
    have=${have:-0}
    if [ "$have" -lt "$n" ]; then
      gate_error "$(gate_name): $want carries $have probe-gated test suite(s), below the $n its coverage was argued for. A suite deleted, renamed, or re-gated onto a misspelt feature reads exactly like this. If the drop is intended, lower it in CENSUS_FLOOR deliberately; do not let the loop quietly stop covering it."
      rc=1
    fi
  done
  [ "$rc" -eq 0 ] || exit 1

  if [ "$CENSUS_CRATES" = true ]; then
    printf '%s\n' "$tally" | cut -d' ' -f1
    return 0
  fi

  # THE CITATION HALF. Four sentences in the tree describe what CI does
  # to these suites. Nothing else greps for the step that makes them
  # true, so renaming it would leave all four quietly false — the exact
  # shape that let their predecessors rot.
  for entry in "${CITING_FILES[@]}"; do
    [ -f "$entry" ] || { gate_error "$(gate_name): $entry is gone; it cited CI's \`$CITED_STEP\` step. Move the citation with the file or drop it from CITING_FILES deliberately"; rc=1; continue; }
    grep -qF "$CITED_STEP" "$entry" ||
      { gate_error "$(gate_name): $entry no longer names CI's \`$CITED_STEP\` step, but describes what CI does to probe-gated suites. Re-cite the step or rewrite the claim"; rc=1; }
  done
  if [ -f .github/workflows/ci.yml ]; then
    grep -qF "$CITED_STEP" .github/workflows/ci.yml ||
      { gate_error "$(gate_name): .github/workflows/ci.yml has no step named \`$CITED_STEP\`, which ${#CITING_FILES[@]} files cite as their mechanism"; rc=1; }
  else
    gate_error "$(gate_name): .github/workflows/ci.yml does not exist under $PWD — the cited step cannot be checked"
    rc=1
  fi
  [ "$rc" -eq 0 ] || exit 1

  GATE_SCAN_FILES=$(printf '%s\n' "$files" | wc -l | tr -d ' ')
  gate_ok "$(printf '%s\n' "$tally" | wc -l | tr -d ' ') crates own probe-gated suites, all at or above their floor, and ${#CITING_FILES[@]} citing files name the step that covers them"
  printf '%s\n' "$tally" | while read -r c n; do printf '  %-14s %s suite(s)\n' "$c" "$n"; done
}

# The fixture is a miniature repo: crate test trees, the four citing
# files, and a ci.yml naming the cited step.
gate_plant_clean() {
  local t=$1 entry c n i
  for entry in "${CENSUS_FLOOR[@]}"; do
    c=${entry%%:*}; n=${entry##*:}
    mkdir -p "$t/crates/$c/tests"
    for ((i = 0; i < n; i++)); do
      printf '#![cfg(feature = "probe")]\n' > "$t/crates/$c/tests/probe_$i.rs"
    done
  done
  mkdir -p "$t/crates/plain/tests"
  printf '// mentions feature = "probe" in prose only\n' > "$t/crates/plain/tests/all.rs"
  mkdir -p "$t/.github/workflows"
  printf 'jobs: { k-lint: { steps: [{ name: %s }] } }\n' "$CITED_STEP" \
    > "$t/.github/workflows/ci.yml"
  for entry in "${CITING_FILES[@]}"; do
    mkdir -p "$t/$(dirname "$entry")"
    printf 'cites CI %s\n' "$CITED_STEP" > "$t/$entry"
  done
}

plant_no_tests_dirs() { rm -rf "$1"/crates/*/tests; }
# The gate spelling changed under every file at once.
plant_gate_renamed() { sed -i 's/"probe"/"probe2"/' "$1"/crates/*/tests/probe_*.rs; }
# ONE file re-gated onto an always-off feature: the per-crate floor is
# what turns 5 -> 4 into a failure.
plant_one_file_misgated() { sed -i 's/"probe"/"prboe"/' "$1/crates/topo/tests/probe_0.rs"; }
# The gate line deleted, the file's PROSE mention of the feature left —
# the case the substring predicate could not tell from a real gate.
plant_prose_only() {
  printf '// this file is #![cfg(feature = "probe")] in spirit\n' \
    > "$1/crates/sweep/tests/probe_0.rs"
}
plant_citation_dropped() { printf 'no longer cites it\n' > "$1/${CITING_FILES[1]}"; }
plant_step_renamed() { printf 'jobs: {}\n' > "$1/.github/workflows/ci.yml"; }

gate_selftest() {
  gate_selftest_clean
  gate_selftest_case 'scanned nothing' plant_no_tests_dirs
  gate_selftest_case 'no longer matches it' plant_gate_renamed
  gate_selftest_case 'topo carries 4 probe-gated test suite(s), below the 5' plant_one_file_misgated
  gate_selftest_case 'sweep carries 0 probe-gated test suite(s), below the 1' plant_prose_only
  gate_selftest_case 'no longer names CI' plant_citation_dropped
  gate_selftest_case 'has no step named' plant_step_renamed
  printf '%s selftest OK: passes a clean fixture; fires on an absent tests/ tree, a renamed gate spelling, one file re-gated onto a misspelt feature, a gate line replaced by a prose mention, a dropped citation, and a renamed CI step\n' "$(gate_name)"
}

gate_main

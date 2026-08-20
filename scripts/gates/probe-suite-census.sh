#!/usr/bin/env bash
# probe-suite-census.sh — every `probe`-gated test suite is compiled by
# CI, and every sentence claiming so names the step that provides it.
#
# THE INVARIANT. `probe` (`geom_core::k_stats::Probe`) is an opt-in
# feature, so a `tests/*.rs` behind a `probe` cfg gate is invisible to
# every default row: it can rot into a build error, or out of existence,
# with the whole matrix green. This gate derives the census of such
# suites; `k-lint`'s *type-check every probe-gated test target* step
# `cargo check`s exactly the crates it prints.
#
# WHY DERIVED AND NOT LISTED. The set of crates owning such suites is not
# a constant — it has grown four times — so a literal list in a workflow
# is a snapshot that goes stale silently, which is the same failure one
# level up. A crate that gains its first `probe` suite is covered by the
# next run, with nothing to remember.
#
# WHY THE PREDICATE IS THE CFG ATTRIBUTE, AND WHY IT IS NOT VERBATIM.
# Matching a MENTION of the feature is wrong: an earlier version matched
# the substring `feature = "probe"` anywhere in the file, so a doc
# comment naming the feature satisfied the floor below — including prose
# describing this very mechanism. Requiring the ATTRIBUTE form, anchored
# to the whole line, is what makes the census a statement about what
# compiles rather than about what is written.
#
# Requiring the attribute VERBATIM is the same mistake from the other
# side. `#![cfg(all(feature = "probe", not(miri)))]` is a correct gate; a
# verbatim predicate has no vocabulary for it, so such a file would be
# uncounted — and a crate whose FIRST probe suite were spelled that way
# would never enter the derived crate list at all, which is silence, not
# a wrong number. So the condition is matched anywhere inside the
# attribute's parentheses: `all(…)`, `any(…)` and feature conjunctions
# all count.
#
# THREE WAYS A GATE CAN BE UNCOVERED, AND WHICH MECHANISM ANSWERS EACH.
# The predicate above is only one of them, and saying so is the point:
#
#   1. MISSPELT — `#![cfg(feature = "prboe")]`. Not this gate's, and it
#      does not need to be. Cargo emits `--check-cfg` for every feature a
#      manifest declares, so rustc's `unexpected_cfgs` fires on an
#      unknown feature VALUE even where the condition is false and the
#      module is stripped, and the workspace `clippy` row's `-D warnings`
#      makes it an error. That row is therefore part of THIS gate's
#      coverage argument, so it is checked below rather than assumed:
#      neither may the row lose `--all-targets` or `-D warnings`, nor may
#      anything under `crates/` silence the lint.
#
#   2. COMPOUND — `#![cfg(all(feature = "probe", not(miri)))]`. Correct,
#      and covered, once the predicate has vocabulary for it: the file is
#      counted, so its crate enters the derived type-check list. A
#      verbatim predicate is what made this case silent, and only for the
#      crate whose FIRST probe suite is spelled that way.
#
#   3. REQUIRES A SECOND FEATURE — `all(feature = "probe", feature =
#      "interval")`. Correctly spelled, counted, and compiled by nothing:
#      no CI row builds `probe` with another feature. The compiler cannot
#      object, so it is REFUSED below, loudly, rather than counted.
#
# WHAT IS LEFT, AND WHY IT IS LEFT. A gate whose extra condition is not a
# feature and is false on every runner — `not(target_os = "linux")`,
# `miri` — is correctly spelled, counted, silenced by nothing, and
# compiled nowhere. Only a behavioural check sees it: `cargo test -p
# <crate> --features probe --test all -- --list` per crate, asserting
# every counted suite appears as a module prefix. It was built, it works
# (planting such a gate reds the listing while `cargo clippy -p topo
# --all-targets -- -D warnings` stays green), and it was rejected ON A
# MEASUREMENT. From an empty target dir on a 4-vCPU container, the
# `k-lint` job's probe steps cost 141 s today (55 s type-check loop, 85 s
# for the sweep's own probe binaries); building every probe target and
# listing the aggregate costs 205 s. +64 s on every merge buys the case
# above and nothing else, and the three cases that are actually plausible
# are answered for free. If that trade ever changes, the check is four
# lines and this paragraph is the argument to re-run.
#
# WHAT THIS PREDICATE CANNOT MATCH, stated because the previous one's
# blind spot was not: a gate broken across lines (rustfmt keeps these on
# one); a gate reached through a `cfg_attr` or a macro; and
# `cfg(not(feature = "probe"))`, which it counts as a probe suite though
# it means the opposite — an over-count, and the harmless direction,
# since such a file compiles under the DEFAULT rows and is covered there.
#
# WHAT MUST STILL BE LOUD. A derived selection that matches NOTHING exits
# 0, which is the silence this mechanism exists to remove. So the census
# refuses an empty answer, and refuses any crate on CENSUS_FLOOR falling
# below the suite count its coverage was argued for. Growth is free — a
# new suite or a new crate is simply covered; shrinkage is a deliberate
# edit here.
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

# The workspace clippy row, which is what makes a misspelt cfg gate a
# hard error. Every clause is load-bearing: the workspace scope reaches
# every crate, `--all-targets` is what compiles the `tests/all.rs`
# aggregate at all, and `-D warnings` is what promotes `unexpected_cfgs`.
CLIPPY_ROW_RE='cargo clippy .*cargo_scope.*--all-targets.*-D warnings'
# The other way to re-open the hole: silence the lint at the site.
CFG_LINT_SILENCED_RE='(allow|expect)\(unexpected_cfgs\)|unexpected_cfgs[[:space:]]*=[[:space:]]*"allow"'

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
# (`crates/topo/tests/review_m3_pr2.rs` gates a single test) — with the
# `probe` condition anywhere inside its parentheses.
PROBE_GATE_RE='^[[:space:]]*#!?\[cfg\(.*feature = "probe".*\)\][[:space:]]*$'

census_files() {
  find crates/*/tests -type f -name '*.rs' 2>/dev/null | sort |
    xargs -r grep -lE "$PROBE_GATE_RE" 2>/dev/null || true
}

census_tally() {
  sed 's|^crates/\([^/]*\)/tests/.*|\1|' | sort | uniq -c |
    while read -r n c; do printf '%s %s\n' "$c" "$n"; done
}

# `<file>: feature = "X"` for every feature a counted gate names besides
# `probe` itself. Read off the MATCHED LINES, not the whole file: a
# probe suite may of course carry `#[cfg(feature = "interval")]` on some
# item inside it, and that is not this question.
census_second_features() {
  local suite
  while IFS= read -r suite; do
    grep -hE "$PROBE_GATE_RE" "$suite" |
      grep -oE 'feature = "[^"]*"' | grep -vFx 'feature = "probe"' |
      sed "s|^|$suite: |" || true
  done <<<"$1" | sort -u
}

gate() {
  local files tally rc=0 entry want n have silenced extra

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
      gate_error "$(gate_name): $want carries $have probe-gated test suite(s), below the $n its coverage was argued for. A suite deleted, renamed, or re-gated onto a feature this predicate cannot see reads exactly like this. If the drop is intended, lower it in CENSUS_FLOOR deliberately; do not let the loop quietly stop covering it."
      rc=1
    fi
  done
  [ "$rc" -eq 0 ] || exit 1

  # A COUNTED GATE MAY REQUIRE `probe` AND NOTHING ELSE THAT IS A
  # FEATURE. No CI row builds `probe` together with another one — the
  # interval legs are their own build graph, `budget` is its own row —
  # so `#![cfg(all(feature = "probe", feature = "interval"))]` is a
  # correctly spelled gate that compiles under no configuration anyone
  # runs. The compiler cannot object (both names exist) and the floor
  # above still counts it, so without this the file is exactly as
  # invisible as the misspelt one, and considerably more plausible.
  extra=$(census_second_features "$files")
  if [ -n "$extra" ]; then
    gate_error "$(gate_name): a probe suite's gate requires a SECOND feature, and no CI row builds \`probe\` with another one, so the file compiles under no configuration this repo runs: $(printf '%s' "$extra" | tr '\n' ' '). Split the suite, or add the row that builds the combination and teach this gate about it"
    rc=1
  fi
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
    # THE MISSPELT-GATE HALF. This predicate cannot tell a typo from a
    # feature it has not heard of; the compiler can, and says so through
    # `unexpected_cfgs`. That only fails a run where the workspace clippy
    # row still denies warnings over all targets, so the row is checked
    # rather than assumed.
    grep -vE '^[[:space:]]*#' .github/workflows/ci.yml | grep -qE "$CLIPPY_ROW_RE" ||
      { gate_error "$(gate_name): .github/workflows/ci.yml has no workspace \`cargo clippy … --all-targets -- -D warnings\` row. That row is what turns a misspelt cfg gate (\`feature = \"prboe\"\`) into a failure, through rustc's \`unexpected_cfgs\`; without it such a suite compiles to nothing under every row and this census never sees it. Restore the row or re-argue the coverage here"; rc=1; }
  else
    gate_error "$(gate_name): .github/workflows/ci.yml does not exist under $PWD — the cited step cannot be checked"
    rc=1
  fi
  silenced=$(grep -rlE "$CFG_LINT_SILENCED_RE" crates 2>/dev/null || true)
  if [ -n "$silenced" ]; then
    gate_error "$(gate_name): $(printf '%s' "$silenced" | tr '\n' ' ')silences \`unexpected_cfgs\`, which is the only thing that reports a test suite gated on a misspelt feature. Code that needs an unknown cfg name declares it in check-cfg values, never by allowing the lint"
    rc=1
  fi
  [ "$rc" -eq 0 ] || exit 1

  GATE_SCAN_FILES=$(printf '%s\n' "$files" | wc -l | tr -d ' ')
  gate_ok "$(printf '%s\n' "$tally" | wc -l | tr -d ' ') crates own probe-gated suites, all at or above their floor, ${#CITING_FILES[@]} citing files name the step that covers them, and the clippy row that reports a misspelt gate is wired"
  printf '%s\n' "$tally" | while read -r c n; do printf '  %-14s %s suite(s)\n' "$c" "$n"; done
}

# The fixture is a miniature repo: crate test trees, the four citing
# files, and a ci.yml carrying both rows this gate reads.
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
  {
    printf 'jobs:\n  clippy:\n    steps:\n'
    printf '      - run: cargo clippy ${{ needs.filter.outputs.cargo_scope }} --all-targets -- -D warnings\n'
    printf '  k-lint:\n    steps:\n      - name: %s\n' "$CITED_STEP"
  } > "$t/.github/workflows/ci.yml"
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
# The clippy row loses the flag that promotes `unexpected_cfgs`.
plant_clippy_undenied() { sed -i 's/ -- -D warnings//' "$1/.github/workflows/ci.yml"; }
# The lint silenced at the site instead.
plant_cfg_lint_allowed() {
  printf '#![allow(unexpected_cfgs)]\n#![cfg(feature = "probe")]\n' \
    > "$1/crates/topo/tests/probe_0.rs"
}
# A gate no CI row builds, spelled correctly. Nothing else in the tree
# can object to it: both feature names exist, so the compiler is silent.
plant_second_feature() {
  printf '#![cfg(all(feature = "probe", feature = "interval"))]\n' \
    > "$1/crates/topo/tests/probe_0.rs"
}

# A COMPOUND GATE IS CORRECT, NOT A VIOLATION, and the fixture asserts
# the census counts it — `sweep`'s floor is 1 over exactly one file, so
# a predicate with no vocabulary for `all(…)` reds this case.
selftest_compound_counted() {
  local tmp out
  tmp=$(mktemp -d)
  gate_plant_clean "$tmp"
  printf '#![cfg(all(feature = "probe", not(miri)))]\n' \
    > "$tmp/crates/sweep/tests/probe_0.rs"
  if ! out=$(cd "$tmp" && gate 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: a compound `probe` gate was not counted as a suite\n%s\n' "$out" >&2
    exit 1
  fi
  rm -rf "$tmp"
}

gate_selftest() {
  gate_selftest_clean
  selftest_compound_counted
  gate_selftest_case 'scanned nothing' plant_no_tests_dirs
  gate_selftest_case 'no longer matches it' plant_gate_renamed
  gate_selftest_case 'topo carries 4 probe-gated test suite(s), below the 5' plant_one_file_misgated
  gate_selftest_case 'sweep carries 0 probe-gated test suite(s), below the 1' plant_prose_only
  gate_selftest_case 'no longer names CI' plant_citation_dropped
  gate_selftest_case 'has no step named' plant_step_renamed
  gate_selftest_case 'no workspace `cargo clippy' plant_clippy_undenied
  gate_selftest_case 'silences `unexpected_cfgs`' plant_cfg_lint_allowed
  gate_selftest_case 'requires a SECOND feature' plant_second_feature
  printf '%s selftest OK: passes a clean fixture and counts a compound gate; fires on an absent tests/ tree, a renamed gate spelling, one file re-gated onto a misspelt feature, a gate line replaced by a prose mention, a dropped citation, a renamed CI step, a clippy row that stopped denying warnings, the cfg lint silenced at the site, and a gate requiring a second feature no CI row builds\n' "$(gate_name)"
}

gate_main

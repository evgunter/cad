#!/usr/bin/env bash
# probe-suite-census.sh — every `probe`-gated test suite is compiled by
# CI, and every sentence claiming so names the step that provides it.
#
# THE INVARIANT. `probe` (`geom_core::k_stats::Probe`) is an opt-in
# feature, so a `tests/*.rs` behind a `probe` cfg gate is invisible to
# every default row: it can rot into a build error, or out of existence,
# with the whole matrix green. This gate derives the census of such
# suites; `k-lint`'s *compile and list every probe-gated test target*
# step compiles exactly the crates it prints, and feeds each crate's test
# listing back to `--check-listing` below.
#
# WHY DERIVED AND NOT LISTED. The set of crates owning such suites is not
# a constant — it has grown four times — so a literal list in a workflow
# is a snapshot that goes stale silently, which is the same failure one
# level up. A crate that gains its first `probe` suite is covered by the
# next run, with nothing to remember.
#
# WHY THE PREDICATE IS THE CFG ATTRIBUTE, AND WHY IT IS NOT VERBATIM.
# Matching a MENTION of the feature is wrong: a doc comment naming it is
# not a gate, and the substring form let prose satisfy the floor below —
# including prose describing this very mechanism. Requiring the ATTRIBUTE
# form, anchored to the whole line, is what makes the census a statement
# about what compiles rather than about what is written. Requiring it
# VERBATIM is the same mistake from the other side, which is case 2
# below: the condition is therefore matched anywhere inside the
# attribute's parentheses, so `all(…)`, `any(…)` and conjunctions count.
#
# THREE WAYS A COUNTED GATE CAN GO UNCOVERED, AND WHAT ANSWERS EACH. The
# predicate above is one of them; naming the other two is the point.
#
#   1. MISSPELT (`feature = "prboe"`) — the COMPILER'S, not this gate's.
#      Cargo emits `--check-cfg` per declared feature, so `unexpected_cfgs`
#      fires on an unknown VALUE even where the condition is false and the
#      module is stripped, and a `-D warnings` clippy row makes it an
#      error. That row is part of this gate's coverage argument, so it is
#      checked below rather than assumed.
#
#   2. UNCOUNTED — a correct compound gate the predicate had no vocabulary
#      for, whose crate therefore never entered the derived list. The
#      widening above is the answer.
#
#   3. COUNTED BUT NEVER BUILT — undecidable from the gate line, and the
#      case this gate once got WRONG. Whether `all(feature = "probe",
#      feature = "X")` compiles depends on the crate's manifest, not on
#      the gate: `crates/{topo,sweep}/Cargo.toml` carry SELF
#      DEV-DEPENDENCIES enabling `test-support` and `sweep-testing`, so
#      those gates DO compile, while `interval` and `budget` do not. Nor
#      can a line-reader tell `all` from `any`, or a required feature from
#      a negated one. So this half is BEHAVIOURAL: `--check-listing` reads
#      what the compiler built.
#
# WHERE THE BEHAVIOURAL HALF IS SPLIT, AND WHY THERE. The cargo run stays
# in the workflow beside the compile loop that already consumes
# `--crates`; what arrives here is its `--list` OUTPUT, on stdin. The
# deciding half therefore keeps `--root` and a `--selftest` whose fixture
# is TEXT, not a compilable workspace — which is why it costs no fixture
# build. It was adopted on correctness: it is the only mechanism that
# accepts a `test-support` gate and refuses an `interval` one, both
# checked by planting.
#
# ITS COST IS SMALL, AND BOUNDED RATHER THAN KNOWN. The marginal is the
# link `--test all -- --list` needs, which the sweep two steps later pays
# anyway. Hosted, `k-lint`'s two probe steps summed 196 s and 212 s over
# two runs WITHOUT this half, and 219 s TWICE with it. So the delta is
# +7 s to +23 s, and most of that range is the no-listing arm's own
# spread (16 s) rather than the change. A fifth run priced the
# arrangement that TYPE-CHECKS and then builds at 271 s, which is why the
# step builds directly instead. **These numbers have no guard and cannot
# get one**: nothing in the repo records `k-lint` step times
# (`rebuild-latency` measures kernel rebuilds), and a threshold over
# runner-to-runner variance would fire on weather. They are a decision's
# dated, sourced evidence, not a baseline — a taker who needs them
# re-measures.
#
# WHAT THE PREDICATE CANNOT MATCH, stated because the previous one's blind
# spot was not: a gate split across lines; a gate with a TRAILING COMMENT
# (the whole-line anchors are what keep prose out, and this is their
# price); a gate reached through `cfg_attr` or a macro; and
# `cfg(not(feature = "probe"))`, counted though it means the opposite — an
# over-count, and the harmless direction, since such a file compiles under
# the DEFAULT rows. WHAT THE LISTING HALF CANNOT SEE: a counted suite
# declaring no `#[test]` of its own lists nothing and is reported missing.
# None exists today; the remedy if one arrives is a test or a de-count,
# never an exception list — that would be the second roster this
# directory exists to avoid.
#
# Usage: --selftest | --crates (bare crate names, for the compile loop)
# | --suites (`<crate><TAB><module>` rows) | --check-listing CRATE
# (read a `--test all -- --list` listing on stdin and assert every
# counted suite of CRATE is in it) | --root DIR | no args (report).
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

GATE_SCAN_NOUN="probe-gated test suite"

# `<crate>:<suites>` — the crates whose probe coverage was argued for,
# and the suite count each had when it was. Not the type-check loop's
# crate list: that is derived. This is the floor beneath it.
CENSUS_FLOOR=(editor-core:2 geom-brep:4 profile:4 sweep:1 topo:5)

# The step whose existence the sentences below cite. #706's
# `release-corruption` job carries the same defence for the same reason:
# renaming the step would otherwise leave every one of them quietly
# false, and a claim citing a mechanism nobody checks is how the
# sentences this gate now guards came to be wrong in the first place.
#
# THIS HALF IS SITED IN THE `mirror` JOB (`--citations`), NOT IN
# `discipline`. Its inputs are prose — two crate headers, the K-REPORT
# runbook, and the local half — and a change set that touches only those
# classifies TIER=docs, so `discipline` is skipped and this half could
# not fire on the only change class that breaks it. `mirror` has no
# `if:`, so it runs on every tier, and it does not prune
# `local-scripts/`, so the local half's citation is readable there too.
CITED_STEP='compile and list every probe-gated test target'

# LIVE CLAIMS. Each of these says, in the present tense, what CI does to
# probe-gated suites; each is false the moment the step is renamed.
CITING_FILES=(
  crates/topo/tests/probe_s5_sectors.rs
  crates/sweep/tests/k_report.rs
  docs/K-REPORT.md
  local-scripts/ci-local.sh
)

# NOT LIVE CLAIMS, and the reason this list exists at all. A dated scan
# or a milestone log RECORDS what CI did on the day it was written; the
# step name in it is history and stays correct when the step is renamed.
# `docs/SMELL-SCAN-2026-08.md` used to sit in CITING_FILES, which made a
# 12,000-line living document a hard CI dependency on a literal string —
# every lane on every concurrent track edits it daily, and archiving or
# reorganising it would have reddened the build for a reason its diff
# could not explain. Removing it is not narrowing the guard: the rename
# it exists to catch is caught by the `ci.yml` check below, which is the
# load-bearing half, and by the four live claims above.
#
# `docs/` IS EXEMPT AS A TREE, and that is the deliberate half of the
# trade. The first version exempted named patterns — dated scans, track
# logs, milestone plans — which left `docs/prompts/`, `docs/REVIEW-*.md`
# and every future document quoting the step name as a hard CI failure
# on EVERY tier, in a directory four concurrent tracks write to daily.
# That replaced one literal-string dependency with a wider one. What
# survives is the claim worth having: a citation in CODE or in CI —
# `crates/`, `scripts/`, `local-scripts/`, `.github/` — is a live claim
# about a mechanism and must be registered. `docs/K-REPORT.md` stays a
# registered live claim below because it is the runbook a reader is sent
# to; the rest of `docs/` is record, and a record naming a step by the
# name it had is not false when the step is renamed.
#
# WHAT THIS GIVES UP, plainly: a NEW document that cites the step and
# later goes stale is not caught. The load-bearing check against the
# rename — that ci.yml still carries a step of this name — is unaffected.
#
# Shell globs, matched against the repo-relative path; `*` matches `/`.
CITATION_EXEMPT=(
  '.github/workflows/ci.yml'                # the step itself
  'scripts/gates/probe-suite-census.sh'     # this gate
  'docs/*'                                  # records, plans, prompts, logs
)

# The clippy row that makes a misspelt cfg gate a hard error.
# `--all-targets` is what compiles the `tests/all.rs` aggregate at all,
# and `-D warnings` is what promotes `unexpected_cfgs`. The scope clause
# is matched too, but claim only what it gives: `cargo_scope` is
# `--workspace` on TIER=all and `-p a -p b …` on a narrower tier
# (`ci-filter.py`), so what this pins is that the row is SCOPED BY THE
# FILTER rather than hand-pinned — not that every crate is linted on
# every run. A new suite lands in its own crate's scope either way.
CLIPPY_ROW_RE='cargo clippy .*cargo_scope.*--all-targets.*-D warnings'
# The other way to re-open the hole: silence the lint at the site.
CFG_LINT_SILENCED_RE='(allow|expect)\(unexpected_cfgs\)|unexpected_cfgs[[:space:]]*=[[:space:]]*"allow"'

CENSUS_CRATES=false
CENSUS_CITATIONS=false
CENSUS_SUITES=false
CENSUS_LISTING=
gate_args=()
want_crate=false
for a in "$@"; do
  if [ "$want_crate" = true ]; then CENSUS_LISTING=$a; want_crate=false; continue; fi
  case "$a" in
    --crates) CENSUS_CRATES=true ;;
    --citations) CENSUS_CITATIONS=true ;;
    --suites) CENSUS_SUITES=true ;;
    --check-listing) want_crate=true ;;
    *) gate_args+=("$a") ;;
  esac
done
if [ "$want_crate" = true ]; then
  printf 'usage: %s --check-listing CRATE < listing\n' "$0" >&2; exit 2
fi
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

# THE CITATION HALF, sited in the `mirror` job. Sentences in the tree
# describe what CI does to these suites; nothing else greps for the step
# that makes them true, so renaming it would leave all of them quietly
# false — the exact shape that let their predecessors rot.
census_citations() {
  local rc=0 entry hit pat exempt
  if [ ! -f .github/workflows/ci.yml ]; then
    gate_error "$(gate_name): .github/workflows/ci.yml does not exist under $PWD — the cited step cannot be checked"
    exit 1
  fi
  grep -qF "$CITED_STEP" .github/workflows/ci.yml ||
    { gate_error "$(gate_name): .github/workflows/ci.yml has no step named \`$CITED_STEP\`, which ${#CITING_FILES[@]} files cite as their mechanism"; rc=1; }

  for entry in "${CITING_FILES[@]}"; do
    [ -f "$entry" ] || { gate_error "$(gate_name): $entry is gone; it cited CI's \`$CITED_STEP\` step. Move the citation with the file or drop it from CITING_FILES deliberately"; rc=1; continue; }
    grep -qF "$CITED_STEP" "$entry" ||
      { gate_error "$(gate_name): $entry no longer names CI's \`$CITED_STEP\` step, but describes what CI does to probe-gated suites. Re-cite the step or rewrite the claim"; rc=1; }
  done

  # COMPLETENESS, so the list above cannot silently go stale. A hand
  # list of claim sites is a roster, and an unchecked roster drifts:
  # every file in the tree that names the step is either a live claim
  # (in CITING_FILES) or declared history (CITATION_EXEMPT). A new
  # citation lands visible instead of unguarded.
  while IFS= read -r hit; do
    hit=${hit#./}
    for entry in "${CITING_FILES[@]}"; do
      [ "$hit" = "$entry" ] && continue 2
    done
    exempt=false
    for pat in "${CITATION_EXEMPT[@]}"; do
      # shellcheck disable=SC2053
      [[ $hit == $pat ]] && { exempt=true; break; }
    done
    [ "$exempt" = true ] && continue
    gate_error "$(gate_name): $hit names CI's \`$CITED_STEP\` step but is neither a live claim this gate keeps true (CITING_FILES) nor declared history (CITATION_EXEMPT). A citation nobody checks is how the sentences this gate guards became false in the first place — add it to one list or the other, with the reason"
    rc=1
  done < <(grep -rlF "$CITED_STEP" . \
             --exclude-dir=.git --exclude-dir=target --exclude-dir=node_modules \
             2>/dev/null | sort)

  [ "$rc" -eq 0 ] || exit 1
  GATE_SCAN_FILES=${#CITING_FILES[@]}
  GATE_SCAN_NOUN='citing file'
  gate_ok "every live claim about the probe type-check loop names the \`$CITED_STEP\` step that ci.yml still carries, and no undeclared citation of it exists in the tree"
}

gate() {
  local files tally rc=0 entry want n have silenced hosted
  local suite crate rest listed missing suites

  if [ "$CENSUS_CITATIONS" = true ]; then
    census_citations
    return 0
  fi

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

  if [ "$CENSUS_CRATES" = true ]; then
    printf '%s\n' "$tally" | cut -d' ' -f1
    return 0
  fi

  # `<crate><TAB><module>`. The module is the file stem because the
  # aggregator declares it that way (`#[path = "x.rs"] mod x;`), which is
  # what `test-aggregation.sh` keeps true. A suite NESTED under `tests/`
  # would need its parent module too, so it is refused rather than
  # mis-reported: none is nested today.
  if [ "$CENSUS_SUITES" = true ] || [ -n "$CENSUS_LISTING" ]; then
    suites=$(
      while IFS= read -r suite; do
        crate=${suite#crates/}; crate=${crate%%/*}
        rest=${suite#crates/*/tests/}
        case $rest in
          */*) gate_error "$(gate_name): $suite is a probe suite nested under tests/, and the module a listing names is the file stem alone. Flatten it, or teach --suites the nesting"
               exit 1 ;;
        esac
        printf '%s\t%s\n' "$crate" "${rest%.rs}"
      done <<<"$files"
    )
  fi
  if [ "$CENSUS_SUITES" = true ]; then
    printf '%s\n' "$suites"
    return 0
  fi

  # THE BEHAVIOURAL HALF. Given a `cargo test -p CRATE --features probe
  # --test all -- --list` listing on stdin, every suite this census
  # counted for CRATE must appear in it. A counted file that compiles
  # under no configuration the loop builds lists nothing, and that is the
  # only way to tell it from one that does — the gate line cannot say.
  if [ -n "$CENSUS_LISTING" ]; then
    listed=$(sed -n 's/^\([A-Za-z0-9_]*\)::.*/\1/p' | sort -u)
    if [ -z "$listed" ]; then
      gate_error "$(gate_name): the listing for $CENSUS_LISTING named no test at all. A listing that matched nothing is not a pass — check that the crate builds its \`all\` target under \`--features probe\`"
      exit 1
    fi
    missing=$(
      printf '%s\n' "$suites" |
        awk -F'\t' -v c="$CENSUS_LISTING" '$1==c {print $2}' |
        while IFS= read -r m; do
          [ -n "$m" ] || continue
          grep -qx "$m" <<<"$listed" || printf '%s ' "$m"
        done
    )
    if [ -n "$missing" ]; then
      gate_error "$(gate_name): $CENSUS_LISTING counts these as probe suites, but \`--features probe\` built no test from them: $missing. The census reads the gate LINE; a gate can be spelled correctly and still be true under no configuration CI runs — a second feature the probe loop does not enable (\`interval\`, \`budget\`; \`test-support\` and \`sweep-testing\` ARE enabled, by the crate's self dev-dependency), or a non-feature condition false on every runner. Fix the gate, give the suite a test, or stop counting the file"
      exit 1
    fi
    GATE_SCAN_FILES=$(printf '%s\n' "$suites" | awk -F'\t' -v c="$CENSUS_LISTING" '$1==c' | wc -l | tr -d ' ')
    gate_ok "every probe suite $CENSUS_LISTING counts was built and listed under \`--features probe\`"
    return 0
  fi

  if [ -f .github/workflows/ci.yml ]; then
    # THE MISSPELT-GATE HALF. This predicate cannot tell a typo from a
    # feature it has not heard of; the compiler can, and says so through
    # `unexpected_cfgs`. That only fails a run where the workspace clippy
    # row still denies warnings over all targets, so the row is checked
    # rather than assumed.
    # MATERIALISED, NOT PIPED, for the reason `gate-roster.sh`'s header
    # records: `grep -q` exits on its first match, SIGPIPEs the upstream
    # `grep -v`, and `pipefail` calls the whole pipeline failed. Which
    # side wins is a race — this passed locally on a six-line fixture and
    # fired against a correctly wired ci.yml on the first hosted run.
    hosted=$(grep -vE '^[[:space:]]*#' .github/workflows/ci.yml || true)
    grep -qE "$CLIPPY_ROW_RE" <<<"$hosted" ||
      { gate_error "$(gate_name): .github/workflows/ci.yml has no workspace \`cargo clippy … --all-targets -- -D warnings\` row. That row is what turns a misspelt cfg gate (\`feature = \"prboe\"\`) into a failure, through rustc's \`unexpected_cfgs\`; without it such a suite compiles to nothing under every row and this census never sees it. Restore the row or re-argue the coverage here"; rc=1; }
  else
    gate_error "$(gate_name): .github/workflows/ci.yml does not exist under $PWD — the clippy row that reports a misspelt cfg gate cannot be checked"
    rc=1
  fi
  silenced=$(grep -rlE "$CFG_LINT_SILENCED_RE" crates 2>/dev/null || true)
  if [ -n "$silenced" ]; then
    gate_error "$(gate_name): $(printf '%s' "$silenced" | tr '\n' ' ')silences \`unexpected_cfgs\`, which is the only thing that reports a test suite gated on a misspelt feature. Code that needs an unknown cfg name declares it in check-cfg values, never by allowing the lint"
    rc=1
  fi
  [ "$rc" -eq 0 ] || exit 1

  GATE_SCAN_FILES=$(printf '%s\n' "$files" | wc -l | tr -d ' ')
  gate_ok "$(printf '%s\n' "$tally" | wc -l | tr -d ' ') crates own probe-gated suites, all at or above their floor, and the clippy row that reports a misspelt gate is wired (the citation half runs in the \`mirror\` job, where it can fire on prose)"
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
plant_citing_file_gone() { rm -f "$1/${CITING_FILES[2]}"; }
# A NEW citation of the step, in a file on neither list. Before the
# completeness check the hand list could go stale silently; this is the
# case that keeps it a roster rather than a sample.
plant_undeclared_citation() {
  mkdir -p "$1/scripts"
  printf 'CI runs %s on every merge.\n' "$CITED_STEP" > "$1/scripts/new-check.sh"
}
# The same citation in a file the gate declares as history: exempt, and
# the NEGATIVE CONTROL for the case above.
plant_exempt_citation() {
  mkdir -p "$1/docs/prompts"
  printf 'On 2026-08-20 CI ran %s.\n' "$CITED_STEP" > "$1/docs/SMELL-SCAN-2026-08.md"
  printf 'A brief quoting %s.\n' "$CITED_STEP" > "$1/docs/prompts/zz-new-brief.md"
}
plant_step_renamed() { printf 'jobs: {}\n' > "$1/.github/workflows/ci.yml"; }
# The clippy row loses the flag that promotes `unexpected_cfgs`.
plant_clippy_undenied() { sed -i 's/ -- -D warnings//' "$1/.github/workflows/ci.yml"; }
# The lint silenced at the site instead.
plant_cfg_lint_allowed() {
  printf '#![allow(unexpected_cfgs)]\n#![cfg(feature = "probe")]\n' \
    > "$1/crates/topo/tests/probe_0.rs"
}
# THE BEHAVIOURAL HALF, BOTH DIRECTIONS. Its subject is a listing, so its
# fixture is text: no cargo, no compilable workspace, milliseconds. The
# clean case feeds a listing naming every counted suite of `topo`; the
# planted one drops a single module from it, which is exactly what a gate
# that is spelled correctly and true under nothing produces.
selftest_listing() {
  local tmp out listing
  tmp=$(mktemp -d)
  gate_plant_clean "$tmp"
  listing=$(cd "$tmp" && gate_listing_for topo)
  if ! out=$(cd "$tmp" && printf '%s\n' "$listing" | CENSUS_LISTING=topo gate 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: the listing check FAILED on a complete listing\n%s\n' "$out" >&2
    exit 1
  fi
  if out=$(cd "$tmp" && printf '%s\n' "$listing" | grep -v '^probe_0::' | CENSUS_LISTING=topo gate 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: the listing check PASSED with a counted suite missing\n%s\n' "$out" >&2
    exit 1
  fi
  case "$out" in
    *'built no test from them: probe_0'*) ;;
    *) rm -rf "$tmp"
       printf 'SELFTEST FAILED: the listing check fired with an unexpected message:\n%s\n' "$out" >&2
       exit 1 ;;
  esac
  if out=$(cd "$tmp" && printf '' | CENSUS_LISTING=topo gate 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: the listing check PASSED on an EMPTY listing\n%s\n' "$out" >&2
    exit 1
  fi
  rm -rf "$tmp"
}

# One `<module>::<test>` row per counted suite of CRATE, the shape
# `cargo test -- --list` prints.
gate_listing_for() {
  gate_suites_for "$1" | while IFS= read -r m; do printf '%s::t: test\n' "$m"; done
}
gate_suites_for() {
  CENSUS_SUITES=true gate | awk -F'\t' -v c="$1" '$1==c {print $2}'
}

# A SIX-LINE ci.yml CANNOT SHOW A SIGPIPE RACE. The real one is 2,000
# lines, and a `grep -q` that matches near the top leaves the upstream
# filter writing into a closed pipe — which `pipefail` then reports as a
# failed pipeline, i.e. the gate firing against a correctly wired file.
# That is what a first hosted run did. Padding the fixture past the match
# makes the race deterministic, so the structural fix is held in place.
selftest_hosted_half_is_large() {
  local tmp out i
  tmp=$(mktemp -d)
  gate_plant_clean "$tmp"
  # NOT comment lines: those are what the filter drops, so they would
  # never reach the downstream matcher and the race would not happen.
  for ((i = 0; i < 20000; i++)); do
    printf '      - run: echo filler, below every row this gate matches on\n'
  done >> "$tmp/.github/workflows/ci.yml"
  if ! out=$(cd "$tmp" && gate 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: the gate FAILED on a clean fixture with a long ci.yml\n%s\n' "$out" >&2
    exit 1
  fi
  rm -rf "$tmp"
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
  selftest_hosted_half_is_large
  selftest_listing
  selftest_compound_counted
  gate_selftest_case 'scanned nothing' plant_no_tests_dirs
  gate_selftest_case 'no longer matches it' plant_gate_renamed
  gate_selftest_case 'topo carries 4 probe-gated test suite(s), below the 5' plant_one_file_misgated
  gate_selftest_case 'sweep carries 0 probe-gated test suite(s), below the 1' plant_prose_only
  gate_selftest_case 'no workspace `cargo clippy' plant_clippy_undenied
  gate_selftest_case 'silences `unexpected_cfgs`' plant_cfg_lint_allowed

  # THE CITATION HALF's cases. The mode reaches the gate through argv,
  # not through this shell: lib.sh's harness runs every case as a REAL
  # subprocess, so a global set here does not cross into it. It used to,
  # and that is why these five cases were silently running the CENSUS
  # half — each one planted a broken citation, the census half passed it,
  # and the case reported green (S157).
  GATE_SELFTEST_ARGS=(--citations)
  CENSUS_CITATIONS=true
  gate_selftest_clean
  gate_selftest_case 'no longer names CI' plant_citation_dropped
  gate_selftest_case 'is gone; it cited' plant_citing_file_gone
  gate_selftest_case 'has no step named' plant_step_renamed
  gate_selftest_case 'neither a live claim' plant_undeclared_citation
  gate_plant_clean_exempt_control
  CENSUS_CITATIONS=false
  GATE_SELFTEST_ARGS=()

  printf '%s selftest OK: passes a clean fixture, one with a ci.yml long enough to race, a compound gate, and a complete listing; fires on a listing missing a counted suite, on an empty one, and on an absent tests/ tree, a renamed gate spelling, one file re-gated onto a misspelt feature, a gate line replaced by a prose mention, a clippy row that stopped denying warnings, and the cfg lint silenced at the site — and in --citations mode, on a dropped citation, a deleted citing file, a renamed CI step, and an undeclared new citation, while PASSING the same citation in a declared-history file\n' "$(gate_name)"
}

# The negative control for the completeness check: the same planted
# citation, in a path CITATION_EXEMPT declares as history, must PASS.
# Without it the check above is satisfied by a matcher that fires on
# every file naming the step.
gate_plant_clean_exempt_control() {
  local tmp out
  tmp=$(mktemp -d)
  gate_plant_clean "$tmp"
  plant_exempt_citation "$tmp"
  if ! out=$(cd "$tmp" && gate 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: the citation completeness check fired on a CITATION_EXEMPT path\n%s\n' "$out" >&2
    exit 1
  fi
  rm -rf "$tmp"
}

gate_main

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
# counted suite of CRATE is in it) | --executed (the executed-set roster,
# `<mode><TAB><crate><TAB><module><TAB><floor>`) | --check-executed (read
# the sweep's `<mode><TAB><crate><TAB><module><TAB><passed>` tally on
# stdin and assert every rostered row RAN) | --root DIR | no args
# (report).
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

GATE_SCAN_NOUN="probe-gated test suite"

# `<crate>:<suites>` — the crates whose probe coverage was argued for,
# and the suite count each had when it was. Not the type-check loop's
# crate list: that is derived. This is the floor beneath it.
CENSUS_FLOOR=(editor-core:2 geom-brep:4 geom-core:1 profile:4 sweep:1 topo:5)

# THE EXECUTED SET, AND WHY IT NEEDS A FLOOR OF ITS OWN. `CENSUS_FLOOR`
# above is a floor on what COMPILES, and compiling is not running: a
# probe suite can be censused, listed, and still assert nothing on any
# merge. What nothing asserted was the SET OF CALLS the sweep makes.
#
# THIS FLOOR IS KEYED ON TESTS THAT RAN, NEVER ON REACHABILITY, and the
# distinction is the whole point. The obvious floor — every censused
# suite is named by some filter the sweep passes — is unsound here, and
# unsound in a way that reproduces the defect it would close:
# `k_probe_sweep.sh`'s `run_dump` passes `--ignored`, so a filter naming
# a suite whose tests carry no `#[ignore]` SELECTS it and runs NOTHING.
# Reachability would score such a suite covered while it is inert. So
# the input here is the test runner's own `N passed` line, one row per
# invocation, written by the sweep as it runs.
#
# THE MODE IS PART OF THE KEY for the same reason. `--ignored` and the
# default selection run DISJOINT halves of a suite, so a roster keyed on
# the module alone cannot tell "the dump ran" from "the pins ran", and
# the pins were the half that never ran. `ignored` and `plain` are the
# two selections the sweep makes; a suite may appear under either or
# both.
#
# `<mode>:<crate>:<module>:<tests>` — the probe suites CI executes, and
# the number of tests each ran under that selection when its execution
# was argued for.
RUN_FLOOR=(
  ignored:editor-core:m4_pr8_k_probe:1
  ignored:sweep:k_report:1
  plain:editor-core:m4_pr8_k_probe:1
  plain:editor-core:m5_pr5_corpus_probe:1
)

# EVERY CENSUSED SUITE DECLARES WHICH SIDE IT IS ON. What the executed
# set was missing was never only a floor: a suite's disposition was
# decided by a filter nobody read, and both dispositions are defensible
# while being decided by accident is not. So a censused suite is either
# in `RUN_FLOOR` above or says this sentence in its own header, and never
# both — a new probe suite has to pick a side, and a reader learns which
# from the file rather than from an invocation two directories away.
NOT_RUN_MARKER='CI COMPILES THIS SUITE AND DOES NOT RUN IT'

# The sweep's own wiring. Without it the floor above is removable by
# deleting one line from a script no gate reads — which is the shape of
# every hole this file already guards.
SWEEP_SCRIPT=scripts/k_probe_sweep.sh
SWEEP_CHECK_RE='probe-suite-census\.sh[^|]*--check-executed'

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
CENSUS_EXECUTED=false
CENSUS_CHECK_EXECUTED=false
CENSUS_LISTING=
gate_args=()
want_crate=false
for a in "$@"; do
  if [ "$want_crate" = true ]; then CENSUS_LISTING=$a; want_crate=false; continue; fi
  case "$a" in
    --crates) CENSUS_CRATES=true ;;
    --citations) CENSUS_CITATIONS=true ;;
    --suites) CENSUS_SUITES=true ;;
    --executed) CENSUS_EXECUTED=true ;;
    --check-executed) CENSUS_CHECK_EXECUTED=true ;;
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

# THE EXECUTED HALF, sited on the sweep's own output. Its input is the
# `N passed` line `cargo test` prints for each invocation
# `scripts/k_probe_sweep.sh` makes, tallied as
# `<mode><TAB><crate><TAB><module><TAB><passed>`. A count of tests that
# RAN is the only reading that can tell a covered suite from a selected
# and inert one, which is the case a reachability floor gets wrong.
census_check_executed() {
  local rc=0 tally bad entry mode crate module want rows have row_crate row_module row_mode
  tally=$(cat)
  if [ -z "${tally//[[:space:]]/}" ]; then
    gate_error "$(gate_name): the executed-set tally is EMPTY — the sweep recorded no invocation at all. A sweep that ran nothing produces exactly this, so it is a failure and not a clean run"
    exit 1
  fi
  bad=$(printf '%s\n' "$tally" | awk -F'\t' 'NF && (NF != 4 || $4 !~ /^[0-9]+$/) { printf "  line %d: %s\n", NR, $0 }')
  if [ -n "$bad" ]; then
    gate_error "$(gate_name): the executed-set tally has unreadable rows; the shape is \`<mode><TAB><crate><TAB><module><TAB><passed>\`"
    printf '%s\n' "$bad" >&2
    exit 1
  fi

  for entry in "${RUN_FLOOR[@]}"; do
    mode=${entry%%:*}; entry=${entry#*:}
    crate=${entry%%:*}; entry=${entry#*:}
    module=${entry%%:*}; want=${entry##*:}
    rows=$(printf '%s\n' "$tally" | awk -F'\t' -v m="$mode" -v c="$crate" -v s="$module" 'NF==4 && $1==m && $2==c && $3==s' | wc -l | tr -d ' ')
    if [ "$rows" -eq 0 ]; then
      gate_error "$(gate_name): the sweep never invoked $crate's \`$module\` under the \`$mode\` selection, so nothing in it ran. The roster in RUN_FLOOR is the executed set; a call dropped from $SWEEP_SCRIPT reads exactly like this. Restore the call, or drop the row from RUN_FLOOR and give the suite its \`$NOT_RUN_MARKER\` sentence"
      rc=1
      continue
    fi
    have=$(printf '%s\n' "$tally" | awk -F'\t' -v m="$mode" -v c="$crate" -v s="$module" 'NF==4 && $1==m && $2==c && $3==s { if ($4+0 > x) x = $4+0 } END { print x+0 }')
    if [ "$have" -lt "$want" ]; then
      gate_error "$(gate_name): $crate's \`$module\` was SELECTED under the \`$mode\` selection and executed $have test(s), below the $want its execution was argued for. Selection is not execution: a \`--ignored\` filter over a suite whose tests carry no \`#[ignore]\` matches the module and runs none of it, and a floor that read reachability would have called this covered"
      rc=1
    fi
  done

  # THE OTHER DIRECTION, so the roster is a roster and not a sample. A
  # suite CI executes but does not roster can be dropped again silently,
  # and it also carries the not-run sentence, which is then false.
  while IFS=$'\t' read -r row_mode row_crate row_module _; do
    [ -n "$row_crate" ] || continue
    for entry in "${RUN_FLOOR[@]}"; do
      [ "${entry%:*}" = "$row_mode:$row_crate:$row_module" ] && continue 2
    done
    gate_error "$(gate_name): the sweep executed $row_crate's \`$row_module\` under the \`$row_mode\` selection, and RUN_FLOOR does not roster it. An execution nobody rostered is one nobody will notice the loss of — add it with the count it runs today"
    rc=1
  done < <(printf '%s\n' "$tally" | awk -F'\t' 'NF==4' | sort -u)

  [ "$rc" -eq 0 ] || exit 1
  GATE_SCAN_FILES=${#RUN_FLOOR[@]}
  GATE_SCAN_NOUN='rostered probe-suite execution'
  gate_ok "every rostered probe-suite execution ran at or above its floor, counted from the test runner's own passed line rather than from what a filter could have matched"
}

gate() {
  local files tally rc=0 entry want n have silenced hosted
  local suite crate rest listed missing suites
  local rostered dcrate dmod marked rmod

  if [ "$CENSUS_CITATIONS" = true ]; then
    census_citations
    return 0
  fi

  if [ "$CENSUS_CHECK_EXECUTED" = true ]; then
    census_check_executed
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

  if [ "$CENSUS_EXECUTED" = true ]; then
    printf '%s\n' "${RUN_FLOOR[@]}" | tr ':' '\t'
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

  # THE DISPOSITION HALF. A censused suite is EXECUTED (rostered in
  # RUN_FLOOR, floored by --check-executed) or DECLARED UNRUN in its own
  # header, and never both. The row this closes is not "a suite went
  # unrun"; it is "a suite's disposition was decided by a filter nobody
  # read", and the only durable answer to that is that the file says
  # which side it is on. It is checked here rather than left to prose
  # because a new probe suite otherwise inherits the unrun side in
  # silence, which is the state this gate exists to end.
  rostered=$(printf '%s\n' "${RUN_FLOOR[@]}" | awk -F: '{ print $2"/"$3 }' | sort -u)
  while IFS= read -r suite; do
    [ -n "$suite" ] || continue
    dcrate=${suite#crates/}; dcrate=${dcrate%%/*}
    rest=${suite#crates/*/tests/}; dmod=${rest%.rs}
    marked=false
    grep -qF "$NOT_RUN_MARKER" "$suite" && marked=true
    if grep -qxF "$dcrate/$dmod" <<<"$rostered"; then
      if [ "$marked" = true ]; then
        gate_error "$(gate_name): $suite is rostered in RUN_FLOOR as a suite CI EXECUTES and its header also says \`$NOT_RUN_MARKER\`. One of the two is false; decide which"
        rc=1
      fi
    elif [ "$marked" = false ]; then
      gate_error "$(gate_name): $suite is a probe-gated suite that nothing in $SWEEP_SCRIPT runs, and its header does not say so. Either roster it in RUN_FLOOR and have the sweep run it, or write \`$NOT_RUN_MARKER\` into its header with the reason. A disposition decided by a filter two directories away is how this suite's siblings came to be compiled-and-inert without anyone choosing it"
      rc=1
    fi
  done <<<"$files"

  # The roster names files, so it can name one that is gone or is not a
  # probe suite at all — in which case --check-executed is floored on a
  # module that cannot exist.
  while IFS= read -r rmod; do
    [ -n "$rmod" ] || continue
    printf '%s\n' "$files" | grep -qxF "crates/${rmod%%/*}/tests/${rmod#*/}.rs" ||
      { gate_error "$(gate_name): RUN_FLOOR rosters \`${rmod#*/}\` in ${rmod%%/*} as an executed probe suite, but no such probe-gated file is censused under $PWD"; rc=1; }
  done <<<"$rostered"

  # THE WIRING. The floor above is enforced by a call in the sweep, and
  # a call is deletable. `gate-roster.sh` keeps this gate in both halves
  # of CI; nothing else keeps the sweep pointed at it.
  if [ -f "$SWEEP_SCRIPT" ]; then
    grep -qE "$SWEEP_CHECK_RE" "$SWEEP_SCRIPT" ||
      { gate_error "$(gate_name): $SWEEP_SCRIPT no longer feeds its executed-set tally to \`$(gate_name).sh --check-executed\`. That call is the only thing that turns a sweep which selected everything and ran nothing into a failure; restore it, or re-argue the floor here"; rc=1; }
  else
    gate_error "$(gate_name): $SWEEP_SCRIPT does not exist under $PWD — the executed-set floor has no producer"
    rc=1
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
  gate_ok "$(printf '%s\n' "$tally" | wc -l | tr -d ' ') crates own probe-gated suites, all at or above their floor, every one of them either rostered as executed or declaring in its own header that it is not, the sweep still feeding its tally to --check-executed, and the clippy row that reports a misspelt gate wired (the citation half runs in the \`mirror\` job, where it can fire on prose)"
  printf '%s\n' "$tally" | while read -r c n; do printf '  %-14s %s suite(s)\n' "$c" "$n"; done
}

# The fixture is a miniature repo: crate test trees, the four citing
# files, and a ci.yml carrying both rows this gate reads.
gate_plant_clean() {
  local t=$1 entry c n i m
  for entry in "${CENSUS_FLOOR[@]}"; do
    c=${entry%%:*}; n=${entry##*:}
    mkdir -p "$t/crates/$c/tests"
    for ((i = 0; i < n; i++)); do
      # Unrostered, so each declares its own disposition.
      printf '//! %s\n#![cfg(feature = "probe")]\n' "$NOT_RUN_MARKER" \
        > "$t/crates/$c/tests/probe_$i.rs"
    done
  done
  # The rostered suites, which must NOT carry the sentence.
  for entry in "${RUN_FLOOR[@]}"; do
    c=$(printf '%s' "$entry" | cut -d: -f2); m=$(printf '%s' "$entry" | cut -d: -f3)
    mkdir -p "$t/crates/$c/tests"
    printf '#![cfg(feature = "probe")]\n' > "$t/crates/$c/tests/$m.rs"
  done
  mkdir -p "$t/scripts/gates"
  printf '#!/usr/bin/env bash\nscripts/gates/probe-suite-census.sh --check-executed < "$ran"\n' \
    > "$t/$SWEEP_SCRIPT"
  mkdir -p "$t/crates/plain/tests"
  printf '// mentions feature = "probe" in prose only\n' > "$t/crates/plain/tests/all.rs"
  mkdir -p "$t/.github/workflows"
  {
    printf 'jobs:\n  clippy:\n    steps:\n'
    printf '      - run: cargo clippy ${{ needs.filter.outputs.cargo_scope }} --all-targets -- -D warnings\n'
    printf '  k-lint:\n    steps:\n      - name: %s\n' "$CITED_STEP"
  } > "$t/.github/workflows/ci.yml"
  # APPENDED, not written: two of the citing files are probe suites, and
  # overwriting them would strip the cfg gate the census counts.
  for entry in "${CITING_FILES[@]}"; do
    mkdir -p "$t/$(dirname "$entry")"
    printf 'cites CI %s\n' "$CITED_STEP" >> "$t/$entry"
  done
}

plant_no_tests_dirs() { rm -rf "$1"/crates/*/tests; }
# The gate spelling changed under every file at once.
plant_gate_renamed() { sed -i 's/"probe"/"probe2"/' "$1"/crates/*/tests/*.rs; }
# ONE file re-gated onto an always-off feature: the per-crate floor is
# what turns 5 -> 4 into a failure.
plant_one_file_misgated() { sed -i 's/"probe"/"prboe"/' "$1/crates/topo/tests/probe_0.rs"; }
# The gate line deleted, the file's PROSE mention of the feature left —
# the case the substring predicate could not tell from a real gate.
plant_prose_only() {
  printf '// this file is #![cfg(feature = "probe")] in spirit\n' \
    > "$1/crates/geom-brep/tests/probe_0.rs"
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
# A NEW probe suite with no declared disposition: the state every unrun
# suite was already in, and the one this gate ends.
plant_disposition_undeclared() {
  printf '#![cfg(feature = "probe")]\n' > "$1/crates/topo/tests/probe_new_suite.rs"
}
# The sentence on a suite the sweep DOES run: the claim is then false in
# the other direction, and the file is the thing a reader believes.
plant_disposition_both() {
  local c m
  c=$(printf '%s' "${RUN_FLOOR[0]}" | cut -d: -f2); m=$(printf '%s' "${RUN_FLOOR[0]}" | cut -d: -f3)
  printf '//! %s\n#![cfg(feature = "probe")]\n' "$NOT_RUN_MARKER" > "$1/crates/$c/tests/$m.rs"
}
# The roster naming a suite the census cannot see.
plant_roster_orphan() {
  local c m
  c=$(printf '%s' "${RUN_FLOOR[0]}" | cut -d: -f2); m=$(printf '%s' "${RUN_FLOOR[0]}" | cut -d: -f3)
  rm -f "$1/crates/$c/tests/$m.rs"
}
# The one line that connects the floor to its producer, deleted.
plant_sweep_unwired() { printf '#!/usr/bin/env bash\necho sweeping\n' > "$1/$SWEEP_SCRIPT"; }
plant_step_renamed() { printf 'jobs: {}\n' > "$1/.github/workflows/ci.yml"; }
# The clippy row loses the flag that promotes `unexpected_cfgs`.
plant_clippy_undenied() { sed -i 's/ -- -D warnings//' "$1/.github/workflows/ci.yml"; }
# The lint silenced at the site instead.
plant_cfg_lint_allowed() {
  printf '//! %s\n#![allow(unexpected_cfgs)]\n#![cfg(feature = "probe")]\n' "$NOT_RUN_MARKER" \
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
  printf '//! %s\n#![cfg(all(feature = "probe", not(miri)))]\n' "$NOT_RUN_MARKER" \
    > "$tmp/crates/sweep/tests/probe_0.rs"
  if ! out=$(cd "$tmp" && gate 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: a compound `probe` gate was not counted as a suite\n%s\n' "$out" >&2
    exit 1
  fi
  rm -rf "$tmp"
}

# THE EXECUTED HALF, BOTH DIRECTIONS. Its subject is a tally, so its
# fixture is text: no cargo, milliseconds. The case that matters is the
# third — a suite SELECTED by a filter that ran none of it, which is the
# state `editor-core`'s corpus replay was in and which a floor keyed on
# reachability scores as covered.
gate_tally_clean() {
  local entry mode c m n
  for entry in "${RUN_FLOOR[@]}"; do
    IFS=: read -r mode c m n <<<"$entry"
    printf '%s\t%s\t%s\t%s\n' "$mode" "$c" "$m" "$n"
  done
}

selftest_executed() {
  local tmp out tally
  tmp=$(mktemp -d)
  gate_plant_clean "$tmp"
  tally=$(gate_tally_clean)

  if ! out=$(cd "$tmp" && printf '%s\n' "$tally" | CENSUS_CHECK_EXECUTED=true gate 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: the executed-set floor FAILED on a tally meeting every row\n%s\n' "$out" >&2
    exit 1
  fi

  # SELECTED AND INERT. The invocation happened, matched the module, and
  # executed nothing — a `--ignored` filter over a suite with no
  # `#[ignore]`. Reachability cannot see this; a passed count can.
  if out=$(cd "$tmp" && printf '%s\n' "$tally" | sed "1s/\t[0-9]*$/\t0/" |
             CENSUS_CHECK_EXECUTED=true gate 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: the executed-set floor PASSED a suite that was selected and executed 0 tests\n%s\n' "$out" >&2
    exit 1
  fi
  case "$out" in
    *'executed 0 test(s)'*) ;;
    *) rm -rf "$tmp"
       printf 'SELFTEST FAILED: the selected-and-inert case fired with an unexpected message:\n%s\n' "$out" >&2
       exit 1 ;;
  esac

  # THE CALL DROPPED ALTOGETHER.
  if out=$(cd "$tmp" && printf '%s\n' "$tally" | tail -n +2 |
             CENSUS_CHECK_EXECUTED=true gate 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: the executed-set floor PASSED with a rostered invocation missing\n%s\n' "$out" >&2
    exit 1
  fi
  case "$out" in
    *'never invoked'*) ;;
    *) rm -rf "$tmp"
       printf 'SELFTEST FAILED: the dropped-call case fired with an unexpected message:\n%s\n' "$out" >&2
       exit 1 ;;
  esac

  # A SWEEP THAT RECORDED NOTHING AT ALL looks exactly like a clean one
  # to anything that only reads rows.
  if out=$(cd "$tmp" && printf '' | CENSUS_CHECK_EXECUTED=true gate 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: the executed-set floor PASSED on an EMPTY tally\n%s\n' "$out" >&2
    exit 1
  fi

  # AN EXECUTION NOBODY ROSTERED, which is how the roster would go stale.
  if out=$(cd "$tmp" && { printf '%s\n' "$tally"; printf 'plain\ttopo\tprobe_0\t3\n'; } |
             CENSUS_CHECK_EXECUTED=true gate 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: the executed-set floor PASSED an execution absent from RUN_FLOOR\n%s\n' "$out" >&2
    exit 1
  fi
  case "$out" in
    *'does not roster it'*) ;;
    *) rm -rf "$tmp"
       printf 'SELFTEST FAILED: the unrostered-execution case fired with an unexpected message:\n%s\n' "$out" >&2
       exit 1 ;;
  esac

  # A TALLY THE SWEEP COULD NOT HAVE WRITTEN.
  if out=$(cd "$tmp" && printf 'plain editor-core m4_pr8_k_probe 1\n' |
             CENSUS_CHECK_EXECUTED=true gate 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: the executed-set floor PASSED a malformed tally\n%s\n' "$out" >&2
    exit 1
  fi
  rm -rf "$tmp"
}

gate_selftest() {
  gate_selftest_clean
  selftest_hosted_half_is_large
  selftest_listing
  selftest_executed
  selftest_compound_counted
  gate_selftest_case 'scanned nothing' plant_no_tests_dirs
  gate_selftest_case 'no longer matches it' plant_gate_renamed
  gate_selftest_case 'topo carries 4 probe-gated test suite(s), below the 5' plant_one_file_misgated
  gate_selftest_case 'geom-brep carries 3 probe-gated test suite(s), below the 4' plant_prose_only
  gate_selftest_case 'no workspace `cargo clippy' plant_clippy_undenied
  gate_selftest_case 'silences `unexpected_cfgs`' plant_cfg_lint_allowed
  gate_selftest_case 'does not say so' plant_disposition_undeclared
  gate_selftest_case 'One of the two is false' plant_disposition_both
  gate_selftest_case 'no such probe-gated file is censused' plant_roster_orphan
  gate_selftest_case 'no longer feeds its executed-set tally' plant_sweep_unwired

  # THE CITATION HALF's cases. `gate` reads CENSUS_CITATIONS as a
  # global and lib.sh's harness runs it in a subshell, so setting it
  # here selects the half under test without a second harness.
  CENSUS_CITATIONS=true
  gate_selftest_clean
  gate_selftest_case 'no longer names CI' plant_citation_dropped
  gate_selftest_case 'is gone; it cited' plant_citing_file_gone
  gate_selftest_case 'has no step named' plant_step_renamed
  gate_selftest_case 'neither a live claim' plant_undeclared_citation
  gate_plant_clean_exempt_control
  CENSUS_CITATIONS=false

  printf '%s selftest OK: passes a clean fixture, one with a ci.yml long enough to race, a compound gate, a complete listing, and a tally meeting every rostered execution; fires on a listing missing a counted suite, on an empty one, and on an absent tests/ tree, a renamed gate spelling, one file re-gated onto a misspelt feature, a gate line replaced by a prose mention, a clippy row that stopped denying warnings, the cfg lint silenced at the site, a suite with no declared disposition, a rostered suite claiming it is not run, a roster row naming no censused file, and a sweep that stopped feeding --check-executed — and in --check-executed mode, on a suite SELECTED that executed nothing, a dropped invocation, an empty tally, an unrostered execution and a malformed row; and in --citations mode, on a dropped citation, a deleted citing file, a renamed CI step, and an undeclared new citation, while PASSING the same citation in a declared-history file\n' "$(gate_name)"
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

---
id: whole-file-skips-do-not-check-their-subject
kind: issue
title: the converted whole-file skips still do not check that the files they exempt exist
status: review
branch: gates/skip-subject-check
pr: 2156
opened: 2026-09-06
refs: [whole-file-skips-are-hand-spelled-not-anchored]
---

## Finding

`whole-file-skips-are-hand-spelled-not-anchored` closed the SMALLER of
the two halves it measured: every whole-file skip in this directory now
builds its pattern with `gate_record_anchor`, so the `FILE:LINE:` shape
is pinned and the escaping is nobody's job. The other half it measured
is untouched, and it is the half with a live route.

**None of the seven gates checks that the files it exempts still
exist.** Each declares its homes as an array (or, for a single home, a
`HOME_FILE`), builds the filter from it with `gate_record_anchor_any`
and plants every entry in the clean fixture — but nothing asserts the
paths are in the tree:

  * `scripts/gates/bit-identity-consumer.sh` — `NON_CONSUMER_HOMES`, 4
  * `scripts/gates/bit-identity-punning.sh` — `HOME_FILE`, 1
  * `scripts/gates/evalscalar-allowlist.sh` — `SEAM_HOMES`, 2
  * `scripts/gates/interval-square-allowlist.sh` — `ALLOWLISTED_HOMES`, 7
  * `scripts/gates/no-ambient-env.sh` — `ALLOWLISTED_HOMES`, 3
  * `scripts/gates/register-equal-allowlist.sh` — `DEFINITION_HOMES`, 5
    and `CALLER_HOMES`, 3
  * `scripts/gates/witness-not-ambient.sh` — `HOME_FILE`, 1

`grep -c gate_require` returns 1 for each, and each of those is
`gate_require_crate_sources` — which proves the CRATE has sources, not
that the skipped paths are among them.

**It is also the unproved half of the "one list" claim** those arrays
now carry. The fixture->filter direction reds: a home the clean fixture
plants and the filter stops covering fails the clean case. The
filter->fixture direction does not — planting only some of the homes
while keeping the full filter is green on every gate tried — because
nothing reads the filter's list back against the tree. A subject check
is what closes it, in both senses at once.

## Why it is worth a row

`lib.sh` argues it at `gate_exact_skip_subject` and
`gate_require_file`, and this directory already reds on the class
twice: **a skip whose home is renamed or deleted exempts nothing, stays
green, and ratifies whatever lands at that path next.** A rename is
ordinary; the colon-in-a-path the conversion closed is not. That is the
direction with the population.

`signed-zero-one-home.sh` is the shape to copy: `gate_require_file
"$HOME_FILE"` before the scan, with a fixture that removes the home and
asserts the diagnosis. Over a list it is one loop — and it likely
belongs beside `gate_record_anchor_any` in `lib.sh`, since every caller
of that builder wants exactly this check over exactly that argument.
The clean fixture that plants every home (landed) is already what makes
such a check pass in the self-test.

## Landed

**One check in `lib.sh`, seven callers.** `gate_require_homes SUBJECT
HOME...` (beside `gate_record_anchor_any`, which is where every caller
of the builder gets its list from) proves each home is a file AND a file
the gate's scan set contains, refusing with a diagnosis that names the
path and what the skip anchored there would have exempted.
SUBJECT is the caller's own words for the exemption, held beside the
list as `NON_CONSUMER_SUBJECT`, `SEAM_SUBJECT`, `ALLOWLISTED_SUBJECT`,
`CALLER_SUBJECT`/`DEFINITION_SUBJECT`, `HOME_SUBJECT` — the half a bare
`[ -f ]` cannot supply, since the path alone says a file is missing and
what a reader has to decide is whether the exemption moved with it or
died with it. Why a missing home is a RED and not an abstention is
`gate_exact_skip_subject`'s argument, pointed at rather than re-made.

**It is a call of its own, not a check inside `gate_record_anchor_any`,**
though the builder's every caller wants exactly this over exactly that
argument. The builder is read inside `gate_grep -vE "$(…)"`, so an
`exit` in it is the substitution's status and the expansion discards it
— the gate would print the diagnosis and then filter on the empty
pattern the refusal left behind, which drops every record and goes
green over a scan it never read; that is
`gate_exact_skip_pattern_for`'s header, one mechanism over. It is also
not `gate_require_file`, whose subject is the SCAN target and which
records `GATE_SCAN_FILES=1`.

Each of the seven calls it after the file set is decided and before the
matcher: after `gate_require_crate_sources` in the five that scan
`crates/*/src` whole, after `gate_production_sources` in the two that
narrow first (`witness-not-ambient`, `interval-square-allowlist`) — a
tree with no production source has no exemption to answer for, and the
guard above names the larger failure.

**The cases are `lib.sh`'s too**, for the reason the anchored exact-text
skip's are: `gate_selftest_homes HOME...` runs `gate_plant_home_gone`
once per home, each wanting the missing path BY NAME. One run per home
rather than one with the list emptied — the refusal is terminal at the
first missing path, so a single case proves only that the first entry
is reached, and a home is renamed one at a time. 26 homes across the
seven (4, 1, 2, 7, 3, 5+3, 1).

**Mutation table.**

| mutation | result |
| --- | --- |
| `gate_require_homes` backed out to `return 0` | all seven selftests red at `gate_plant_home_gone` ("the gate PASSED on a planted violation") |
| check in place, one home removed from the clean tree | each gate reds naming that path, per home |
| **filter→fixture**: a home kept in the filter, dropped from `gate_plant_clean` (`bit-identity-consumer`'s `geom-core/src/interval.rs`; `witness-not-ambient`'s `HOME_FILE`) | reds "the gate FAILED on a clean fixture", through this check's diagnosis |
| the same fixture mutation with the whole unit backed out | **green** — which is the direction PR 2077 left unproved, now closed |

**The builder's own refusal, from PR 2157's review.**
`gate_record_anchor_any`'s no-homes refusal (`lib.sh`) was diagnosed but
not terminal: every caller reads the builder inside
`gate_grep -vE "$(…)"`, so the `exit 1` is the substitution's, the
expansion discards it, `grep -vE ''` drops every record and the gate
prints `OK` with status 0. Fixed the way `gate_exact_skip_record_for`
does — the refusal writes the `GATE_MATCHER_FAILED` marker, which
`gate_ok` refuses to print over — with the comment corrected from
"diagnosed and terminal" to what it is. `gate_require_homes`'s two
refusals write the marker in the same shape, though their `exit` is
already terminal, so the two refusals over one list read alike.
`gate_empty_home_list_case` plants it in `lib.sh`: a scratch gate whose
home list is empty, in a real subprocess, in the callers' own spelling.
Mutation: drop the marker line and the case reds with the finding
reproduced verbatim — the diagnosis on stderr, then
`empty-homes OK: nothing matched (1 source file scanned)`.

**Live output is byte-identical** for all seven gates, stdout and
stderr `cmp`'d against the merge base (`dfa569e01`): every home is in
the tree, so the check prints nothing. `gate-roster.sh` green; every
`scripts/gates/*.sh --selftest` and live pass green, `gated-suite-paths`
included.

The "ONE LIST, AND ONE DIRECTION PROVED" paragraph each of the seven
carried pointed at this row as the residue; it now reads BOTH
DIRECTIONS PROVED and names the check. (The `.#` run-together where PR
2077 spliced that paragraph onto the list's own comment is gone with
it.)

## Fix pass (style review of PR 2156)

Twelve items, one commit. What moved:

* **The failure line names the case.** `gate_selftest_case` captured the
  planter NAME, so seven runs of one parameterised planter reported the
  same word seven times; it captures the planter AND its arguments now,
  and names the gate — `SELFTEST FAILED: bit-identity-consumer PASSED on
  a planted violation (gate_plant_home_gone crates/topo/src/source.rs)`.
  Every parameterised planter in the file gains from it, not just this
  unit's.
* **One text per refusal, two guards each.** `gate_home_gone_refusal`
  and `gate_no_homes_refusal` hold the two texts; the exact-text skip's
  subject check, the whole-file skip's, the builder's empty-list guard
  and the check's all call one of them. The "why a red and not an
  abstention" paragraph lives at the first and is pointed at from the
  three sites that used to re-argue or re-spell it.
* **One home for the substitution argument.** §"A refusal a substitution
  would swallow" states it once — the two routes that make such a
  refusal bite, capture-in-a-statement and the marker — and the four
  sites that told it again are one-line pointers.
* **`gate_require_homes`'s header** is the invariant, the
  why-not-in-the-builder sentence, the ordering rule and the scan-set
  half; the provenance sentence and the paragraph defending a redundant
  marker line are gone (the provenance is the PR body's).
* **The seven array paragraphs and the two ordering comments** are one
  line each, pointing at the check.
* **`interval-square-allowlist.sh`'s SUBJECT** no longer says "the
  header above": a diagnosis is read in a CI log, not beside the script.

**The check proves membership in the scan set, not just `[ -f ]`.** The
review's finding: a home that exists but is not scanned exempts nothing
exactly as a missing one does, and three shapes reach it — a path
outside `crates/*/src`, a home a `#[cfg(test)] mod` declaration mounts
out of the production set, and a SYMLINK, which `[ -f ]` follows and
`find -type f` does not. `gate_require_homes` now reads the set the gate
just decided (`GATE_PRODUCTION_FILES` when the narrowing ran, else
`GATE_SOURCE_FILES`), with its own diagnosis; the `[ -f ]` runs first
only so a home that is simply gone gets the diagnosis about being gone.

`gate_plant_home_unscanned` plants it — a `#[cfg(test)] mod NAME;` in
the home's own directory's `mod.rs`, which the rustc rule in §"WHERE A
TEST-ONLY MODULE LIVES" mounts as a sibling. The same fixture points
both ways, so every gate carries a case from it:
`gate_selftest_homes --narrowed` (the two gates that call
`gate_production_sources`) asserts a RED naming the home; the other five
assert a PASS, because they scan every source and the skip still covers
it.

**Mutation table, re-run after the refactor.**

| mutation | result |
| --- | --- |
| `gate_require_homes` → `return 0` | all seven red at `gate_plant_home_gone`, each naming its own gate and home |
| the loop reads only `${1}` | the five multi-home gates red; the two single-home gates unaffected, which is the shape that says the case is per home |
| `if [ "$scanned" = false ]` → `if false` | both narrowing gates red at `gate_plant_home_unscanned` |
| the marker dropped from `gate_no_homes_refusal` | `gate_empty_home_list_case` reds over `empty-homes OK: nothing matched` |
| **filter→fixture**, re-run | both gates red "the gate FAILED on a clean fixture" through the shared refusal |

Live output still byte-identical for all seven, stdout and stderr —
measured the way a merge makes necessary: `origin/main`'s copies of the
gates extracted and pointed at THIS tree with `--root`, so the only
variable is the gate code. Compared against the pre-merge capture
instead, all seven "differ" in their scanned-file count alone (440 ->
443, 403 -> 406), which is PR 2157's three new sources arriving in the
tree and not a gate deciding anything differently. All 21 gates
`--selftest` and live green after the merge; `scripts/work.py lint` ok.
The whole `--selftest` sweep is ~87 s against ~90 s before the fix pass
— the added cases are inside run-to-run noise.

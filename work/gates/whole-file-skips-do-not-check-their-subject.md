---
id: whole-file-skips-do-not-check-their-subject
kind: issue
title: the converted whole-file skips still do not check that the files they exempt exist
status: review
branch: gates/skip-subject-check
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
of the builder gets its list from) refuses with a diagnosis naming the
missing path and what the skip anchored there would have exempted.
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

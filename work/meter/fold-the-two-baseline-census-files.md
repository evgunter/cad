---
id: fold-the-two-baseline-census-files
kind: issue
title: the two baseline census files should be one
status: closed
opened: 2026-09-07
closed: 2026-09-08
branch: meter/10-fold-baseline-censuses
refs: [baseline-sizing-census-pointers-stale]
---



## What

`tools/tess-lint/tests/baseline_census.rs` and
`tools/tess-lint/tests/baseline_sizing_census.rs` are two test files
over one artefact
(`docs/tess-budget-data/tess-budget-baseline.csv`), reached the same
way, failing on the same trigger, and they should be one.

The evidence, from the METER unit 2 style review:

- **A self-declared duplicate at the copy site.**
  `baseline_sizing_census.rs` says of the `include_str!` path that it
  is *"the one thing that does appear in both, for want of a shared
  `tests/` module"*. That is the Q1 tell verbatim, written by the
  author of the second file.
- **Near-identical prose.** Both headers open on the one-home rule,
  both explain that no baseline is a target to preserve, both explain
  the re-cut recourse, and both point at the other.
- **One trigger.** A re-cut of the baseline is the only event that
  reds either.
- **The stated reason not to fold does not survive.** The second file
  said folding *"would put one file's failure under two unrelated
  headings"*. A `cargo test` failure is named by its `#[test]`, not by
  its file; the three test functions already carry their own headings
  and would keep them.

## Proposed shape

One `tools/tess-lint/tests/baseline.rs`: one `BASELINE` const behind
one `include_str!`, the three test functions merged in as they stand
(`the_committed_baseline_carries_this_many_indistinguishable_pairs`,
`six_of_the_eight_identity_entries_discriminate_nothing_among_the_sized_rows`,
`the_committed_baseline_sizes_this_much`), and one module header
carrying the two censuses' scopes as two sections rather than two
files. The no-figure-asserted-twice split between them is preserved by
construction — it is a property of which function asserts what, not of
which file it lives in.

## Why not in the unit that found it

METER unit 2's fence was `docs/TESS-BUDGET.md`,
`tools/tess-lint/tests/baseline_sizing_census.rs` and `work/meter/`;
`baseline_census.rs` was held by a concurrent lane for the whole of
it, and a fold touches both files or neither. **This lands after both
PRs are merged**, on a tree where neither file is held.

## Was

disclosed by METER unit 2's style review (finding S7, `sure`), filed
in the PR that carries the unit.

## Closed

One file, `tools/tess-lint/tests/baseline_census.rs`, holding all
seven tests over one `BASELINE` const behind one `include_str!`;
`baseline_sizing_census.rs` deleted.

**Not the name this item proposed, and the reason is the pointer
count.** `tests/baseline.rs` would have been a third name neither file
had. Twenty-four live mentions across fourteen files name
`baseline_census.rs` — `tools/tess-lint/src/lib.rs` at four,
`tests/cut_line_pin.rs`, `docs/TESS-BUDGET.md` at two and seventeen
`work/` mentions across eleven rows — against five that name
`baseline_sizing_census.rs`. Renaming both would have staled every one
of them to remove a duplication; folding into the majority name stales
five, one of which is this program's own plan line.
`work/meter/baseline-sizing-census-pointers-stale` carries them.

**Three claims above did not survive to the fold, and they are the
same defect this file is about — a description drifting from the
thing it describes.**

- *"the three test functions"* — there are seven. Six of the
  face-identity census (three of them added by the `C15` swap-cost
  unit after this row was filed) and one of the sizing census.
- *"`six_of_the_eight_identity_entries_…`"* — the function is
  `five_of_the_seven_identity_entries_discriminate_nothing_among_the_sized_rows`.
  `IDENTITY_COLUMNS` lost an entry, and this row's copy of the name
  did not follow.
- *"both point at the other"* — only `baseline_sizing_census.rs`
  pointed. `baseline_census.rs` named no sibling at any of its 664
  lines.

**What was verified rather than assumed.**

- *A `cargo test` failure is named by its `#[test]`, not by its
  file.* Executed: three literals mutated at once in the merged file
  gives `failures:` listing three distinct test names out of one
  binary, with the file appearing only as the binary name in the
  rerun hint and as a source location in the backtrace. The second
  file's stated reason not to fold — that folding *"would put one
  file's failure under two unrelated headings"* — is exactly backwards:
  the headings are the test names and folding does not touch them.
- *No figure is asserted twice.* Checked mechanically over the merged
  file rather than argued from the split's shape: every numeric
  literal inside an `assert*!` extracted per test function and
  compared across functions. The two censuses' figures are disjoint —
  1353 / 64 / 12 / 7 / 14 / 22,352 against 1,552,822 / 164,710 /
  46,019 / 110,811 / 93,066 / 44,162 / 2.408 / 1.0420. Three
  incidental collisions, none a census figure: `0` from two
  `[] as [&str; 0]` empties, `4` from a *"rule 4"* inside an assert
  message, and `72`, which is two different predicates that agree on
  this corpus — *scenes carrying a corpus-wide indistinguishable
  pair* and *scenes in the baseline* — both already in
  `baseline_census.rs` before this fold and neither introduced by it.

**Test count: 7 before (6 + 1 across two binaries), 7 after (one
binary), each of the seven names read out of `cargo test`'s output on
both sides; crate total 80 before and after.**

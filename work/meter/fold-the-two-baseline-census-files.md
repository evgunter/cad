---
id: fold-the-two-baseline-census-files
kind: issue
title: the two baseline census files should be one
status: open
opened: 2026-09-07
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

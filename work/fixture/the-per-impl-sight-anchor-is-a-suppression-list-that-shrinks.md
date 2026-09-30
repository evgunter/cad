---
id: the-per-impl-sight-anchor-is-a-suppression-list-that-shrinks
kind: issue
title: the hand-written-impl census's only per-impl sight anchor is its suppression list, so it goes blind as the rows it names are repaired
status: open
opened: 2026-09-16
priority: P4
cost: E
---



Found by CENSUS-HAND-LISTED-SIBLINGS (2026-09-16), which caused it:
the unit repaired four of the five impls `KNOWN_HAND_LISTED` named and
deleted their entries, as that list's sibling row requires.

## The finding

`crates/test-utils/tests/hand_written_impl_census.rs` has two sight
rows. `the_walk_still_sees_every_file_holding_one` holds a SET OF
PATHS, so it catches a reader that goes blind to a whole file and
cannot catch one that loses some of a file's impls. The residue
paragraph on `IMPL_FILES_TODAY` acknowledged that and named
`every_known_hand_listed_impl_is_still_found` as what covers the worst
case — keyed by self type, so it reds wherever in its file the impl
sits.

**That was never per-impl sight.** It was the accident that **ONE of
the five** entries had a compliant sibling impl earlier in its file —
`coset.rs`, which holds `PartialEq for Subgroup` above `PartialEq for
Coset` — so a reader that stopped after a file's first match lost that
one entry's impl and the row red, naming it and nothing else. The
other four (`clearance.rs`, `expr.rs`, `program.rs`, `props.rs`) are
each their file's only `Debug`-or-`PartialEq` impl, so the mutation
never lost them and they contributed no sight at all.

**This row was filed saying FOUR of five**, and a style review ran the
mutation and read the failure message, which names exactly one entry.
The measurement table below was right and the sentence above it was
wrong — corrected 2026-09-16. The correction matters for whoever takes
this row: per-impl sight here was never a property of sibling ordering
across the list, it was one impl's worth, over one file, for the whole
of main's history — and `coset.rs` is one of the four the repairing
unit removed.

## Measured, both trees

A one-line mutation — `break` after the first `Site` pushed in
`sites_in`, i.e. a classifier that reports only each file's first
`Debug`/`PartialEq` impl:

| tree | `every_known_hand_listed_impl_is_still_found` | the other five rows |
| --- | --- | --- |
| `origin/main` at `a11b600df` (five entries) | **FAILED** | green |
| `census/hand-listed-siblings` (one entry) | **ok** | green |

So the mutation that the residue paragraph cited as caught is now
caught by nothing. A second mutation — `break` after the first `impl`
KEYWORD of each file, whether or not it matches — still reds, but it
reds `the_walk_still_sees_every_file_holding_one` too, so it is the
whole-file blindness that row already owns and not this one.

## Why it is this program's shape

The instrument gets weaker exactly as the tree gets better: every row
`KNOWN_HAND_LISTED` names that someone repairs removes a line from
what the census can still see, and the census says nothing when the
last one goes. A guard whose power is a side effect of the defects
still outstanding cannot go red once they are fixed.

## What a fix has to weigh

The file refuses a per-impl TALLY deliberately and argues it: *"this
census's subject is arrival and a compliant arrival should cost one
line, not a re-count"* — and that argument is sound, so the answer is
probably not a count. A per-file impl COUNT keyed like
`IMPL_FILES_TODAY` would cost a compliant arrival a number; a
per-(path, trait, self type) roster of every site the walk finds
would cost it a row. Whether the residue is worth either, or should
simply be stated and left, is the row. Nothing about it is urgent —
what is not acceptable is the paragraph claiming a cover it no longer
has, which this unit's diff corrects in the same breath as creating
the gap.

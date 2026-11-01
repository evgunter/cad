---
id: features-share-row-asserts-a-conjunct-its-neighbour-subsumes
kind: issue
title: The features-share row's is_finite conjunct is subsumed by the range beside it
status: open
opened: 2026-09-21
priority: P3
cost: E
---


## Finding

`crates/viewer/src/app.rs`, the `features_fraction` test row:

```rust
matches!(answer, Some(share)
    if share.is_finite() && (0.0..=FEATURES_SHARE_CAP).contains(&share)),
```

`RangeInclusive::contains` is `start <= x && x <= end`, and both
comparisons are false for `NaN` and for `+inf`. `-inf` fails the
lower bound. So the range already refuses everything
`share.is_finite()` refuses, and **no runtime value makes the
conjunct false while its neighbour is true**.

`docs/prompts/implementer-discipline.md` §2 settles the disposition:
*"Write assertions a bug could break. Name the runtime value that
would make one false; where there is none — a predicate over things
fixed at compile time, or one its neighbours already subsume — it is
documentation, and deleting it is the repair."* Delete the conjunct.
It is the whole change.

## Why it is filed rather than fixed

Found inside the fence of a different unit
(`chrome/datum-honesty`, `crates/viewer/src/datums.rs`) by the sweep
for hand-spelled finiteness predicates, and `app.rs` was outside it.
It is filed rather than mentioned because `work/README.md` and
`work/meta/a-stated-sweep-blind-spot-is-never-swept` both say so: the
site was named in that unit's PR as an instance of a stated blind
spot — a finiteness question asked by a range check rather than by
`is_finite` — and a named instance inside a blind-spot paragraph is
the exact shape that never gets swept.

Stakes are low and the row is cheap. It is here so the next lane to
open this file does not re-derive it.

## Fence

`crates/viewer/src/app.rs` — chrome's, and also view's and vseam's.

## Evidence: the shape recurred and was caught in review (2026-09-22)

`chrome/rowstatus-exhaustive` (PR 3055) added
`tree_badges::a_downstream_failure_alone_is_a_fault_the_reader_cannot_act_on`
with a second assertion —
`rows.iter().all(|row| row.status.tone() == Tone::Advisory)` — that
was character-for-character the expression already inside
`only_the_row_whose_own_operation_refused_is_actionable`, seven
hundred lines up the same file, which pins every `RowStatus`→`Tone`
pair over rows a real evaluation produced rather than hand-built ones.
No runtime value makes the new one false while the neighbour holds.
Deleted on that branch rather than filed, so this row gains a data
point and not a sibling: the subsumed-conjunct shape is not confined
to `app.rs`, and a hand-built fixture is where it turns up.

---
id: a-bundles-equations-are-read-twice
kind: issue
title: A bundle's equations are assembled twice (equations_of at the door, solve_bundle at the fold), and settle's unreachable! holds only by the rule-then-fold order
status: closed
opened: 2026-10-18
closed: 2026-10-18
priority: P2
cost: M
---

Found by the review of INTENT stage 3 F (PR #5299; reviewer style findings S3 and S5).

**Two readings of one bundle.** The door reads a bundle's equations through `equations_of` (`crates/editor-core/src/mate/solve.rs:826`), from each mate's `on` variable's kind. The fold reads them again in `solve_bundle` (`solve.rs:915`), this time from the target value's `symmetry().family()`, and re-runs each value's `admitted_by` check. Only `eval_pose`'s assert (`crates/editor-core/src/eval/wire/pose.rs:90`) keeps the two in step. That is the same "door and solve as two spellings" shape F set out to close.

**An invariant held by convention.** `settle`'s `Translations::None => unreachable!` (`crates/editor-core/src/mate/value.rs:322`) is unreachable only because `solve_bundle` asks the rule (`overconstrained`) before it folds (`fold_bundle`) and settles. Nothing in the types says so. The review found one state, since fixed in the same PR, where the rule and the fold disagreed.

**The work.** Build one `Equations` reading per bundle and give it to both the door and the fold, or type the admitted bundle so that `settle` takes only what the rule passed.

## Closed (2026-10-18)

A bundle's equations are read once, off its definitions (`equations_of`).
`fold_with` passes that reading to `solve_bundle`, so the door and the solve
decide the rule over one value. `solve_bundle` no longer re-derives the
families from the pose values' symmetry.

`settle`'s `Translations::None => unreachable!` is gone, along with the
`Translations` machinery (see `the-slide-charts-indeterminate-arm-is-unreachable`).
What `settle` relies on is now one stated premise, that every slide is on one
chart, and `no_admitted_slide_sits_beside_a_second_mate` pins it, and `settle` asserts it.

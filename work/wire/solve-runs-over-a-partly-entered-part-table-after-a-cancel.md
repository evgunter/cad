---
id: solve-runs-over-a-partly-entered-part-table-after-a-cancel
kind: issue
title: after a cancel during the census pass, the mate solve still runs over the partly entered part table and its loose map lands in the canceled Evaluation
status: closed
opened: 2026-10-18
priority: P4
cost: E
closed: 2026-10-18
---


## The finding

`evaluate_at_descent` (`crates/editor-core/src/eval/mod.rs`) runs the
top's census pass (`PartStore::enter_census`, `eval/parts.rs`) with the
caller's cancel token, and the pass stops before its next part when the
token fires. The rows it did not enter refuse `PartFault::NotEntered`.
The mate solve right after it (`crate::mate::solve_with_env(doc, &env,
&reach, tol)`) takes no token, so on a canceled run it still solves
every placement over that partly entered table: each mate on a part the
pass skipped faults `NotEntered`. No node runs (the walk checks the
token before its first node), so no refusal reaches a node, but the
solve's `spaces_of` result is written into the canceled `Evaluation`'s
`loose` map, which is then read off a run that solved over parts it
never entered.

Found in review of PR 5273 (`wire/parts-enter-before-the-map`).

## What a fix is

Either check the token before the solve and return the canceled run
with no solve (an empty `loose`), or hand the token into
`solve_with_env`. The first keeps `mate/*` untouched; the second is the
solve's own question (its owner's ground).

## Resolution (2026-10-18)

The finding's premise no longer holds on main. `evaluate_at_descent`
runs no mate solve: each placement folds its bundle where it evaluates,
and `loose` is `spaces_of` over the node results the walk produced. A
cancel during the census is seen by the walk before its first node, so
no node runs and `loose` comes out empty. No guard was needed, and
`mate/*` is untouched. Pinned by
`asm2a_instantiate::a_run_canceled_in_the_census_has_no_loose_copies`.
In that test a resolver fires the token while the census enters the
first of two parts. The test asserts one resolve (the census stopped
before its second part), a canceled outcome, no node results and an
empty `loose`. Uncanceled, the same document has both
instances as loose copies. The test goes red under a mutant that gives
an unrun placement its recipe space. `enter_census`'s doc, which still
said the census runs "before the evaluation's mate solve", is
corrected.

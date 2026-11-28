---
id: main-red-the-tours-die-with-a-pip-moved-to-0-58-refuses-its-union-at-a-hole-loop
kind: issue
title: Main red: the tour's die with one pip moved to 0.58 of its neighbour refuses its union at a hole loop
status: open
opened: 2026-11-28
---



Red on main at `ceb63c3965` (local run, CI's profile, the whole
editor-core suite, the slow set included), and the same on
`boxfinish/node-store`, whose diff does not reach the union it reads
(the row evaluates at `f64`, with no drive and no store):
`editor-core::all the_tours_die_with_two_pips_run_together_builds::each_pip_moved_into_its_neighbour_builds_its_union_and_meets_a_fillets_door`
(`crates/editor-core/tests/the_tours_die_with_two_pips_run_together_builds.rs`,
the panic in its loop over the pip centres) panics at one pip centre:

> `#129:767741f704e8ebe6 = 0.5800000000000001`: the pips' union builds
> and names: the fold refused at the step that folds in member 5 (read
> 12cdcd2e6d62): the Boolean op refused: the operands' sections could
> not be joined: every vertex of a hole loop lies on the boundary of the
> piece being divided off, so which piece holds it cannot be decided.

Every work row naming this file is closed (the ORBIT and HELDRECORDS
rows that turned it from a refusal into "builds"), so no open row holds
it. Not measured: which commit since 2026-11-24 turned it red, and at
which other pip centres it refuses.

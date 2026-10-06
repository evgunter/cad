---
id: revolve-angle-is-a-signed-size-beside-a-directed-axis
kind: issue
title: The revolve's signed angle stays (Ev, PR 3941); its zero and full-turn refusals state no recourse, as the pattern step's now do
status: open
opened: 2026-10-01
priority: P3
cost: E
---


A named follow-on of Ev's ruling on #3551 (2026-10-01), which adopted the rule "**a size an operation covers is positive; its direction has one home**" and applied it to the extrude first (`work/recipe/extrude-distance-is-a-depth-and-a-side`).

A revolve's angle is signed, and its axis already carries a direction, so the sign states the direction a second time. The #3551 designers rated this "likely" the same shape; it is not measured. Weigh it under the rule before building: whether the angle becomes a positive size with the direction in the axis, what a negative angle means today, and what the refusal's recourse says (Ev asked that the extrude's refusal show how to write the other direction; the same applies here). Record the answer here.

Filed by the AUTHOR orchestrator on Ev's ruling. Ground: `crates/sweep/src/revolve/` (CARVE).

## Answered by Ev's ruling on the pattern step (PR 3941, 2026-10-03)

The pattern-step row (`work/recipe/pattern-spacing-is-a-signed-size-beside-a-direction.md`)
weighed the same question for an angle about a directed axis, and Ev
ruled that such an angle keeps its sign within a turn, "like
everywhere else", naming the revolve's as the precedent. So the
revolve's angle stays signed, nonzero and within a turn, and the
weighing this row asked for is done. What is left is the remainder
that row names: whether the revolve's `DegenerateAngle` and
`FullRangeAngle` refusals state a recourse, as the pattern's
`DegenerateStep` and `FullRangeStep` now do (`eval/wire.rs`
`turns_off` is the shared full-turn decision). That is refusal text
and not on the D10 hold's ground. (CARVE, 2026-10-06, at the cut.)

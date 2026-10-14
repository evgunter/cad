---
id: half-revolve-caps-are-never-an-operand
kind: issue
title: A pi revolve of an axis-touching profile returns coplanar co-oriented Start and End caps on different keys, so it can never be a boolean operand
status: open
opened: 2026-09-25
priority: P2
cost: M
refs: [full-revolve-emits-split-planar-walls]
---


## What

`Revolution::Partial(π)` of an axis-touching profile returns Start and End
caps that are coplanar, co-oriented and adjacent across the axis edge, on
DIFFERENT surface keys. `merge_coplanar_faces` finds no group, and the
boolean's F7 gate refuses `UndeclaredCoincidence`. Their coplanarity rests
on θ = π, which is decided numerically. Measured by GERM's dumbbell lane
(2026-09-25) with a rectangle; a 3π/2 revolve is maximal and unions fine.

## Parked by Ev

Split off the full-revolve fork. Ev, on its `[ev]` PR (2026-09-25): park it,
"file it as P2". If it is ever wanted, both designers pointed the same way:
recognise an exact half turn as a recorded, flagged verdict, the way the
recipe layer recognises `Full` from |θ| − τ at the band, and emit one cap
spanning both sides of the axis. Their reports are in the `[ev]` PR that
ruled `full-revolve-emits-split-planar-walls`.

## Parked on the D10 hold (CARVE, 2026-10-06)

The refusal this row is about is the boolean's `UndeclaredCoincidence` on two caps whose coplanarity is a value-decided coincidence. D10 retires the undeclared-coincidence refusals into the `unproven-coincidence` lint and makes the boolean glue what its verdicts decide Zero (INTENT stage 4), which changes what this row asks for. It waits on that build.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: the F7 gate refuses UndeclaredCoincidence on value-decided coplanar caps; stage 4 turns that into Zero glue plus the lint, changing what the row asks for. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Released by INTENT stage 4 E (`intent/s4-e-glue-on-zero`) (2026-10-09)

E deletes `UndeclaredCoincidence`, but the half revolve is still no operand. The caps sit in ONE operand, and a Zero-decided coplanar neighbour pair there is `NonMaximalFaces` (`crates/topo/src/boolean/reduce.rs:636`), or `CoplanarNeighbours` where the θ = π margin reads in band (`reduce.rs:643`). The boolean's glue door reads cross-operand pairs only (`crates/topo/src/boolean/glue.rs:40`). The revolve runs no merge of its own (`crates/sweep/src/` calls no `merge_coplanar_faces`), so it still returns the caps on two keys. The row's ask stands: either the revolve emits one cap across the axis, or it merges what its margins decide one plane. The merge now glues such a pair by margin (`crates/topo/src/merge_faces.rs:2255`, `faces_continue`).

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

- 2026-11-02 — moved from CARVETAIL to LATHE by `git mv` when CARVETAIL merged into LATHE, id and body unchanged (the track budget now counts only P0, P1 and unbanded rows, and CARVETAIL had fallen far under it).

## Re-measured (2026-11-22, EXPORT, a probe for the `[ev]` PR 7331 fork)

The caps now share ONE plane record. `sweep/src/revolve/partial.rs` joins them across the axis edge through `one_record_across`, and `sweep/tests/revolve_partial.rs::half_revolve_axis_edge_stays_conventional` asserts it. The body is still no operand.

Probe at the default ε (`Tol::witness()`), main at 3a9a9b82e1: the square (0,0)–(1,1) on `SketchPlane::zx()`, axis dir (−1,0), `Revolution::Partial(±π)`.
- **Faces:** 5. The caps are two faces on one `SurfaceKey`, both sense true, with the same outward normal (±y), meeting across the axis edge.
- **Tier 3:** `validate_geometric` returns `Ok`.
- **As an operand:** union, subtract and reversed union against a far brick and against a box poking the top face all return `NonMaximalFaces { edge: <the axis edge> }` (`gate_maximal_faces` in `boolean/reduce.rs`).
- **Other routes leave no such pair:** π/2 and 3π/2 revolves, a triangle touching the axis at one point, a square off the axis, a flush-topped union, and an L union followed by a split.

**The same state reaches import.** `step-import`'s `freecad/fuse_boxes.step` imports with top and bottom each three same-facing faces on one plane record (8 same-key planar adjacencies), passes tier 3, and is refused by the same gate. That is filed as `work/export/an-imported-union-ships-coplanar-same-record-neighbours-no-boolean-takes.md`. Both point at one gap: maximal faces are checked at the boolean's input, but not at rest, although DESIGN says the form "is checked at rest on the result alone".

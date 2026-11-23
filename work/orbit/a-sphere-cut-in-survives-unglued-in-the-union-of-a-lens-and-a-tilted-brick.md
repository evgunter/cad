---
id: a-sphere-cut-in-survives-unglued-in-the-union-of-a-lens-and-a-tilted-brick
kind: issue
title: A sphere cut-in's meridian survives unglued in the union of a lens and a tilted brick, leaving two pieces of one face
status: open
opened: 2026-10-13
priority: P0
cost: H
design: true
parent: closed-sphere-escape-is-re-charted-by-rotation-beside-the-meridian-cut
---


## What

Found closing `work/emit/a-sphere-cut-in-cap-fails-its-names-missing-upstream`
(branch `emit/naming-refusals`, 2026-10-13). The document is
`crates/editor-core/tests/emit_sphere_cut_in_cap.rs`'s `document`: the
lens `ball(1, 0) ∩ ball(0.8, 1.4)`, and a 0.4 × 0.4 × 0.3 brick whose
near face lies 0.985 from the origin toward latitude 68°, azimuth 50°.
The brick meets the lens's unit-sphere face in a circle no edge
reaches, so `topo::boolean::ops::apply_cut_ins` cuts that face along
its chart's meridian through the circle. The cut runs from a hit on
the lens rim, through the circle, to the pole (0, 1, 0).

- **Intersect, either slot, and brick − lens**: the output stage glues
  the cut away. The circle is one closed edge with one conventional
  vertex.
- **Union, either slot, and lens − brick**: the cut survives. Measured
  on the union with the lens first, the result holds the lens face as
  two pieces on one sphere (clone keys `6v5`, `8v1`). Two edges lie
  between them: `6v3`, from the circle to the pole, and `9v5`, from the
  rim to the circle. The circle is cut into two seam edges at the
  cut's two crossings. Both pieces lie on one surface across both
  edges, so the result is not maximal (`docs/DESIGN.md`, maximal
  faces and edges).

## Why it matters

Naming refuses these results. The surviving cut is no edge of the lens
(`BooleanNaming::chart_cuts`), and both of its faces descend to one
lens face. No name reads it, so `emit_topo` refuses it as
`Emission("a chart cut survives in the result")`. Before that guard,
it refused at the vertex where the circle crosses it on one main
(`Emission("a crossing's edge has no piece the boolean classified at the crossing")`)
and `SharedRim { face, other: face, .. }` on the next.
`emit_sphere_cut_in_cap::a_sphere_cut_in_refuses_only_where_its_rows_are_filed`
pins that refusal under this row's path (`CUT_LEFT`). That refusal is an
untyped `NamingError::Emission`, the shape of an emitter bug, on a body
a document reaches: until the join glues the cut, a typed refusal
citing this row is what the result should carry. If the join
glues the cut, those orders reach the conventional vertex as the cap
does, and the pin moves.

## Where to look

Why the curved join leaves the two pieces apart when the cut ends at
the pole (the full revolve's seam strut ends there too). The intersect
cap, whose cut runs between the circle's two crossings only, glues.
`joinable-reads-a-chart-singularity-by-distance-alone` may be the same
ground.

## 2026-11-23 — claimed by ORBIT, re-banded P0 (the die)

ORBIT's P0 (`closed-sphere-escape-is-re-charted-by-rotation-beside-the-meridian-cut`,
PR 7602) retires the rotation re-chart: every sphere escape is now cut
along a meridian, closed balls included. The tour's die with two pips
dragged together (`editor-core` `the_tours_die_with_two_pips_run_together_…`,
eight edits of `die_composed_tour`) no longer refuses at the turned
seam. All eight now refuse at the same `Node::Union` with
`Emission("a chart cut survives in the result")`, which is this row.
The kernel body is reported sound. Measured on a two-pip union by the
P0 lane: `merge_faces` skips each ball's run with `PeriodClosure`. The
cut's pieces meet across both the cut and the ball's own seam, and C12(5)
(`crates/geom-brep/README.md`, ratified) refuses a curved run that closes
its period. It refuses the whole run rather than killing the cut and
keeping the seam.

This row now stands between a normal verb and normal geometry (a valid
part a user gets by dragging a pip), so it takes the P0 band
(`work/README.md`, Priority: a blocker inherits the band of what waits
on it). How a chart cut that survives into the result should be treated
reads a ratified clause, so it carries `design: true`: designers first.
Moved from `work/hone/` (HONE is `ready`, with no orchestrator); id
unchanged. (ORBIT orchestrator)

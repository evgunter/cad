---
id: a-sphere-cut-in-survives-unglued-in-the-union-of-a-lens-and-a-tilted-brick
kind: issue
title: A sphere cut-in's meridian survives unglued in the union of a lens and a tilted brick, leaving two pieces of one face
status: closed
opened: 2026-10-13
priority: P0
cost: H
parent: closed-sphere-escape-is-re-charted-by-rotation-beside-the-meridian-cut
closed: 2026-11-24
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

## Evidence (2026-11-23, ORBIT `orbit/one-sphere-escape-cut`)

**This row now stands between the tour's die and building.** That branch retires the re-chart, so a closed ball's escape is cut in too (`apply_cut_ins`). The die of `die_composed_tour` with two pips dragged together used to refuse `UnsupportedTurnedCell` at its pips' union. It now refuses there with this row's `Emission("a chart cut survives in the result")`, in all eight edits (`crates/editor-core/tests/the_tours_die_with_two_pips_run_together_refuses_their_surviving_cut.rs`). The kernel result is sound; only the naming refuses.

Where the cut survives, measured on two radius-0.09 balls 0.08 apart (z-poled, offset along y). The union keeps one live chart cut per ball, and its naming lists `merge_skipped: PeriodClosure` for each ball's run. The run's faces meet across both the cut and the ball's own seam, so merging the whole run would close the sphere's period. `merge_faces` refuses curved runs that close the period (`crates/geom-brep/README.md` C12(5)), and it refuses the whole run rather than merging across the cut while the seam stays as the period's edge. `a - b` keeps its cut for the same reason. The intersect glues both cuts away. The same skip already leaves operand seams unmerged on balls that need no cut-in, for example y-poled balls offset along y. There the edges are operand edges, which names read.

Two ways through, each on other ground. One is a partial merge: kill the chart cut and keep one period edge, the operand's seam. The question there is whether C12(5)'s "a run that closes its period refuses" allows a sub-period merge of the same run. The other is to name a surviving chart cut. By the blocker rule (`work/README.md`, Priority), the part of this row that unblocks the die is P0 work.

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

## Closed (2026-11-24, PR 7664)

Ev ruled the fork on [ev] PR 7639: a cut the kernel makes for its own
reasons is transient, and the door that mints it removes it. PR 7664
carries the cut-in's meridians into the re-entered boolean as a record
and closes each surviving piece in the output stage (`slit::close_cuts`:
`kef`, `kev`, `kemr`) before the coplanar merge, so no result holds a
chart cut and naming no longer reads them. The lens-and-brick union and
lens − brick now build without a surviving cut, and so does the die's
pips union; their conventional vertex is named by PR 7694 (next
section), with which every op here builds, names and meshes. Dual review (two Opus) approved at d8b78b203; the unreached
ring-move path is `a-chart-cut-kef-onto-a-ringed-piece-has-no-row`.

## 2026-11-24 — every op names on `orbit/conventional-vertex-names`

With PRs 7602 and 7664 beneath it and the conventional vertex named
for its circle (`a-conventional-vertex-mints-no-edge-derived-name`),
all six ops of `emit_sphere_cut_in_cap`'s document build and name, the
union and the lens minus the brick included: no cut survives there. The
pin named above is now
`emit_sphere_cut_in_cap::every_op_over_a_sphere_cut_in_names_the_circles_vertex_for_the_circle`,
which asserts each op names one conventional vertex on the seam between
the lens and the brick. (orbit-cvname lane)

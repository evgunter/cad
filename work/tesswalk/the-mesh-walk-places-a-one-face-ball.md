---
id: the-mesh-walk-places-a-one-face-ball
kind: issue
title: the mesh walk places a one-face ball (one wrap meridian pole to pole) as the two sides of its wrap, not a zero-width column
status: closed
opened: 2026-10-29
closed: 2026-11-11
priority: P1
cost: E
pr: 6823
branch: tesswalk/one-face-ball
---


Ev ruled on `[ev]` PR 6014 (2026-10-29, fork row 215) that a chart singularity (a sphere's pole, a cone's apex) with no written edge to it is reached by ONE wrap edge, walked out and back: the wrap tip (DESIGN `[topology]`, the seam clause and tiers 2/3), "if the complexity it adds is tolerable", asking that keeping that complexity down be investigated. The probe that measured each reader is on branch `probe/foreign-wraptip` (edits E1–E7).

Owed here: `mesh::walk::loop_polygon` places a rimless loop of one edge walked up and back as the two sides of its wrap, a period apart (the probe's E7: before it `SingleColumnCurvedFace`, after it watertight, 960 triangles, the two-band ball's signed volume). Hemisphere and cone caps meshed with no edit.

Signed: (FOREIGN orchestrator)

## Released (2026-10-30)

`loop-readers-each-derive-a-corners-turn-with-no-shared-loop-walk` closed with PR 6088. The shared structure is `Body::both_sides_face`, read through two views: `surface_cycles` (region) and `chart_image` (spur-free: integrators and chart readers). Read the tip through these, never by a turn of your own. (RESTCURVE orchestrator)

## Held (2026-10-30)

Re-parked on `sphere-faces-take-a-per-face-chart-or-pole-vertices` (a ruling open on `[ev]` PR 6085). Ev pointed out that a sphere face other than the whole sphere has a singularity-free chart, which may retire the pole vertex and the wrap tip for spheres. Do not build on the wrap tip until it is ruled. (RESTCURVE orchestrator)

## Released (2026-10-31)

Ev ruled on `[ev]` PR 6085, decision 3 (fork-log row 222): keep the wrap tip. A tip is any valence-1 end of a wrap edge, and the wrap edge is the face's seam kept to its end. A chart reader reads a tip as the seam, through the lift's reset at the singular vertex. A chart-free reader drops it, keeping the singular vertex as the side's datum. The hold is lifted. (RESTCURVE orchestrator)

## Dispatch

2026-11-11, TESSWALK orchestrator. One implementer, lane `tesswalk-ball`, branch `tesswalk/one-face-ball`. The lane will:
- reproduce the one-face ball's `SingleColumnCurvedFace` on main;
- port the probe's E7 (`probe/foreign-wraptip`) onto `mesh::walk::loop_polygon`, reading the tip through `Body::both_sides_face`'s `chart_image` view as RESTCURVE's release says, never by a turn of its own;
- pin the ball watertight with its signed volume.

Review tier: a single style review, since the change sits in the shared mesh walk and must read the tip through the one structural predicate.

## Closed

2026-11-11, PR 6823. Reproduced on main: the one-face ball (the unit ball
with one meridian killed by `kef_describing` and the survivor restated as
the wrap edge) is tier-3 valid and refused `SingleColumnCurvedFace` at
`walk::require_two_columns`. No user-facing door mints it yet (revolve
mints two bands; import's flip is parked), so the row builds it with Euler
operators. `mesh::walk::wrap_strip` reads the loop through
`topo::surface_boundary::chart_image`: an empty image whose two tips are
the chart's two singular points is the whole surface, and the walk places
the edge's two sides a period apart (`strip_columns`), wound by the face's
sense, with no turn read at the tip. Pinned in
`crates/mesh/tests/the_one_face_ball.rs`: watertight, within δ·area of
4π/3 and of the two-band ball's signed volume, the reverted ball the void;
the wrap edge split off the poles meshes too (on main it panicked at the
census); the hemisphere and cone wrap-tip caps mesh byte-identically to
main. Mutants (zero-width column, sense dropped, sign flipped) each kill a
named row. The one-seam sphere in `loops_with_no_rim.rs` moves from the
single-column class to the strip.

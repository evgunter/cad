---
id: cell-region-clear-of-singular-reads-a-spline-chart-as-having-no-singular-point
kind: issue
title: cell_region's clear_of_singular reads a spline (or Approx) chart as having no singular point, by kind
status: closed
opened: 2026-11-03
priority: P2
cost: E
refs: [a-nurbs-wall-with-a-collapsed-boundary-column-has-no-loop-joint, edge-join-singular-at-reads-a-spline-pole-as-not-singular]
closed: 2026-11-22
rides_with: a-nurbs-wall-with-a-collapsed-boundary-column-has-no-loop-joint
pr: 7325
---

Filed by SPLICE's `splice/kind-reads` lane, from its sweep for doors that
read a spline face by its kind rather than its geometry.

## Finding

`crates/topo/src/cell_region.rs` `clear_of_singular` decides a station
clear of its face's chart singularity on a cone (distance to the apex) and
a torus (distance to the axis), and answers every other surface `true` by
its last arm (`_ => true`). On a sphere that is the poles' own reading
elsewhere; on a NURBS or `Approx` chart it reads "no singular point" by
kind. A spline chart can have one: a collapsed boundary row or column (a
loft's pinned corner,
`a-nurbs-wall-with-a-collapsed-boundary-column-has-no-loop-joint`), where
the chart's normal vanishes, so a station there is read clear and the cell
reads Inside or Outside on a chart it cannot place a side on.

The boolean's twin, `boolean::edge_join::singular_at` (`_ => return
Ok(false)`), is `work/parting/edge-join-singular-at-reads-a-spline-pole-as-not-singular`.

**Latent as far as this lane read.** The loft refuses a wall that
collapses to a point (`sweep::loft`, `PinnedCornerUnsupported`), and
`pcurves::singular_at` reads a spline only at the whole-net level. Whether
STEP import admits a NURBS net with a collapsed row was not checked; if it
does, this is live.

## Fix

Give the arm a spline chart's reading (a collapsed boundary row or column,
read off the net, as the loop joint row would read it) or refuse typed,
with a row on a spline face whose chart collapses a boundary column.


## Closed (2026-11-22, with `a-nurbs-wall-with-a-collapsed-boundary-column-has-no-loop-joint`)

`crates/topo/src/cell_region.rs` `clear_of_singular` reads a NURBS or
`Approx` face's collapsed sides from the singular set's one home
(`geom_brep::chart_singularity::singular_points`). A station must be
decided clear of each side's point by the cell's reach, as a cone's
apex is. The last arm now names the plane, the cylinder and the sphere,
whose poles this reader has never read (unchanged). A collapsed side
is one stored point (DESIGN `[extensibility]`, ratified on PR 7287),
so no band decides which sides there are.

- 2026-11-23 — `sweep::LoftError::PinnedCornerUnsupported`, cited above,
  is deleted (LATHE, `lathe/skin-sub-arc-walls`): the loft cuts a full turn
  into quarter walls at its sub-arc joints, so the one case it refused, a
  one-segment loop's pinned vertex, builds as an ordinary pinned corner
  (`sweep`'s `pinned_corners_at_the_loft_door::a_full_turns_pinned_vertex_builds`,
  closing `work/lathe/a-full-turns-pinned-vertex-refuses-at-the-loft-door.md`).
  The citation is left as written, as the record of what the loft did then.

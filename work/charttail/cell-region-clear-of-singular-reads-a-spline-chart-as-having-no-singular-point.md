---
id: cell-region-clear-of-singular-reads-a-spline-chart-as-having-no-singular-point
kind: issue
title: cell_region's clear_of_singular reads a spline (or Approx) chart as having no singular point, by kind
status: open
opened: 2026-11-03
priority: P2
cost: E
refs: [a-nurbs-wall-with-a-collapsed-boundary-column-has-no-loop-joint, edge-join-singular-at-reads-a-spline-pole-as-not-singular]
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


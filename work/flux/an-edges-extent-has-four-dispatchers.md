---
id: an-edges-extent-has-four-dispatchers
kind: issue
title: an edge's extent has four per-Curve3 dispatchers and no home, so each per-site fold of a face's extent picks a different one
status: open
opened: 2026-10-06
priority: P3
cost: M
---


Found by PR 4115's review (the SHELL `planar-gate-misses` lane, Style Q1
and claim C6). Filed on flux because `geom::curves::boxes`, the most
exact of the four, is flux ground (`work.py territory`).

**The question with no home**: *what region can an edge's arc occupy?*,
an upper bound to fold into a face's extent. It is answered by four
per-`Curve3`-variant dispatchers, each with its own bound and tightness:

1. `geom::curves::boxes::{circle,ellipse,spiric}_arc_aabb` /
   `conic_arc_aabb` / `nurbs_curve_aabb`: exact arc boxes, outward
   rounded, but behind `T: Bounds`, which most `topo` callers do not
   carry (`bounds-allowlist.sh` ratifies compound bounds per file).
2. `topo::splitting::containment::carrier_ball`: a ball under `Decide`.
   A conic gets its whole carrier's ball, a spiric a speed-bound ball
   about the arc's midpoint, a spline its control box's ball. The shell
   clearance gate (`shell::carrier_box`) now reads this one; it was
   made `pub(crate)` for that in PR 4115.
3. `topo::boolean::boxes::{conic_extent, arc_extent}`: full-turn
   amplitude and certified subdivision over `Span`s at `Real`.
4. Until PR 4115's fix pass, a fourth one in `shell.rs` (`arc_extent`).
   It was deleted for `carrier_ball`.

The torus's own ball, `major_radius + minor_radius`, is respelled as a
literal at several sites: `boolean/solid_contain.rs` (two in the
spiric box reads, one `ext`), `splitting/containment.rs` (a tilt lever),
`boolean/carrier_eq.rs`.

These do not ask the question, and are not this class:
`geom_brep::certify::edge_extent` and `readback::edge_extent` are
LOWER bounds on an edge's diameter (a lever's certified direction).
`merge_faces.rs` `boundary_points` are on-face witness points.

**Three per-site folds that forgot the arcs wait on this**. Each would
otherwise pick a different dispatcher:
`chart-region-carrier-gates-read-vertices-not-arcs` (chart),
`splitting-face-extent-lever-reads-vertices-not-arcs` (hone) and
`tangent-witness-span-reads-vertices-not-arcs` (tang).

**Final state**: one recommended door for "an edge's extent in a frame
or about a point" at the `Decide` bound, which the three sites and the
shell gate read. The exact `Bounds` boxes stay as its tight twin, and
the literal torus ball becomes a named helper.

---
id: a-nurbs-wall-with-a-collapsed-boundary-column-has-no-loop-joint
kind: issue
title: a NURBS wall with a collapsed boundary column (a loft's pinned corner) has no loop joint: the walk resets only on a sphere's or cone's singular point
status: open
opened: 2026-10-31
priority: P1
cost: H
---

Filed by CARVE's `carve/pinned-corners` lane (phase 1 of
`work/carve/a-wall-pinned-between-two-loft-sections-refuses-at-the-wrong-door.md`).
P1 because that P1 row's class 1 waits on it.

## What

A loft section whose corner is the same world point in two adjacent
sections (a pinned corner; a hinge is two adjacent ones) is a legal
body: the corner is one vertex, and each skinned NURBS wall beside it
has a collapsed boundary column there, a chart singularity, which
DESIGN's tier-3 clause makes a vertex. The kernel cannot build it:

- `geom_brep::chart_singularity::singular_points` names a sphere's
  poles and a cone's apex only; a spline chart has no singular set.
- `topo` `pcurves.rs` `singular_at` (line ~4070) reads the singular
  set for a sphere or cone only, so `decide_joint` (line ~4318) never
  writes `JointElement::Reset` at a spline joint, and reads the joint by
  its chart-space gap (`spline_gap_closes`, line ~4732). At a collapsed
  column the wall's bottom rim ends at `(u, 0)` and its top rim starts
  at `(u, 1)`: one 3-D point, a whole `v` span apart in the chart, which
  no deck element of an open net closes.
- Today the loft mints a strut there, a zero-length edge, and the
  attach gate refuses it (measured on main by
  `crates/sweep/examples/carve_pin_phase1.rs` on `carve/pinned-corners`,
  commit c9c20ba: `StatedInterval { Forward(Zero) }` for a two-section
  wedge, `IntervalNotForward` length 2.2e-16 for a corner pinned
  through three sections at v-degree 2).

The loft door now refuses the case typed
(`sweep::LoftError::PinnedCornerUnsupported`, naming this row), so
nothing reaches the gate.

## What closing it takes

A spline chart's collapsed boundary column read as a chart singularity
(its own `Singularity` kind, decided as 3-D incidence like a pole), so
the loop walk resets its joint there and tier 3's chart-singularity and
wrap-tip readings know it; then the loft assembles class 1 (no strut at
a pinned corner; a segment pinned at both ends has no wall, and its
neighbours share the edge, as revolve's `WallClass::OnAxis` does) and
the door's last refusal (`crates/sweep/src/loft/stacking.rs`,
`stacking_door`, step 4) is deleted.

Class 1's build also owes the skin's vertex rows reading the door's
verdict: today `crates/sweep/src/skin.rs` `chord_length_parameters`
reads every control row by an exact compare, so a corner the door
decides pinned only up to rounding would count as moving and set the
parameters by that rounding's ratios. While the door refuses every
pinned corner before the skin, no such row reaches it.

---
id: a-nurbs-wall-with-a-collapsed-boundary-column-has-no-loop-joint
kind: issue
title: a NURBS wall with a collapsed boundary column (a loft's pinned corner) has no loop joint: the walk resets only on a sphere's or cone's singular point
status: closed
opened: 2026-10-31
priority: P1
cost: H
branch: charttail/collapsed-column
pr: 7325
closed: 2026-11-23
---

Filed by CARVE's `carve/pinned-corners` lane (phase 1 of CARVE's
`a-wall-pinned-between-two-loft-sections-refuses-at-the-wrong-door`,
the row that weighed the loft's pinned corners into three classes and
built the stacking door's refusals; it closed with PR 6141 and left the
tracker with CARVE, recoverable at 0f01217666 per
`docs/doc-ledger/carve-leaves-the-tracker.md`). P1 because that P1
row's class 1 waits on it.

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

## Built (2026-11-22, on Ev's ruling on PR 7287)

The row's "What" no longer holds as written. There is no
`JointElement::Reset`: an element at a singular point is an ordinary
deck element. `singular_points` was keyed on a `ChartInverse`, which
excludes `Nurbs` by type. Five readers already read a collapsed side,
and they disagreed. What the branch makes true:

- **One home.** `geom_brep::chart_singularity::singular_points(&Surface)`
  is exhaustive over kinds. A spline net's side is
  `Singularity::Side(ChartSide)` where its control points are all one
  stored point (`collapsed_sides`, exact, ±0 one value). Its readers
  ask it: tier 3's singularity-as-vertex check and wrap tips,
  `euler_ring` (a side's joint stands at either sense), `topo::chart`,
  step-import's normalization and `recognize.rs` (its bitwise
  `collapsed_side` folded in), `pcurves` `singular_at`, `edge_join`
  `singular_at`, and `cell_region` `clear_of_singular`.
  `spline_collapsed_row` reads the same exact rule: it is no longer
  banded, and it no longer folds undecided into collapsed.
- **The joint.** A vertex decided on a collapsed side frees the channel
  along it. The other channel is checked at its lever, with the
  identity on an open net (`AzimuthRule::OnCollapsedSide`). A side along
  a channel the net closes (a NURBS cone's apex) refuses
  `SingularMiss::ClosedSide`. Closure itself is
  `a-spline-charts-closure-is-read-at-a-band-and-folds-in-band-to-open`.
- **Exact constructors.** The loft door returns its classes. The skin
  writes a pinned corner as its first section's point (`pin_strips`).
  A row of one stored point abstains from the parameters and passes
  through the solve unsolved, and the knot algebra keeps a blend of one
  point with itself as that point.
- **Class 1 builds.** There is no strut at a pinned corner, a flat
  segment has no wall, and a cap–cap hinge rests as its wedge demands.
  `stacking_door`'s step 4 now refuses a full turn's one vertex only.
  The rectangle quadrature lane reads a collapsed side as a side of its
  rectangle.
- **Import heals** a side read one point at ε_in, and reports it
  (`NormalizationKind::CollapsedSideMadeOne`).

## Closed (2026-11-23)

By PR 7325, on Ev's ruling on PR 7287: a spline chart's singular set is read exactly off its net, with one home, `singular_points(&Surface)`. A vertex on a collapsed side takes `AzimuthRule::OnCollapsedSide` (the free channel along the side, both ends checked on it), and the loft builds a pinned corner (class 1), a hinge and a lune exactly. `edge_join` and `cell_region` ask the home. The full turn's single pinned vertex is LATHE's `a-full-turns-pinned-vertex-refuses-at-the-loft-door`, and the closed collapsed side is `a-closed-collapsed-side-has-no-turn-or-side-reading`.

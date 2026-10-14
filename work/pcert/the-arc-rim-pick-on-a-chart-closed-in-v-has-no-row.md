---
id: the-arc-rim-pick-on-a-chart-closed-in-v-has-no-row
kind: issue
title: on a chart closed in v the arc rim's pick handed both halves of a seam circle the v₀ row by candidate order
status: open
opened: 2026-10-13
priority: P3
cost: E
---


Found by the review of PR 4988 (`pcert/arc-rim-row-pick`, its S6).

## What is missing

`nurbs_iso_derive`'s arc rim (`crates/topo/src/pcurves.rs`, the
`arc_rim` closure) offers four `IsoArc` candidates to `probe_pick`:
each boundary row `v ∈ {v₀, v₁}` × each `u` direction, `v₀` first. On a
chart closed in `v` the two rows are one 3-D curve, so both rows' images
hold the carrier at the probe and the pick returns the `v₀` one by
order alone. That is the one case where the candidate ORDER, not the
geometry, decides the stored row. Both answers lie on the same 3-D
locus, so the arc-rim certification accepts either; what is unpinned is
which row the loop walk then sees, and whether it closes.

## Why no row in 4988

A chart closed in `v` with the rim running along `u` is a torus-like
spline wall: the rim is a circle along `u`, and `v` closes on itself.
`sweep::loft_body`, the door 4988's rows use, stacks sections, so its
`v` is open; 4988 did not find another door that builds one, and did
not survey them all. Building it needs a spline chart closed in `v` set on a face whose
rim edge is a circle along `u`, through a door that keeps the rim's
description coherent.

## Fix

A row on such a body: the rim derives its image, the mint stores it,
and the loop walk closes over it. If the `v₀`-first order makes the
walk refuse where the `v₁` row would close, the pick wants the loop's
own context (the neighbouring rows), not a fixed order.


## What step 0 found (PCERT, 2026-10-14)

**The case is a seam, and the order was wrong for one half.** On a
chart closed in `v`, a circle along `u` on row `v₀` is the same 3-D
curve as row `v₁`. A face that wraps `v` meets that edge from both
sides, so the edge is a seam: both of its halves bound the same face.
A rim to a second face would need the face split in `v`, and then its
chart is not closed. So the only edge that reaches this pick with both
rows holding is a seam circle, and the walk needs one half on `v₀` and
the other on `v₁`. The fixed order handed both halves `v₀`.

**No door builds it, and no census pick hit it.**
- Over 264 arc-rim picks, more than one candidate never held.
- Revolve profiles are line/arc segments, and they mint analytic walls.
- Loft stacks sections, so its `v` is open.
- The row therefore builds the wall itself: `tube_along_arc`'s quarter
  tube, its two torus faces joined by killing the inner equator, then
  re-charted on the exact rational biquadratic torus net with `v₀` at
  the outer equator.

## Fixed

`nurbs_iso_derive`'s `arc_rim` (`crates/topo/src/pcurves.rs`) offers
first the two candidates on the half's own side, then the other two.
- **The side rule.** The face lies left of the half-edge about its
  outward normal, and left of `+u` about the chart normal is `+v`. So
  `v₀` is the half's row where its walk runs `+u` exactly when the
  face's sense agrees with the chart's normal.
- **Open charts pick as before.** Only one row holds there; the order
  moves only how many probes run before the winner.

**The row:** `sweep/tests/an_arc_rim_picks_the_row_it_lies_on.rs::a_seam_circle_on_a_chart_closed_in_v_takes_the_row_on_each_halfs_side`.
- The seam's halves derive the `v = 0` and `v = 1` rows, both running
  with the chart's `u`.
- Each is checked against a 3-D side oracle: the chart's step into the
  face from the row points left of the half about the outward normal.
  The oracle is read off the net and the carrier, not off the pick's
  algebra.
- **Mutants, both red at the oracle:**
  - the old fixed order: the second half takes `v = 0`;
  - the side rule inverted: the first half takes `v = 1`.

**The loop walk does not close yet, for a second reason.** The wall's
cap arcs are circles along `v` and take `General` images. The fitted
foot at the seam vertex lands on the `v = 0` branch, so an arc running
`v` from 0.5 to 1 is fitted as ending at 0. Before this fix every
image folded onto `v₀`, so the walk read no gap, and the folded fit
then refused certification (`FittedLimb HullValueChart`). After it,
the walk refuses `LoopDiscontinuity` at the seam half on `v₁`. Filed
as `work/iso/a-general-image-ending-on-a-closed-charts-seam-takes-its-foot-on-the-first-branch.md`.

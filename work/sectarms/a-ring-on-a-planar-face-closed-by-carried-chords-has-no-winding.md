---
id: a-ring-on-a-planar-face-closed-by-carried-chords-has-no-winding
kind: issue
title: A ring the join closes on a planar face with carried chords has no winding: choose_roles refuses CurvedBooleanUnsupported naming the plane, and three readings of a spline run are viable
status: closed
opened: 2026-11-12
priority: P1
cost: M
closed: 2026-11-13
---


Filed by the `sectarms/spline-ring-winding` lane, which wound the ring
on the spline wall and stopped here on the fork below.

## What

A slab round the loft prism (`x, y ∈ [−3, 3]`, `z ∈ [0.75, 1.25]`)
meets the prism's four walls in a closed loop on each of its two big
faces: four arcs, one per wall, each a carried component of a plane ×
spline section. Each wall's chord runs rim to rim and takes the outer
lane. The planar face's loop is a ring, and
`boolean::join::choose_roles` takes a `RingClosure::Chord` only on a
face `ring_path::Carrier::of` reads (a cylinder, sphere, torus or
spline wall), so the plane refuses
`CurvedBooleanUnsupported { kind: Plane }` naming the slab's face, in
`prism ∪ slab` and `slab ∖ prism` (measured on this branch, about 25 s
a run in debug).

Admitting it takes two things. `choose_roles` hands the planar face to
the planar ring lane (`RoleLane::Ring`), and that lane's winding
(`ring_run_ccw`, through `Body::planar_run_winding_decided`) has to
read a spline: the closing chord is one, and so are the run's earlier
chords, which `run_spline_edge` refuses today
(`EdgeCarrierUnsupported`, site `RingRun`) before the sum answers
`Unwound::Spline`.

## Decided (Ev, PR 6896, 2026-11-13; fork-log row 265)

A planar loop's winding stays one reading, the shared sum in
`loop_winding`, and that sum gains a spline edge term, the same way it
already has line, conic and spiric terms:

- **The term comes from `geom_brep::props`**, beside
  `loop_vector_area`, rather than from a second integrator in `topo`:
  twice the edge's vector area about its own start point. It is exact
  for a non-rational spline (Gauss per knot span). For a rational one it
  is a certified enclosure (`props::rational_rim`), refined by piece
  count until the sign is decided, as `spiric_winding` does for the
  spiric. The perimeter lever is an upper bound from the carrier's
  speed.
- **`Unwound::Spline` retires.** `Uncertified` stays, for an
  enclosure-only carrier at a scalar with no certified lane.
- **The face's surface picks the reading, not the germ lane.** A plane
  is wound by the sum, closed by the segment's curve, whichever lane
  minted it. `choose_roles` routes a plane's `RingClosure::Chord` to the
  planar ring lane, and `run_spline_edge`'s pre-refusal and
  `ring_run_ccw`'s "no lane mints a spline on a plane" invariant go.
- **Every reader of the sum answers alike**: the merge's role
  assigner, tier 3's check 6 planar arm, the join's ring lane, the
  splitter's `section_loops::loop_sense` and `chord_join`'s plane
  re-homing.
- **A fitted chord's pad is not charged.** `n · A⃗` reads the stored
  arcs' projection on the plane, so their height off it drops out
  exactly.

## Done when

- The slab row above passes `choose_roles` in both orders and is
  pinned where it stops next.
- Check 6's planar arm checks role and sense on a planar loop with a
  spline edge, where today it skips it (`validate.rs:8211`, `let
  Some(winding) = … else { continue }`).
- `a_nurbs_carrying_cycle_stays_undecidable` is re-pinned to a decided
  winding.
- A rational spline edge is decided at a certifying scalar.
- `loop_winding`'s module docs and `ring_run_ccw`'s doc describe the
  new carrier set.

## Closed (2026-11-13, `sectarms/planar-spline-winding`)

The sum reads every carrier. Which reading an edge takes is
`geom_brep::props::segment_kind`'s table, beside `loop_vector_area`:
a line is its chord; a circle, an ellipse and a non-rational spline of
degree up to 5 have a closed form at the body's scalar
(`closed_segment_term`); a spiric, a rational spline and a non-rational
one past degree 5 are read by a certified enclosure built once per edge
and refined by piece count (`SegmentQ`), at the interval scalar
(`loop_winding`'s `enclosed_winding`, which `spiric_winding` became).
`Unwound` keeps `Uncertified` alone.

`choose_roles` hands a plane's `RingClosure::Chord` to the planar ring
lane, and `run_spline_edge`'s pre-refusal covers a wall's and a
quadric's closing only (the P3 item
`a-quadric-or-wall-ring-closing-refuses-a-spline-run-edge-it-may-read`
holds those).

The slab's five runs pass the join and stop at the inside/outside test
(`work/inside/point-containments-ray-test-has-no-spline-arm.md`),
pinned by sweep's
`a_slab_round_the_prism_closes_a_ring_on_each_of_its_big_faces`. That
stop comes before any reader of the slab's faces, so which way round a
planar island winds is held by topo's rows:
`loop_winding::spline_rows`, the join's
`a_planar_ring_run_closed_by_a_spline_winds_by_the_splines_area`, and
`merge_faces`' `a_spline_carrying_cycle_winds_alike_for_every_reader`,
which also holds check 6 refusing a planar face with a spline edge
wound against its sense.


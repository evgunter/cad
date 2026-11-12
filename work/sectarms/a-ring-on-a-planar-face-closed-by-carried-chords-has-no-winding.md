---
id: a-ring-on-a-planar-face-closed-by-carried-chords-has-no-winding
kind: issue
title: A ring the join closes on a planar face with carried chords has no winding: choose_roles refuses CurvedBooleanUnsupported naming the plane, and three readings of a spline run are viable
status: open
opened: 2026-11-12
priority: P1
cost: M
design: true
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

## The fork

Three readings of a planar run that holds spline arcs are viable, and
the text does not choose between them.

1. **The shared sum reads a non-rational spline.** `loop_winding`'s
   module docs name it as the remainder that home has not built:
   `geom_brep::props::loop_vector_area` integrates a non-rational
   B-spline's vector area exactly per knot span, so a spline edge's
   bulge over its chord is one more per-edge term, and its perimeter
   term an upper bound from the curve's speed. The same docs say the
   merge's role assigner, tier 3's check 6 and the join's ring lane
   answer on one carrier set, so this moves all of them: check 6 starts
   falsifying planar loops with non-rational spline edges, the merge
   door starts assigning roles on them, and the row that pins the
   present answer
   (`merge_faces::…::a_nurbs_carrying_cycle_stays_undecidable`, "no
   closed form exists for the region a fitted carrier bounds") flips. A
   rational spline, and a degree past the Gauss table's five nodes,
   stay unwound.
2. **The ring lane alone reads the spline term**, on an open run
   closed by a carried chord. Nothing else moves, and the three sites
   stop answering on one carrier set, which the module docs give as
   the reason the sum has one home.
3. **Paths on the plane**, the reading the curved faces take
   (`chord_join::path_island_winding_closed_by`): a straight segment
   from an outer-loop point to the closing chord's middle, its
   crossings of the run read by the curve-pair table. It reads a
   rational spline too and needs no area, but it is a second reading
   of a planar island beside the sum, and `path_parity`'s line and
   conic arms are stated for pieces on a quadric, so a planar run's
   lines and conics would need arms of their own or the table's.

A fitted carried chord lies in the plane only within its pad, which
each reading has to hold: the sum projects on the plane's normal, and
a path's meetings are read in space within the band.

## Done when

The slab row above passes `choose_roles` in both orders and is pinned
where it stops next.

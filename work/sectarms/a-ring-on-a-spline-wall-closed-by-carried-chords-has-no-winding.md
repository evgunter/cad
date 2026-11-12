---
id: a-ring-on-a-spline-wall-closed-by-carried-chords-has-no-winding
kind: issue
title: A ring the join closes on a spline wall with carried chords has no winding: choose_roles refuses CurvedBooleanUnsupported naming the wall
status: open
opened: 2026-11-12
priority: P1
cost: M
---

Filed by the `sectarms/join-reads-spline-implicit` lane.

## What

A brick across the loft prism's flat wall, wholly inside the wall's
face (`x ∈ [−0.25, 0.25]`, `y ∈ [0.75, 1.25]`, `z ∈ [0.75, 1.25]`),
meets the wall in a closed loop of four arcs, one per brick face, each
a carried component of a plane × spline section. The join reads them
on the value and mints the chords, and the wall's ring lane then
refuses: `boolean::join::choose_roles` takes a `RingClosure::Chord`
only on a cylinder, sphere or torus face (`ring_path::Quadric::of`), so
a spline wall refuses `CurvedBooleanUnsupported { kind: Nurbs }` naming
the wall, in every op. On main the same runs refused earlier, at the
germ frame (`GermFrameUnsupported`), in about 0.4 s each in debug; they
now take about 7.3 s each, of which the four sections' whole readings
are 0.56 s and the at-rest certificates of the 12 chords the join mints
6.4 s
(`a-carried-plane-spline-chords-at-rest-certificate-costs-half-a-second`).

The island's winding on a spline face needs paths on the wall's chart
(the cylinder's ring lane winds on its chart), not the quadric paths
`chord_join::path_island_winding_closed_by` reads. A planar face closed
by a carried chord refuses the same way; its ring lane winds by the
segment's curve, and `run_spline_edge` refuses a spline closing curve,
so admitting it needs that lane to read a spline arc too.

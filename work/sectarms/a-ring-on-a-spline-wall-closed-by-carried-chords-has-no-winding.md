---
id: a-ring-on-a-spline-wall-closed-by-carried-chords-has-no-winding
kind: issue
title: A ring the join closes on a spline wall with carried chords has no winding: choose_roles refuses CurvedBooleanUnsupported naming the wall
status: closed
opened: 2026-11-12
priority: P1
cost: M
closed: 2026-11-12
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

## Closed (2026-11-12, `sectarms/spline-ring-winding`)

A ring on a spline wall is wound as a carried chord's cylinder is, by
the parity of a path across the island's boundary
(`chord_join::path_island_winding_closed_by`), the path's two pieces
the wall's own iso curves (`ring_path::SplineWall`, where a cylinder's
are a ruling and a parallel) and its meetings with the run's spline
arcs the curve-pair table's. A bystander ring is re-homed by the same
paths (`path_ring_side`). The brick's five runs pass the join and stop
at the inside/outside test
(`work/inside/point-containments-ray-test-has-no-spline-arm.md`),
pinned by
`a_brick_inside_a_lofted_walls_face_closes_a_ring_on_the_wall`; a fork
with two prongs in the wall, whose second prong's rings are bystanders
of the first island, is pinned there too. That stop comes before any
reader of the wall's two faces, so which way round the island winds is
held by topo's
`a_spline_wall_island_winds_as_its_corners_turn_about_the_outward_normal`
and not by the brick.

The planar face's case is not built: three readings of a planar run
with spline arcs are viable
(`a-ring-on-a-planar-face-closed-by-carried-chords-has-no-winding`).
Two walls stay unread, each typed: one whose weights are constant
along neither parameter
(`a-spline-wall-whose-weights-vary-along-both-parameters-has-no-ring-winding`), and
any wall at a scalar wider than a point
(`a-spline-walls-ring-is-not-wound-at-an-enclosure-scalar`).
Re-homing refuses where the face a chord walls off holds one of the
wall's rim edges
(`re-homing-a-ring-on-a-spline-wall-refuses-where-the-new-face-holds-a-rim-edge`),
and a point just below a closed wall's seam has no path
(`a-closed-spline-walls-projection-stops-at-its-seam`).

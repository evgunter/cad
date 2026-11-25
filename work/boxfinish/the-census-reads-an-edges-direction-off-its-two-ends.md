---
id: the-census-reads-an-edges-direction-off-its-two-ends
kind: issue
title: The census reads an edge's direction off its two vertices where the ray's pass reads its stored line
status: open
opened: 2026-11-15
priority: P2
cost: M
refs: [6958]
---

Left by the aimed-ray unit (PR 6958), disclosed in its body and raised by
its second review.

**What is there.** One file holds an edge's direction in two forms.
The census's exact sweeps read `EdgeGeo` (`crates/topo/src/census.rs:348`),
whose direction is the difference of the edge's two vertices
(`line_edges`, `census.rs:2377`). The ray's edge pass (`line_passes`,
`census.rs:11493`) reads the edge's stored line (`edge_line_span`,
`census.rs:2354`), since PR 6958.

**Why it was left.** The PR's author: a direction taken from the carrier
with ends taken from the vertices disagrees with itself by the body's
tolerance, and the census's sweeps read from both ends and intersect.
Not measured.

**Why it matters to this track.** Over a parameter box a vertex
difference carries the box twice, where a stored line's direction does
not once the sweep doors store the direction they were handed
(`the-extrude-mints-its-plane-normals-and-a-cylinders-u-ref-through-arithmetic-over-the-box`).
Until then the two agree: the door mints an edge's line from its two
boxed ends.

**What closes it.** After the doors store exact edge directions: measure
which census readings over a boxed door body are wide only because
`EdgeGeo` reads two ends, and either read the stored line there or say
why the two-end reading is the right one for the exact sweeps.

## Evidence (2026-11-15, the ray-readings unit's sweep)

Swept for "a difference of two points that share a boxed term, then a
three-component dot, cross or norm" over `census.rs`'s production code
(branch `boxfinish/ray-readings`). Read from the code, none measured
over a box. Beside `edge_geo`'s `chord = p1 − p0` (the source of
`e.dir` in `wedge`, `lines_frame`, `parallel_reading`, `meeting_point`,
`overlap_gap` and `edge_in_curved_face`), the same shape reads:

- a point of a face against that face's plane origin
  (`geom::plane_offset(f.origin, f.normal, p)`): `pair_holds_point`,
  `pair_holds_edges`, `pair_vertex_face`, `pair_edge_face`,
  `confirm_edge_face`, `distance`, and `meeting_point`'s
  `(f.origin − e.p0)·f.normal`;
- two points of one edge: `pierce_inside` (`chord`, `q_hi − q_lo`),
  `edge_crossings` and `meetings` (`b − a`), `arc_interiors_meet`,
  and `pierce_span`'s `r0 − r1` (a scalar difference of two residuals
  that each carry the plane's origin);
- two stored vertices or ends: `pair_vertex_vertex`,
  `confirm_vertex_vertex`, `any_boundary_vertex_at`,
  `pair_vertex_curved_edge`, `line_segments_meet`, `overlap_gap`,
  `meeting_point`'s `a.p0 − b.p0`, `half_edge_tangent`, `round_face`;
- a cover against its axis origin after two projections:
  `curved_same_side_pairs` (`lo − base`).

`ee_lines_closest` is read by the ray too (`line_passes`). There its
four end pairs are intersected (`both_enclose`), so the lower end of a
gap is the best of the four and a ray clear of an edge stays clear;
measured on the door's slab, it is not what stops a ray.

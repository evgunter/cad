---
id: a-pierce-ring-on-a-one-face-torus-face-reads-undecided
kind: issue
title: A pierce ring on the one-face torus's face reads RingHomingAmbiguous: no path from it decides
status: closed
opened: 2026-10-12
closed: 2026-10-12
branch: germ/one-face-torus-pierce-ring
pr: 4910
---


## What

The one-face torus `R = 2`, `r = 0.5` about `y` with its vertex on the
inner equator (`germ_plane_torus_frame::one_face_torus(2.0, 0.5, π)`)
against the box `[0, 6] × [h, h + 12] × [−6, 6]` (`x ≥ 0 ∧ y ≥ h`)
refuses `Join(RingHomingAmbiguous)` in all six ops at `h = −0.1`,
`−0.3` and `−0.45` (default ε). Measured on PR 4847's fix pass by
instrumenting the three raise sites in `crates/topo/src/chord_join.rs`:
the refusal is `ChordJoiner`'s re-homing (`rehome_rings`'
`RingSide::OnRun | RingSide::Undecided` arm, `chord_join.rs` :3941),
on a **torus** face, through `path_ring_side` (`chord_join.rs` :3234),
which read `RingSide::Undecided`. The ring is a pierce ring
(`is_pierce_ring` true) at one point, `(0, h, −(R + √(r² − h²)))`:
the box's edge `x = 0, y = h` piercing the torus's outer side.

So a public pose does reach the ring lane on a torus face; PR 4847's
body said none did.

The same box builds in all six ops against the donut, the fat torus
`(1, 0.9)`, the seamed torus and the one-face torus at phases 0 and 1,
over `h` from `−0.9r` to `0.95r` (scaled to the fat torus's tube). At
phase 0, `h = 0` refuses `CurvedPierceUnsupported` instead (the
equator edge in the box face).

## Not yet known

Why no path decides. `path_ring_side` asks paths from the ring's
point to `outer_references` of the divided face (its outer loop's
vertices and the midpoints of its non-run edges); on this face those
lie on the wrap meridian and the wrap parallel through the vertex, and
each `Quadric::paths` path ends round its reference's own meridian or
parallel, which may run along those wrap circles. That reading is
unverified: phases 0 and 1 have the same shape of references and build.

## Done when

The pose builds its true body in all six ops (volume, tier 3,
`point_in_solid`), or refuses with a typed cause that names why the
ring cannot be placed.

## Measured and fixed (`germ/one-face-torus-pierce-ring`)

At `h = −0.3` the divided face's run is four arcs: the inner parallel
of `y = h` and the meridian circle of `x = 0` (both box faces'), and
pieces of the wrap meridian and the wrap parallel. The ring point
`(0, h, −2.4)` lies in the planes of the box faces' two arcs: on the
other loop of `y = h`'s pair of parallels, and on the `x = 0` meridian
circle itself, off the arc's span. Every reference of the face's outer
loop lies on the wrap edges' circles, which carry the other two arcs.
So each of the 768 paths had a piece running along an arc's plane: the
lead (zero length, from the ring point), the ring point's meridian
(in `x = 0`), its parallel (in `y = h`), or the reference's meridian or
parallel (a wrap circle). `split_ring_path_meets_plane` or
`split_ring_path_segment_side` read Zero on each, and every path said
nothing.

`ring_path::path_parity` now reads such a piece apart from the arc
where it is decided so (`misses_in_plane`: its circle clear of the
arc's annulus, or the arc's own circle with disjoint spans; a lead
decided farther from the arc than half its length). Every op builds
its true body at all three `h`
(`germ_plane_torus_chord::a_box_edge_piercing_a_one_face_torus_builds_its_true_body`).
The two-face donut's bar `[−3, 3] × [0.2, 1.2] × [0, 3]`, torus first,
was the same defect (its run's two arcs lie in the plane `z = 0`, the
ring point on that plane's other loop) and builds too
(`…::a_bar_on_the_donuts_meridian_plane_builds_its_true_body`).

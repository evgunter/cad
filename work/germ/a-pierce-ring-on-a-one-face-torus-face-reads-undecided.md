---
id: a-pierce-ring-on-a-one-face-torus-face-reads-undecided
kind: issue
title: A pierce ring on the one-face torus's face reads RingHomingAmbiguous: no path from it decides
status: open
opened: 2026-10-12
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

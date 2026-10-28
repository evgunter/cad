---
id: a-plane-tangent-to-a-rim-at-its-wrap-edges-vertex-has-no-join-arm
kind: issue
title: A boolean whose plane touches a one-face wall's rim at its wrap edge's vertex refuses SingleSiteSectionLoop where split builds the same cut and the two-arc form builds the boolean
status: open
opened: 2026-10-11
priority: P2
cost: H
---


Found by CLEAVE (`a-split-through-a-wrap-edges-vertex-cuts-the-wall-along-it`,
branch `cleave/wrap-edge-split`).

## What

A one-segment circle (r = 1 about the z axis, its vertex at azimuth
π/2, sweep −2π) extruded over `z ∈ [0, 2]` is one wall whose strut is
its wrap edge. Tilt it 0.4 rad about x and take the half-space brick
`z ≤ z₀` whose top face passes through the bottom vertex
(`z₀ = sin 0.4`). That face touches the bottom rim at the vertex, and
the section ellipse on the wall meets the strut only there, so the
section loop has one site: the wrap edge's end vertex, where the rim
also meets the plane.

All four ops (A − B, A ∩ B, A ∪ B, B − A) refuse
`Join(SingleSiteSectionLoop { count: 1 })` at ε = 1e-9, 1e-6 and
1e-12, and so do the same poses with the face moved ±0.6ε and ±0.9ε.
Measured against two other forms of the same cut:

- `topo::split` by that plane builds, one wall face with the strut as
  its wrap edge above, the closed-form volume, all three tiers
  (`crates/sweep/tests/a_split_through_a_wrap_edges_vertex.rs`, pose
  "round, leaning to the vertex");
- the two-arc circle (vertices at azimuths π/2 and −π/2) builds the
  boolean, because the far strut gives the section a second site.

Pinned as it stands by
`a_split_through_a_wrap_edges_vertex::a_boolean_through_a_strut_vertex_keeps_one_face_where_the_piece_goes_round`
(the `single_site` rows) and by
`crates/sweep/tests/a_plane_across_a_one_face_wall.rs`
`a_plane_tangent_at_the_wrap_edges_end_refuses`. Both go red when an
arm builds it.

## Where to look

PR 4345's lone-site arm (`boolean/join.rs`, `join_lone_ring`) takes a
one-site record only where the site is a wrap edge piercing a planar
face. Here the site is the wrap edge's vertex, where the rim (a closed
edge on that vertex) touches the face. The split's join already pairs
a face's two halves at one site into a self-loop chord
(`closed-in-face-section-loop-has-one-site`, "Also seen by CLEAVE"),
which may be the shape of the arm. Once circles lower to one segment,
this is any plane tangent to a round boss's rim at its profile vertex.

## Moved to SECTTAIL (2026-10-25)

From SECT, cut on its priority seam (SECT measured 46 budget points against 31) in a chat-approved refold. Read SECT's step 3 first: SECT's plan says several of these may close by retirement then.

## The hole's form of the pose (2026-10-28, SPLITDOOR `splitdoor/boolean-strut-order`)

The same pose on a hole: the block `[-2, 2]² × [0, 2]` with the
one-segment hole (`holed_block(π/2, ±2π)` in
`crates/sweep/tests/a_split_through_a_wrap_edges_vertex.rs`), turned
0.4 about x against the brick through the turned vertex. Once the
pierce ring orders its two germs along the rim's tangent (that
branch), A − B, A ∩ B and A ∪ B refuse
`JoinDesync { what: "derived ring role order separates a loose
scaffolding pair" }` (`join.rs` `ring_order`) at ε = 1e-6, 1e-9 and
1e-12, either winding. The two-arc hole and the one-segment hole at
`t = 0.9` (whose section crosses the top rim, a second site) build
sound at their closed forms. Pinned as it stands by
`a_boolean_tangent_to_a_holes_rim_at_a_vertex_builds_sound` (its
`held` rows), red when it builds.

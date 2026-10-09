---
id: a-touch-at-a-saddle-corner-refuses-unanalysed
kind: issue
title: A touch at a saddle corner (an L's inner corner) that no plane separates refuses TouchUnanalysed, so a block seated in that corner on the floor cannot certify
status: open
opened: 2026-09-25
priority: P2
cost: D
---


Filed by CONTACT-1, as a class its sufficient test refuses by name.
The census's touch analysis (`crates/topo/src/census.rs`,
`touch_verdict`) certifies a rest by a separating plane through the
touch point, or by the complement test when one cone's complement is
convex. A SADDLE cone — a vertex with both convex and reflex edges, or
a face sector of 180° or more, such as the L-bracket's inner corner
`(1, 1, 0)` — is decided by neither when no plane separates the two
cones, and the touch refuses as `Undecided::TouchUnanalysed`.

Witness: `contact1_touch_cones::a_block_on_the_floor_of_the_inner_corner_refuses_unanalysed`
(`crates/topo/tests/contact1_touch_cones.rs`) — a block `[1, 2] × [1,
2] × [0, 0.5]` seated in the corner on the floor: its corner coincides
with the saddle vertex, the cones' interiors are disjoint, and the
pair refuses. The same block lifted off the floor clears (the vertical
reflex edge's wedge IS co-convex).

The exact fix is interior-disjointness of two spherical polygons, or a
convex decomposition of the saddle cone (the pieces cut by the planes
of the faces at its reflex edges) with the pairwise separating-plane
test, which is exact for convex pieces. Difficulty M.

**A reader to build on** (TANG, `tang/non-convex-partner-cones`):
`crates/topo/src/boolean/sectors.rs` `cone_side` reads a direction's
side of any polygon cone at a vertex — a dart's apex, a saddle — by
the parity of the face sectors a great arc to a reference inside one
face crosses, each decision a point deviation
(`a_saddle_corner_reads_as_its_polygon_cone`). It classes directions,
not cones against cones; interior-disjointness of two cones still needs
the crossing test between their links.

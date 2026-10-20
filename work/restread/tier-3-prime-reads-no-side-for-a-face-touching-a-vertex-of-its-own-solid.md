---
id: tier-3-prime-reads-no-side-for-a-face-touching-a-vertex-of-its-own-solid
kind: issue
title: Tier 3′ reads no side where a face of a solid touches one of its own vertices, and the literal D1 corollary would refuse a cavity's corner on a face
status: open
opened: 2026-10-19
priority: P2
cost: M
---


Found by `census-passes-a-face-inside-an-edges-material-wedge`'s lane
(branch `restread/face-in-edge-wedge`), measured there.

## What

Pass 3 (`census.rs` `pair_vertex_face`) backs a vertex lying inside a
face of its own solid on its record alone and reads no side. The edge
half of the D1 check (`edge_in_own_face`) does not reach it.

The corollary PR 4795's designers stated, "a face lies outside a
vertex's material cone", was tried on that branch: the vertex's link
(its faces at that corner) against the face's, through `rest::judge`,
on every backed vertex-on-face pair inside one solid. It refused:

- the slab `[0,4]² × [0,1]` less a pyramid whose apex rests on the
  floor (base `[1,3]²` at `z = 2`, apex `(2, 2, 0)`): one solid, one
  vertex-on-face record;
- `union_flush_onto_edge_contact::a_dangling_null_edge_holding_another_pairs_cut_builds_in_every_op`,
  `cube ∖ y`: the spike's far corners on the cube's faces `x = 1` and
  `y = 1`, from the cavity's side.

Each is a cavity whose corner touches a face from inside the material.
The material about the point is the half-space less a cone touching it
at the point: one piece, whose link is an annulus with two boundary
circles. The cone's circle is the vertex and the plane's is the face's
interior, which is the only manifold form that solid has. Restread's
closed `a-solid-touching-itself-at-a-vertex-reads-its-star-from-the-vertex-alone`
already reads such a plate (a pyramidal void's apex on its top) as a
valid body with its own record. The vertex's link and the face's
materials overlap there, so the literal "opposite sides" refuses it.
On a line the same configuration is two material wedges, the slit
form, and every overlap is a defect. At a point it is not.

## The fork (for Ev, weighed by the designers first)

1. **Materials on opposite sides, as written**: refuse the cavity
   corner too, which leaves the pit and `cube ∖ y` with no valid form.
2. **Either the materials or the voids on opposite sides**: a convex
   corner rests on a face from outside, a cavity's corner from inside.
   This refuses a face crossing the vertex's faces, and a corner whose
   sides disagree with the face's (a convex corner inside the face's
   material, a cavity's outside it). The D1 sentence would need to say
   which a point admits.
3. **Leave the vertex unread**: an edge of the vertex lying in the face
   is read by `edge_in_own_face`, and a vertex's faces crossing the
   face are a boundary crossing other arms may catch (unmeasured).

The same question stands for a vertex on an edge's interior (pass 2,
`ve_face_backed`) and two vertices of one solid on one point key
(pass 1).

## A missing cross-solid finding beside it

Arm 2's `blocks` (`census.rs`, `Named::of` and the closure in
`sweep_cross_solid_backstop`) maps a vertex-on-face touch inside one
solid to no block. Two planar faces of one solid can cross showing
only such events: face A, the triangle `(0,0,0), (4,1,1), (4,-1,-1)`
in `y = z`, and face B, `(2,0,0), (-2,2,0), (-2,-2,0)` in `z = 0`,
cross along `[(0,0,0), (2,0,0)]` with no edge piercing or crossing,
each with a vertex in the other's interior. A `FaceThroughEdgeWedge`
whose wedge lies inside the face's material forces such a
self-crossing elsewhere, so where it takes this form no finding blocks
the solid's pairs in arm 2. The body is still refused by its own
`FaceThroughEdgeWedge`, and by `UnrecordedContact` unless the touches
are recorded; the gap is the secondary cross-solid finding, not a
silent pass. `sweep/tests/common/differential.rs`'s `Verdict::Rider`
judge reads a body refused by `FaceThroughEdgeWedge` alone as a rider,
so it would hide that finding's absence.

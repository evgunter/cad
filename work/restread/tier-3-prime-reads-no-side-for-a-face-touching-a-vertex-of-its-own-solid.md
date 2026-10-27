---
id: tier-3-prime-reads-no-side-for-a-face-touching-a-vertex-of-its-own-solid
kind: issue
title: Tier 3′ reads no side where a face of a solid touches one of its own vertices, and the literal D1 corollary would refuse a cavity's corner on a face
status: closed
opened: 2026-10-19
priority: P1
cost: M
closed: 2026-10-25
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
its face rung, `Declared::confine`) and two vertices of one solid on one point key
(pass 1).

## A missing cross-solid finding beside it

Arm 2's `blocks` (`census.rs`, `Named::of` and the closure in
`sweep_cross_solid_backstop`) maps a vertex-on-face touch inside one
solid to no block. Two planar faces of one solid can cross showing
only such events: face A, the triangle `(0,0,0), (4,1,1), (4,-1,-1)`
in `y = z`, and face B, `(2,0,0), (-2,2,0), (-2,-2,0)` in `z = 0`,
cross along `[(0,0,0), (2,0,0)]` with no edge piercing or crossing,
each with a vertex in the other's interior. A `FaceThroughEdgeWedge`
whose wedge lies inside the face's material forces a self-crossing
elsewhere, and where that crossing takes this form no finding blocks
the solid's pairs in arm 2. The body is still refused by its own
`FaceThroughEdgeWedge`, and by `UnrecordedContact` unless the touches
are recorded; the gap is the secondary cross-solid finding, not a
silent pass. No test asserts that finding:
`sweep/tests/common/differential.rs` reads every tier 3′ refusal as
`Bad`. A judge that admits a body refused by `FaceThroughEdgeWedge`
alone, as the `Verdict::Rider` this branch carried until the slit
parting reached it did, hides the finding's absence.

## Measured on main (2026-10-21, `d1e5e7f3e3`)

A single solid whose two planar faces cross, showing only vertex-on-face events, passes tier 3′ once its touches are recorded. This is a silent wrong pass, not only a missing secondary finding.

**The crossing body.**
- Two interlocked tetrahedra in one shell, joined through a triangular tube between holes in faces away from the crossing:
  - P = (0,0,0), (4,1,1), (4,−1,−1), (−10,5,−1);
  - Q = (2,0,0), (−2,2,0), (−2,−2,0), (8,1,2).
- Faces A and B are the witness triangles above.
- The body has 20 vertices, 17 faces and genus 0, and passes tiers 1–3.

**Tier 3′ readings.**
- Without records: exactly four `UnrecordedContact { VertexOnFace }`, at (0,0,0), (2,0,0), (4,1,1) and (−2,2,0).
- With those four recorded: `Ok(())`.
- Control: Q lifted by 0.125 refuses four `EdgeFacePierce`.
- An exact check of every cell pair finds only the four vertex-on-face-interior events.

**The pit.** The slab `[0,4]² × [0,1]` less a pyramid with its apex resting on the floor: tier 3′ refuses `UnrecordedContact` without its record, and returns `Ok(())` with it.

Both were built through step-import's `assemble::build_one_solid` from generated STEP. The generators are probe scripts in the orchestrator's scratch, not committed.

## The design question, weighed

Two designers weighed it and converged after two rounds. The `[ev]` PR states the clause in `crates/topo/README.md` tier 3: the whole-link winding at a point where one solid touches itself.

## Ev's ruling (2026-10-21, PR 5517)

**Decision 1, accepted:** "decision 1 makes sense". At an isolated point where one solid touches itself, the solid's whole link there bounds one material, so its winding takes two consecutive values. It is now in `crates/topo/README.md` D1 tier 3, and is what this row builds.

**Decision 2, no opinion:** "we can keep doing it but i don't want it to seem like it's a solid decision". The cells sentence stays out of the README. It describes only what the kernel builds today, so nothing should cite it as ratified. As the designers corrected it:
- the cells at such a point are one per boundary circle of the material;
- a circle inside one face is that face;
- a circle creased only along an edge through the point is that edge;
- any other circle is a vertex.

**What to build (designers' sketch, for the lane):**
- a winding reader beside `Link::reads`;
- one unit per (solid, point) wherever ties or shared keys name two or more cells of one solid;
- a finding for the failure, mapped in `Named::of` as a pierce of the solid;
- the crossing body and the pit as witnesses;
- the two-pits-and-a-needle body as a control that must pass.

## Closed

2026-10-26 (branch `restread/point-touch-winding`, PR 5704). Tier 3′
reads each point where one solid touches itself once, over the solid's
whole link (`sweep_self_touch_points`, `rest::one_material`). Its
units are the vertex-vertex, vertex-on-edge and vertex-on-face ties and
the point keys that name two cells of one solid; a line touch reaching
the point is read with it, every sheet through the point included. No
sheet may cross another, and every piece's front winding must be equal,
so the winding takes two consecutive values; a link of one sector with
no reading in band rests (one sheet alone bounds one material, and the
tie loop cannot read a lone plane). A decided failure refuses
`BoundaryCrossesItself`, which arm 2 reads as a pierce of the solid,
and a planar link read in band, or not built, refuses
`BoundaryCrossesItselfUndecided`, which leaves the solid unexamined.
That closes the cross-solid gap above.

- **Witnesses** (`census::self_touch`): the crossing tetrahedra refuse
  with their four records, and the lifted control refuses four
  `EdgeFacePierce`. The pit, two pits beside a needle and the mould of
  a pyramid on a plate (its touch recorded) pass.
- **A link holding a curved face or edge is left unread**, by the
  orchestrator's ruling with Ev (option B, PR 5704): no refusal, a
  silent pass of the class. Filed P1 as
  `tier-3-prime-reads-no-side-where-a-solid-touches-itself-at-a-point-on-a-curved-face`.
- **A face held at a corner and along an edge** at the point is read as
  one sector per pass, so the Boolean union pinched at a crossing
  (`vertex_on_edge_records`) certifies.

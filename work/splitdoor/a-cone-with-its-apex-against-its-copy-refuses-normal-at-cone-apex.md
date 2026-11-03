---
id: a-cone-with-its-apex-against-its-copy-refuses-normal-at-cone-apex
kind: issue
title: A cone with its apex, booleaned with an identical copy, refuses NormalAtConeApex at the apex vertex before the On verdict
status: dispatched
opened: 2026-10-29
priority: P1
cost: H
branch: splitdoor/cone-apex-vertex-pairs
---


Found by the sweep on `splitdoor/curved-copy` (the class: a curved body against a coincident copy).

## What

The triangle `(0,0) (1,0) (0,1)` revolved fully about `y` (a cone with its apex on the body) against an identical copy refuses every op, both orders, at ε = 1e-9:

`NormalAtConeApex { operand: A, face: FaceKey(1v1), .. }`

The cone faces are settled one carrier with their copies (the carrier ladder's cone arm), so the boolean reaches the vertex classification. There the apex of A lies on the apex of B, and the sector lane asks the cone face's tangent plane at that vertex (`boolean/sectors.rs`, the `VertexFace::Apex` arm), which the apex does not have. A frustum, a ring frustum and the lantern against their copies build (`crates/sweep/tests/curved_copy_against_itself.rs`), so the apex vertex is what refuses.

Probe: `curved_copy_against_itself::curved_copy_sweep` (ignored), the "cone with apex" row.

## Where to look

Whether a vertex lying on a vertex of the copy, every face about each settled one carrier with a face about the other, needs a sector reading at all, or can be answered by the `On` verdict as the section pass's settled pairs are (`boolean::ops::OnDeferred`). Sibling row: `boolean-readers-at-a-cone-apex-vertex-escalate-under-their-own-reading`.

## Widened (2026-11-01)

Now P1 / H. This is the owed vertex–vertex unit of the cone-apex decision, recorded on `a-planar-face-through-a-cones-apex-refuses-the-boolean-where-the-split-builds`. Sectors become arcs of circles on the direction sphere, so that apex-on-vertex, apex-on-edge, apex-on-apex and a cone against its copy all build. The witnesses are listed there. It is built after that row's first unit. (SPLITDOOR orchestrator)

## The refusal's new name (2026-11-01, SPLITDOOR, `splitdoor/cone-apex-boolean`)

The first unit moves the refusal out of `boolean::sectors::sector_face`,
which both lanes call, into the vertex–vertex lane. Right after both
sector arrays are built, `sectors::apex_pair_refusal` refuses a pair one
of whose vertices is a cone's apex: `VertexPairAtConeApex { operand,
face }`. The pierce lane reads the apex. `build_sectors` takes the
plane an apex corner is subdivided against as `sectors::ApexPlane`,
`None` or `One(normal)`. The vertex–vertex lane passes `None` today:
ordering two planes' rulings round the axis is this unit's decision,
and this unit widens the enum.

Measured at the first unit's head (one-wrap-edge 45° cone, ε 1e-9):

- its apex against its copy's apex, turned over about it, refuses
  `VertexPairAtConeApex` in ∪, ∩ and ∖, pinned in
  `crates/sweep/tests/a_cone_apex_on_a_plane_through_it.rs`
  (`a_tipped_cones_apex_on_its_copys_refuses_the_vertex_pair_by_name`);
- a box whose corner or edge sits at the apex refuses earlier,
  `CrossingAtConeApex`, at the box's edge through the apex.

## Stage B1: the vertex–vertex lane (2026-11-02, `splitdoor/cone-apex-vertex-pairs`)

Unit B lands in two stages; this row closes with the second.

B1 builds the vertex–vertex lane at an apex:

- a sweep edge ending at the other operand's apex meets it at that end
  alone, the vertex pair's incidence (`boolean::reduce::line_cone_roots`);
- the fan arms of the pair algebra (`boolean::fan`);
- sectors cut at their partner faces' extremes (`boolean::sectors::Partners`);
- two cones through one apex meet in their common rulings, or touch at it
  (`boolean::section_cert`'s cone pair, `boolean::join`'s `KindArm::Apex`).

`VertexPairAtConeApex` now names only two things: the readers with no fan
arm, and a whole-turn cone vertex whose partner is not one cone's fans.
Pinned in `crates/sweep/tests/a_cone_apex_meets_a_vertex.rs`:

- the copy, both forms;
- the copy turned over;
- hourglasses, coaxial and tilted;
- a box corner at the apex, turned;
- two cones through one apex crossing along two rulings.

Left for B2:

- the apex on a curved edge or pierced against a curved face (the
  seam-meridian sphere pose, an apex inside a sphere face);
- a box edge through the apex.

The small box `[0, 0.3] × [0.5, 1] × [0, 0.35]` with its corner at the
45° cone's apex reaches the join and refuses `GermSectionOpenConic`.
That is the hyperbola join limit, which already stands and which this
unit does not touch. At 30° it builds.

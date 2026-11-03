---
id: a-spline-wall-holding-a-pierce-point-has-no-operand-that-reaches-it
kind: issue
title: A spline wall read holding a pierce point (HeldPierceUnbuilt, placed: true) has no operand that reaches it: each join that would make one refuses first
status: open
opened: 2026-11-03
priority: P2
cost: M
---


Filed by the `a-spline-wall-cannot-say-whether-it-holds-a-pierce-point`
lane (`splice/held-pierce`).

## What

`boolean::vtxfac::held_reads` (inside `faces_holding_pierce`) refuses
`HeldPierceUnbuilt { placed: true }` for a spline face of the piercing
body whose trim places the pierce point `In`: the face has no residual
to put the point on its carrier and the arm reads no normal there. The
doors the arm needs exist: `contain::curved_face_placement_decided`
returns the in-trim foot of an `In` placement,
`face_normal::spline_outward_normal_at` reads the outward normal at
that chart point, and the wall's bend is `T::spline_chart_arm` (as
`PierceLever::Within`). The placement's uniqueness on the wall is
`the-spline-placement-read-certifies-no-second-in-trim-foot`.

No input reaches the arm, so it stays a typed refusal. The wavy loft
(`editor-core` `emit_topo`'s
`wavy_loft::a_union_across_a_nurbs_edge_refuses_at_the_spline_walls_germ_frame`)
reads its spline walls `OffCarrier` at both of its pierce points and
goes on to the join.

## The shape that reaches it

A held pierce is a triple point: the pierced face, a face of the
piercing body the piercing vertex does not bound, and the vertex. So the
piercing operand has two lumps touching where the point lies: one
lump's edge or vertex lies in the other's spline wall, inside its trim,
and the other operand's face passes through that point (the analytic
rows are `sweep`'s `a_pierce_held_in_a_curved_face`, which join a
square prism along a cylinder's ruling). The operand is made by a
union, and every union that makes one with a spline wall refuses
before it is built. Measured on `splice/held-pierce` (2026-11-03), a
loft of three 2 × 2 squares at z = 0, 1, 2 (straight, and with the
middle one offset 0.3 in x), whose wall y = −1 is a planar NURBS face:

- **An edge lying in the wall**: a diamond prism whose vertical corner
  edge lies along x = 0 in y = −1, joined to the loft, refuses
  `SplineCrossingUnsettled { operand: B, face_of: A, why: Floor }`. The
  sweep's curve × spline face reading cannot settle a line lying in
  the face.
- **A vertex touching the wall, and nothing else meeting**: a pyramid
  whose apex sits on the wall at (0, −1, 1) refuses
  `NurbsExtentUnsupported { operand: A }`. A union with no crossing
  takes the no-crossing path in `boolean::ops` (the NURBS re-gate),
  which refuses any NURBS face that the other operand's boxes reach.
- **That vertex, with a box that crosses only the loft's plane top
  cap**, so that the union crosses something: it refuses
  `Containment(KindUnsupported { kind: Nurbs })`. The point-in-solid
  ray reading (`boolean::solid_contain`) has no NURBS arm.

## What it needs

One of those three joins built, then the arm built on the doors above,
with a row through a plate whose top passes through the touching
point. Until then the refusal stays typed.

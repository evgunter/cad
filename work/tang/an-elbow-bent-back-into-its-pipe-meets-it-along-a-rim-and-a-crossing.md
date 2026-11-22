---
id: an-elbow-bent-back-into-its-pipe-meets-it-along-a-rim-and-a-crossing
kind: issue
title: An elbow revolved off a pipe's cap and bent back into the pipe overlaps it: the rim is a nested touch, the carriers cross beside it, and no overlap arm builds it
status: open
opened: 2026-11-22
priority: P1
cost: M
---


Found by HELDRECORDS' elbow lane (2026-11-22), filed here because the
pair is an overlapping tangency at a rim: the nested-touch side of the
rim routing and the overlap the membership decides, TANG's ground.

## Measured

The pipe of
`crates/editor-core/tests/a_pipe_unions_with_its_elbow.rs` (the ring
`circle(1)`, `circle(0.6)` extruded 2 along z) and the same loops drawn
on a cap and revolved a quarter turn about a line in the cap's plane
three units off the pipe's axis. Four of that file's eight poses bend
the elbow back INTO the pipe: off the End cap (z = 2) a turn of +π/2
(the elbow spans z ∈ [−2, 2]), off the Start cap (z = 0) a turn of
−π/2 (z ∈ [−4, 0]). For these the two outer walls leave the shared rim
on ONE side, so `rim_wedge::departures` should read `Same` and the
glue door leave the rim unglued (`TangencyFailure::Unrouted`; expected
from the geometry, not traced), and the bend runs through the pipe's
material: its outer torus crosses the pipe's cylinder beside the rim.

Today each refuses at the crossing layer
(`reduce::curved_face_arm`, `BooleanError::CurvedPierceUnsupported`,
pinned in `a_pipe_and_an_elbow_off_either_cap_pass_the_glue_door_in_every_pose`)
— the same wall the outward poses meet, an edge ending on the rim
tangent to the other's wall. Once that wall has its reading (the
elbow lane's next PR, `heldrecords/elbow-ends`), these poses owe a
true answer of their own: the union built by the overlap lane at its
closed form (the pipe's volume plus the elbow's less their shared
region), or a typed refusal naming the unbuilt overlap arm, with a
recourse that can be followed.

## What is owed

The overlap arm for two operands that share a rim as a nested touch
(normals agreeing, one departure side) while their carriers cross
beside it, or the typed refusal that names it. Not measured: where
the membership and the section walk stop on this pair today, behind
the crossing layer's refusal.

---
id: boolean-gate-refuses-a-pair-the-census-reads-apart
kind: issue
title: The boolean operand gate and the NURBS extent test refuse a face pair as within reach where the census's nearest-point directions and held sphere reach would part it
status: open
opened: 2026-10-26
priority: P1
cost: M
---


Found by RESTREAD's sweep for readers that refuse a pair only because
it is within reach (`restread/curved-pairs-apart`). Read off the code;
not measured.

## What

Two boolean doors refuse a face pair that `boolean::separating::apart`
does not part:

- the operand gate (`boolean/reduce.rs`, the `UnsupportedPair` scan:
  `boxed.overlaps(other_box) && !covered(..) && !apart(..)`);
- the NURBS extent test (`boolean/ops.rs`, the
  `NurbsExtentUnsupported` scan over a NURBS face's rows).

Both read `apart` along the anchor axis and the operands' planar
normals only. The census's arm 1 now reads two more things through
`separating::apart_by`:
- the offset from an axis or centre either face proposes to the other
  face's nearest boundary segment point (`census::depth_gap::toward`);
- a sphere face's reach, cut back to its boundary's at an end whose
  pole the face places `Out` (`census::held_reach_in`).

That second reach places a point in the face (`curved_face_placement`).
The census does that at rest, but a boolean door also reads bodies
mid-operation. So the first question is whether the placement is sound
to call there.

## Measure first

Take as operands, against a NURBS or torus partner the gate does not
serve:
- a box beside a rod's wall, 0.04 clear on the diagonal;
- the two lumps of a bar cut through a ball.

Both shapes are in `sweep::cross_solid_apart`. Measure which refuse,
then decide whether the two doors should read `apart_by` with the
census's reach.

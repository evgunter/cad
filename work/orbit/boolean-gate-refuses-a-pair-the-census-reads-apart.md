---
id: boolean-gate-refuses-a-pair-the-census-reads-apart
kind: issue
title: The boolean operand gate, the NURBS extent test and the split's carrier gate refuse as within reach what the census's nearest-point directions and held sphere reach would part
status: open
opened: 2026-10-26
priority: P1
cost: M
---


Found by RESTREAD's sweep for readers that refuse a pair only because
it is within reach (`restread/curved-pairs-apart`). Read off the code;
not measured.

## What

Three pair doors refuse on reach alone:

- the operand gate (`boolean/reduce.rs`, the `UnsupportedPair` scan:
  `boxed.overlaps(other_box) && !covered(..) && !apart(..)`);
- the NURBS extent test (`boolean/ops.rs`, the
  `NurbsExtentUnsupported` scan over a NURBS face's rows);
- the split's carrier gate (`splitting/classify.rs` `carrier_gate`):
  a sphere, torus, NURBS or Approx face refuses
  `CurvedBooleanUnsupported` unless its reach along the plane's normal
  (`gate_face_reach`) clears the plane (`reach_clears`).

A fourth refuses a point, not a pair, on reach: point-in-solid
(`boolean/solid_contain.rs` `past_every_reach`) keeps its
`KindUnsupported` refusal for a selection holding a NURBS or Approx face
unless the point lies past every face's reach along a turned direction
(`separating::support_along`). A sphere face beside the spline one is
read there by `face_reach_in`, so the held pole would sharpen it too.

The first two read `apart` along the anchor axis and the operands'
planar normals only. The split gate reads one direction, the plane's
normal, so no direction family helps it; a sharper reach does. The census's arm 1 now reads two more things through
`separating::apart_by`:
- the offset from an axis or centre either face proposes to the other
  face's nearest boundary segment point (`census::depth_gap::toward`);
- a sphere face's reach, cut back to its boundary's at an end whose
  pole the face places `Out` (`census::held_reach_in`).

That second reach places a point in the face (`curved_face_placement`).

Since `restread/arm1-third-face-directions` arm 1 also reads, through
`census::depth_gap::across`, each arc's chord normal on either face's
boundary and the line between the two faces' carrier axes or centres:
a sliver off a block's corner, the two pieces a lens cuts off a block,
and two parallel walls apart off their seams
(`sweep::cross_solid_apart`). These are geometry of the pair alone, so
the first two doors could read them as they are; the measurement below
should take those three shapes too.

## One home for the sharper reaches

Two private sharpenings of `census::face_reach_in` now exist:
- `splitting/classify.rs` `gate_face_reach`, a torus face's reach read
  over its chart window in closed form;
- `census::held_reach_in`, a sphere face's reach cut back to its
  boundary at an end whose pole it places `Out`.

Each serves one caller and each is sound for every caller of
`face_reach_in`. They should become one reach, with the torus window and
the held pole as two of its per-kind rules, read by all these doors.
Whether a door that reads bodies mid-operation may call the placement
the held pole needs is the question below.
The census does that at rest, but a boolean door also reads bodies
mid-operation. So the first question is whether the placement is sound
to call there.

## Measure first

For the boolean doors, take as operands, against a NURBS or torus
partner the gate does not serve:
- a box beside a rod's wall, 0.04 clear on the diagonal;
- the two lumps of a bar cut through a ball.

Both shapes are in `sweep::cross_solid_apart`. Measure which refuse,
then decide whether the two doors should read `apart_by` with the
census's reach. For the split gate, cut the ball-less-a-slab patch of
`cross_solid_apart::a_trimmed_sphere_face_holding_its_pole_clears_only_past_it`
by the plane `y = 0.5`: past the patch's rim at `y = 0.1`, inside the
whole sphere's reach, on the side whose pole the patch does not hold.

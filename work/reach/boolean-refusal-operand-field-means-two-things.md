---
id: boolean-refusal-operand-field-means-two-things
kind: issue
title: topo: BooleanError's operand field names the face's operand at some raise sites and the edge's at others
status: open
opened: 2026-09-22
refs: [error-and-check-text-overflows-its-region]
---


## What (measured)

`BooleanError::CurvedBooleanUnsupported`'s `operand` is documented as
"the offending operand and face", and the old `Display` rendered it as
"face F of operand X". At one raise site that sentence is false.

- **Measured:** `topo/tests/review_m3_pr4.rs`
  `nurbs_wall_boolean_surfaces_the_crossing_layer_refusal` puts the
  NURBS wall on operand **B** (`boolean_reduce(Union, &a, &b)` with
  `b`'s first face swapped to a NURBS placeholder). The refusal
  carries `operand: A`. The raise site is `reduce.rs`
  `curved_face_arm`, which passes `operand: x_is` (the operand of the
  EDGE being swept) with `face` looked up in `y` (the OTHER operand).
- The same field at `ops.rs` (the extent scan's
  `CurvedBooleanUnsupported`) is `x_is.other()` with `face: yf`, i.e.
  the face's own operand. So one field carries two meanings.
- `BooleanError::ArcLoopContainmentUnsupported` has the same split:
  its doc says "the operand whose face carries the loop", but
  `reduce.rs` `esc(e, x_is)` (the two `contfp(y, face, …)` calls in
  the vertex-placement arms) and `ops.rs` (`operand: x_is` around
  `contfp(y, yf, …)`) pass the edge's operand, while `reduce.rs`'s
  other two `esc(e, x_is.other())` calls pass the face's.

## What the concision PR did about it

Nothing to the payload (its fence was prose only). It stopped the two
`Display`s from naming an operand at all, so no sentence the viewer
shows is false; each `Display` carries a comment saying why.

## The repair

Pick one meaning (the face's operand is what both docs promise) and
make every raise site pass it, then let the `Display` name the operand
again ("the second solid's nurbs face"), which is information the
person holding the mouse can use. A test that puts the curved face on
B and asserts `operand: B` at each raise site is what could go red.

---
id: boolean-refusal-operand-field-means-two-things
kind: issue
title: topo: BooleanError's operand field names the face's operand at some raise sites and the edge's at others
status: open
opened: 2026-09-22
refs: [error-and-check-text-overflows-its-region]
cost: E
priority: P3
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

## 2026-10-03 — the measuring row moved (REACH)

`boolean_reduce` takes finished operands
(`boolean-door-adopts-the-finished-body-type`), and the NURBS-walled
brick does not finish: the row is
`a_placeholder_nurbs_wall_is_refused_at_rest`, pinning
`UncertifiableSurface` and the four `DescriptionNotAdjacent` at the
at-rest gate. No row reaches `curved_face_arm`'s raise with a finished
body; `work/roots/the-operand-gates-curved-arms-have-no-finished-fixture.md`.

## 2026-10-05 — `ArcLoopContainmentUnsupported`'s half: wired at every site, witnessed at two

`reduce.rs` `esc` now takes the face it read, and every caller passes
that face's operand: the two vertex-placement `contfp` calls changed
from `x_is` to `x_is.other()`. The sphere extent scan, now
`ops.rs` `extent_scan_refusal`, passes `x_is.other()` with `yf`. So
`ArcLoopContainmentUnsupported.operand` names the face's operand at
every raise site, as its doc says, and so does the new
`PointInFaceRefused.operand`
(`contain-refusals-on-a-sound-face-reach-the-boolean-as-a-classification-invariant`).
`CurvedBooleanUnsupported` at `curved_face_arm` still has the split.

The fix pass after review keeps that wiring but cannot witness all of
it. Two callers are pinned: `reduce.rs` `esc_tests`'s
`vertex_on_face_refuses_on_the_faces_operand` drives `vertex_on_face`
with A and with B against an `mvfs` seed face, and `ops.rs`'s
`extent_scan_refusals_name_the_faces_operand` drives
`extent_scan_refusal`, which now takes the sphere's operand and
decides the face's. Reverting either to the vertex's or sphere's
operand goes red. The other four sites have no witness: the two
`sweep_direction` calls, `vertex_on_curved_face_at`, and
`carrier_cross.rs` `boundary_crossing`. They are reached only inside a
reduction on a working copy, and no fixture there reaches a refusing
face door (the boolean batteries move no line into these arms). Their
operand rests on reading the code, not on a check.

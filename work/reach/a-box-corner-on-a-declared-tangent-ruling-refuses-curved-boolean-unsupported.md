---
id: a-box-corner-on-a-declared-tangent-ruling-refuses-curved-boolean-unsupported
kind: issue
title: A box whose corner stands on the ruling of a declared Tangent refuses CurvedBooleanUnsupported in both operand orders
status: open
opened: 2026-10-02
priority: P3
cost: M
---

Found while building `a-stack-across-a-mid-edge-tangency-builds-in-one-operand-order-only`,
measured on that unit's branch.

## Repro

The rounded 6 × 4 × 1 plate (`rounded(0.5)` in
`crates/sweep/tests/reach_continuation.rs`) and a unit box of the same
height, turned 45°, its west wall tangent to the south-east fillet
along the ruling at azimuth −45° (the pose of
`a_declared_tangent_beside_a_fillet_builds_in_either_operand_order`),
but placed so a box CORNER stands on the touch point — the box running
from the ruling along +(1, 1)/√2, or along −(1, 1)/√2. The wall and the
fillet declared `Tangent`:

- union, subtract and intersect all refuse
  `CurvedBooleanUnsupported { face: 6v1, kind: Plane }` in both operand
  orders (operand A with the box as A, B with the plate as A).

The same box placed with the ruling in the middle of its wall builds in
both orders, exact (that row in the test above).

## Not yet measured

The payload names one of the box's planar faces; the raising site
among the C7 lump sites (`vtxfac.rs`, `recl.rs`, `sectors.rs`) has not
been instrumented. At the corner three of the box's faces meet the
fillet: the tangent wall along the ruling, the end wall through the
cylinder's axis (transverse), and the bottom/top (continuations of the
plate's). Measure the raising site before designing anything.

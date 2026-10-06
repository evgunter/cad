---
id: split-gate-refuses-a-whole-body-for-one-unarmed-face
kind: issue
title: topo::split refuses a whole body for one sphere, torus or spline face the plane cannot meet; the gate should be box-scoped as the boolean's is
status: closed
closed: 2026-10-02
opened: 2026-10-01
pr: 3843
branch: reach/split-gate-per-face
---

Found by the plane × cone split lane (`plane-cone-elliptic-section-split-refusal`),
which admitted `Cone` at the split's operand gate and was asked whether
the gate itself should become per-face.

## What happens

`splitting::classify::gate_operand` walks EVERY face of the operand and
refuses `SplitReduceError::CurvedBooleanUnsupported` for any kind the
split pipeline has no arm for (`Sphere`, `Torus`, `Nurbs`, `Approx`),
whether or not the plane can reach that face. Its edge loop does the
same for `Nurbs` and `Spiric` carriers (`CurvedEdgeUnsupported`).

Measured: a cylinder of radius 1 and height 1 under a spherical cap
(the arc `(1, 1) → (0, 1.5)` about `(0, 0.25)`, revolved about `y`),
cut by the plane through `(0, 0.5, 0)` with normal `(sin 0.3, cos 0.3, 0)`.
The plane stays inside `y ∈ [0.19, 0.81]`, the cylinder, and never meets
the cap face (`y ≥ 1`). `split` refuses
`Reduce(CurvedBooleanUnsupported { kind: Sphere })`.

## The general form

The boolean retired the same wholesale gate per pair (C12.1, M5 PR 9):
`boolean::reduce::gate_operand_pairs` refuses an unarmed face only when
its certified box (`boolean::boxes::face_box`, padded by `sweep_pad`)
may meet the other operand. The split's twin is: an unarmed face, or an
unarmed edge carrier (`edge_box`), refuses only when the plane may meet
its box — all eight padded corners definitely on one side clears it.
Behind a box that clears, the face has no vertex on or across the plane
and no chord inside it, so the pipeline passes it through whole.

A whole-SURFACE test is not enough and is not the fix: the full sphere
above spans `y ∈ [−1, 1.5]` and meets the plane, while the cap face does
not.

## Why the lane that found it did not do it

`face_box` and `edge_box` take `T: Decide + Bounds`, and the public
`topo::splitting::split` / `split_reduce` are bounded by
`Decide (+ AtRestPolicy)` only. Box-scoping the gate adds `Bounds` to
those public signatures (every scalar in the tree implements it, but
every generic caller re-states the bound), and it sends sphere, torus
and spline faces through split finish and the pcurve mint pass
untouched for the first time, which wants its own rows. The cone half
of the original complaint — a plane missing the cone — no longer
refuses, since the cone is armed.

## Closed (2026-10-02, PR 3843)

`topo::split` now refuses a sphere, torus, spline or Approx face only
when the plane may meet the face's reach box. The box is padded by the
boolean's sweep pad and cleared by one support-function margin
(`split_gate_box_side`). A trimmed sphere face is boxed by its
latitude zone, and only when the face's side of its boundary is
certified; otherwise the whole ball is used. An edge with a spiric or
spline carrier refuses unless its own box or a bounding face's box
clears.

The dual review (DR-44) found no wrong split: 189 admitted planes at
1e-9, all clean. The fix pass added rows that go red under each named
mutant. Filed:
- the world-axis box's pose sensitivity;
- the Approx and spiric rows;
- the sphere zone's fold into `FaceBoxRule`.

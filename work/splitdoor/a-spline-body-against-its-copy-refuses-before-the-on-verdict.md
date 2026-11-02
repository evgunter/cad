---
id: a-spline-body-against-its-copy-refuses-before-the-on-verdict
kind: issue
title: A body with spline faces, booleaned with an identical copy, refuses CurvedBooleanUnsupported or CurvedPairUnsupported before the On verdict
status: open
opened: 2026-10-29
priority: P3
cost: H
---


Found by the sweep on `splitdoor/curved-copy` (the class: a curved body against a coincident copy).

## What

Three spline bodies against identical copies refuse every op at ε = 1e-9, before the no-crossings path:

- `lofted_cylinder(1)`, `loft_prism`, `swept_elbow` (`sweep::test_support`): `∪` refuses `CurvedBooleanUnsupported { operand: B, face: FaceKey(3v1), kind: Nurbs }`, and `∩` and both differences `CurvedPairUnsupported { site: RevertRoster, operand: A, face: FaceKey(3v1), kind: Nurbs, other_face: FaceKey(1v1), other_kind: Plane }`.

The carrier ladder has no spline description (`boolean::carrier_pair::face_carrier` answers `None` for NURBS and `Approx`), so the glue door never settles a spline face with its copy, and the operand gate's pair-scoped curved arms refuse the spline pairs by kind.

Probe: `curved_copy_against_itself::curved_copy_sweep` (ignored), its three spline rows.

## Owed

A spline carrier the ladder can compare (`coincide/the-carrier-ladder-cannot-read-a-spline-carrier-so-coincident-spline-faces-never-merge`), then the same path the analytic copies take: the pair settled one carrier and held for the shells' `On` verdict. The NURBS re-gate in `boolean::ops::sphere_extent_scan` and the pair-scoped gate would each have to admit a settled spline pair.

---
id: an-ovoid-cannot-be-authored
kind: issue
title: An ovoid cannot be authored: a profile has no ellipse arc to revolve, and a body cannot be scaled non-uniformly
status: open
opened: 2026-10-25
priority: P1
cost: M
design: true
---


Banded P1 on request in chat (2026-10-25), from lily wall 4.

## What

A lily bud is an ovoid, a sphere stretched along its own axis, and the
kernel has no way to say one:

- **No ellipse arc in a profile.** `crates/profile/src` has no ellipse
  segment (`git grep -i ellipse -- crates/profile/src` finds nothing),
  so the natural construction, revolving an elliptical meridian arc,
  cannot be sketched. The carrier exists: `Curve3::Ellipse` is a curve
  kind the boolean's operand gate admits (`crates/topo/src/boolean/reduce.rs`,
  `gate_operand_edges`).
- **No surface for the result.** `SurfaceKind`
  (`crates/geom/src/surfaces.rs:291`) has plane, cylinder, cone,
  sphere, torus and NURBS and no spheroid, so a revolved ellipse would
  be a rational NURBS wall, unless a spheroid kind is added.
- **No non-uniform map.** A body moves only by a `RigidMotion`.
  `RigidMotion::check_rigid` refuses a scale of 1.6 along one axis
  `NotRigid { check: "transform_rigid_col2_unit" }`. This is lily wall 4
  (`demos/tour/src/lily.rs:2087`), whose retirement instruction is
  "model buds as spheroids".

## The ask

The everyday route is the profile one: an ellipse arc, or a spline
segment, in a profile, revolvable like any arc. Non-uniform scaling of
a whole body is a secondary route, and lower priority. Other kernels
support it mostly by converting the stretched surfaces to NURBS.

## The design question

What surface a revolved ellipse arc becomes: a new exact spheroid kind
(every face-pair table then owes it a row), or the rational NURBS
surface of revolution (which the boolean meets as `Nurbs`, behind the
operand edge gate `splice/delete-the-boolean-operand-edge-gate`).

## Done when

The lily's bud can be authored as an ovoid, and wall 4 is retired or
re-aimed at the non-uniform scale it pins.

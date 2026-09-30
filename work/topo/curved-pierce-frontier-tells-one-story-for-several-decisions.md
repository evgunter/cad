---
id: curved-pierce-frontier-tells-one-story-for-several-decisions
kind: issue
title: topo: CurvedPierceUnsupported is the definite arm of the clearance coincidence and of the wall and torus root lanes, and tells one story for all
status: open
opened: 2026-09-30
---


(TOPO, the §5 second pass of PR 3513: the definite siblings of its new
decisions.)

## What

`BooleanError::CurvedPierceUnsupported` (`crates/topo/src/boolean/mod.rs`)
is raised by `boolean::reduce::curved_face_arm`'s `frontier` closure
for every shape the crossing layer cannot take. PR 3513 ends it in the
coincidence's levers without the tolerance ("declare the coincidence,
or move the geometry"), the story of its in-band sibling, the line and
circle clearance escalations (`Coincide::EdgeOnCurvedFace`), where the
declared-cover rung does read a declaration.

It is also the decided arm of decisions no declaration settles:

- `BooleanDecision::WallRoots(WallRung::AxisParallel)`: a decided zero
  `bool_point_in_solid_denom` is `WallRoots::AxisParallel`, which every
  edge-sweep arm refuses at the frontier;
- `BooleanDecision::WallRoots(WallRung::Discriminant)`: a zero-band
  discriminant is `WallRoots::Tangent`, the frontier;
- `BooleanDecision::TorusRoots`: an uncertain count
  (`TorusRoots::Uncertain`) is the frontier;
- the circle × torus lane's `Coaxial` and `Uncertain`.

Their in-band arms name "move the parts so the edge clearly crosses the
wall (torus) or clearly misses it", and their decided arms the
frontier's declare menu: one decision, two stories (D4 ¶1 (iv)).

A second sibling of the same shape: the declared-coaxial cylinder ×
sphere arm's radius guards (`BooleanDecision::Radius`). Their decided
arm, `SectionError::DegenerateOperand`, reaches `join::cs_pair_frame`'s
catch-all and renders as `BooleanError::JoinDesync` ("A/B lockstep
invariant violated … kernel bug"), while the in-band arm names the
radius lever. No in-tree caller passes `CoaxialEvidence::Declared`
yet, so neither arm is reachable from a public door today.

## Repair shape

Carry on `CurvedPierceUnsupported` which decision refused (a closed
type the `frontier` closure's callers set, as the root lanes' wrap
sites now do for the escalations), and end each arm in its decision's
lever; route `cs_pair_frame`'s `DegenerateOperand` to the radius
decision's decided arm.

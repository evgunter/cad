---
id: transform-rigid-maps-a-poisoned-net-into-the-placeholder
kind: issue
title: transform_rigid maps a poisoned NURBS net instead of refusing it, and a rotation can turn it into the placeholder
status: open
opened: 2026-10-10
priority: P3
cost: E
refs: [described-net-two-state-reads-hand-a-poisoned-net-the-described-arm]
---


## What

`topo::transform`'s `map_surface` asks a `Surface::Nurbs` payload only
"placeholder or not" (`if n.is_placeholder()`, `transform.rs` ~449) and
maps everything else by its control points (`NurbsSurface::map_points`).
`map_curve`'s `Curve3::Nurbs` arm (~599) does the same for a carrier.
A net in `geom::NetState::Poisoned` therefore reaches the mapping arm,
which is the wrong answer for it twice over:

- **It is mapped instead of refused.** `geom`'s rule (`net.rs`'s
  `is_placeholder` doc, `NetState::Poisoned`) is that a net poisoned in
  some channel is corrupt described geometry that must fail at the
  consumer's described arm. Here the described arm is a data move that
  never reads the poison.
- **A rotation can make it the placeholder.** The linear part mixes
  channels, so a point with NaN in `x` comes out NaN in every channel it
  rotates into. A net with every `x` poisoned becomes all-poison: the
  benign "no description yet" state that the census arm-1 skip, the
  mesh refusal text and the attach doors' chartless landing all treat
  as mid-surgery scaffolding. That is the one answer the width rule says
  such a net must never get.

Evidence, from a throwaway in-crate probe on the `m7_8_cube` fixture
(`cert_m3r1_probes.rs`), with the wall's payload replaced under its own
key and a 45° rotation about `z` (`Affine3::rotation_about_axis`):

- the centre point's `x` NaN: `Poisoned` before, `Poisoned` after `map_points`;
  `transform_rigid` → `Err(Certify { source: PlaneNurbs(ChartSpeed(NotFinite { axis: U })) })`;
- every `x` NaN: `Poisoned` before, **`Placeholder` after**;
  `transform_rigid` → `Err(Certify { source: Unimplemented })`, because the
  edge re-certification now resolves the wall as the placeholder.

So on this fixture the door still refuses, but only because an edge
description names the wall, and the refusal it gives names the wrong
cause (`Unimplemented`). A face whose surface no certified description
names would come out of the door carrying a placeholder that was minted
from corrupt geometry. Validation at rest would still refuse that body
(check 1's `UncertifiableSurface`), which is why this is P3 and not a
live wrong answer.

## Fix shape

Match `n.net_state()` in `map_surface` and refuse `NetState::Poisoned`
with a typed `TransformError` that names the face's surface, before any
mapping. `TransformError::NurbsPlaceholder` stays for the placeholder.
The curve arm needs the same answer, and `NurbsCurve3` has only
`is_placeholder`. Either `geom` grows the curve twin of `net_state` (it
reads the same `net::any_poison`), or this arm refuses on a non-finite
control point directly. A regression row: the probe above, asserting the
typed refusal for both nets.

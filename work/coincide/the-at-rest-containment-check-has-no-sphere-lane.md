---
id: the-at-rest-containment-check-has-no-sphere-lane
kind: issue
title: the at-rest quieting rule's containment check has no lane for a sphere Gap pair or a non-planar, non-cylindrical overlap face
status: open
opened: 2026-10-10
priority: P1
cost: M
---


Found by stage 5 B (`interference-at-rest-is-a-finding`). The quieting
rule decides "every face bounding the overlap lies between the carriers
of an asserted pair" by building the region past each asserted carrier
as a solid and intersecting it with the overlap
(`crates/editor-core/src/checks/at_rest.rs`, `beyond` and `crossing`).
`beyond` has arms for a plane (a box past it) and a cylinder (the solid
cylinder inside a bore, the tube outside a pin) only.

Two cases refuse, so the finding is `Unlocalized::Containment`: loud,
and nothing quiets it. That is the safe direction, but a correctly
asserted fit cannot be quieted.

- **A sphere `Gap` pair** (a ball pressed into a socket). `Gap` has the
  arm, so the assertion holds and is a candidate. But `beyond` has no
  ball or shell to build, and `reach_of` (`eval/measure.rs`) does not
  bound a sphere face, whose cap bulges past its rim circles.
- **An overlap face that is neither plane nor cylinder** (a fillet's
  torus, a cone). `reach_of`'s boundary-walk bound is stated for plane
  and cylinder patches only.

The fix wants a sound reach bound for those surfaces (a sphere face
lies within its sphere's ball) and a revolve-built ball and shell.

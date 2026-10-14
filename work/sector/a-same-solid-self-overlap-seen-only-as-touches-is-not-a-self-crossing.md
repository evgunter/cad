---
id: a-same-solid-self-overlap-seen-only-as-touches-is-not-a-self-crossing
kind: issue
title: A solid whose own shells overlap while meeting only in touches is not caught by arm 2's self-crossing exception, so its pairs are read against an ill-formed material
status: open
opened: 2026-09-28
priority: P3
cost: M
---


Filed by CONTACT-5's delta review (N3, found by inspection, pre-existing).

Arm 2's `blocks` (`sweep_cross_solid_backstop` in
`crates/topo/src/census.rs`) refuses a pair when one of its solids
crosses its own boundary. It reads a self-crossing from two findings
only:

- a pierce between two entities of that solid;
- an edge cross between two entities of that solid.

That is the `Named::Two` arm for two entities of one solid. A touch
between two entities of the same solid reads `None` there. This is
the reviewer's reasoning:

- A solid can have two shells whose materials overlap while meeting
  only in touches. Two half-overlapping lumps sharing two extents are
  one such solid: coplanar same-sense faces, edges lying in faces,
  corners on edges. That is exactly the shape the half-overlap row
  refuses between two solids.
- Such a solid makes only same-solid touch findings. The body is
  refused on those findings, but not as a self-crossing.
- Its pairs with other solids are then probed and read against a
  material that is not well formed. The point-in-solid door's parity
  over two overlapping shells counts the doubled region as outside.

Nothing is cleared that the body's own findings do not already
refuse. The cost is that the pair verdict is read against a solid the
census could name as self-overlapping.

The repair is to read a same-solid touch through `TouchSite::verdict`
with the two shells' cones, and to treat a decided crossing as the
self-crossing it is. Cost M.

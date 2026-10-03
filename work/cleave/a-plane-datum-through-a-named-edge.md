---
id: a-plane-datum-through-a-named-edge
kind: issue
title: A DM1-style plane datum through a named edge (plus a normal) would let a split through that edge carry its declaration structurally
status: parked
opened: 2026-10-03
priority: P3
cost: M
blocked_on: [3990]
---


From PR 3960 (2026-10-03). Today `SplitPlane` carries bare values, and
`verbs/split.rs` reads only `DatumValue::Plane`. A datum shaped like
`Datum::FaceFrame` (REFERENCES.md DM1), e.g.
`PlaneThrough { at, edge: StableName, normal }`, would reference the
pinching edge by name. The Split node could then emit the declaration for
that edge itself, with no value inspected.

This is Ev's "we literally declared that the tip lies on the plane",
written as a reference. Waits for the split declaration seat.

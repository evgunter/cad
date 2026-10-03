---
id: a-wall-flush-with-a-fillets-tangent-line-refuses-the-pierce
kind: issue
title: A planar wall standing on a fillet band's tangent line refuses CurvedPierceUnsupported
status: open
opened: 2026-10-02
priority: P3
---

## What

A prism whose side wall lies in the plane of a rounded plate's
trimline — the line where a fillet band meets the plate's flat top,
tangent to the band — refuses to union with the plate:

```
Boolean(CurvedPierceUnsupported { operand: B, face: FaceKey(30v3),
  edge: EdgeKey(97v1), band: Band { zero: 1e-9, escalate: 1e-8 } })
```

The prism's vertical edges in that plane touch the band's cylinder
along the trimline, tangentially. `BooleanError::CurvedPierceUnsupported`'s
doc (`crates/topo/src/boolean/mod.rs`) lists "a tangency (not a
crossing at any order the lane sees)" among what the curved pierce
door does not take. Standing a feature flush with the edge of a
fillet is ordinary authoring, and the union it asks for is
well-defined: the wall meets the top face along the trimline and never
enters the band.

## Reached by

The heat sink of `demos/tour/src/heatsink.rs` at r = 1/16 instead of its
shipped 1/32: a `3 × 1 × 0.25` plate with all twelve edges filleted
(`Node::Fillet`), nine `0.1875 × 0.75` fins at a 0.3125 pitch from
x = 0.25. The ninth fin's outer wall is x = 2.9375, the end band's
trimline. Measured on the SHOW branch merged with main (2026-10-02),
in two seats:

- fins sunk 1/16 into the plate (transversal): the payload above;
- fins flush on the plate, declared: the same refusal with
  `face: FaceKey(19v1)`.

At five and seven fins (no wall on a trimline) both build at the
closed-form volume. The scene ships r = 1/32 and cites this row where
it justifies that radius.

## Owner note

Filed on HONE by `work.py territory` (`crates/topo/src/boolean/*`);
the curved pierce door's reach is also REACH's charter.

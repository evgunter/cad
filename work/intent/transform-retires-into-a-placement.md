---
id: transform-retires-into-a-placement
kind: issue
title: D10 stage 3 PR D: Node::Transform retires; a rigid motion of a body is a copy placed by one frame mate
status: parked
opened: 2026-10-08
priority: P0
cost: M
blocked_on: [a-placement-is-the-bundle-of-mates]
refs: [intent-stage3-is-built]
---

INTENT stage 3, PR D. Spec: `docs/INTENT-STAGE3-SPEC.md` §5.

`Node::Transform` (`node.rs:2499`) and `PortKind::PlacedFrom` retire. A rigid motion of a body is a `Place` with one `Frame` mate to an `Offset` of the frame it is placed against. `Step::Literal` retires, and A6's admission runs on evaluated pose values. The tour's `chain`, `diefillet` and `teapot`, the viewer's transform gesture and Python's `.transform` follow. Geometry is bit-equal.

---
id: transform-retires-into-a-placement
kind: issue
title: D10 stage 3 PR D: constructions read no frame and nothing moves a body; Datum, Transform and PlacedFrom retire, each use becoming a placement
status: parked
opened: 2026-10-08
priority: P0
cost: H
blocked_on: [a-mate-relates-two-poses]
refs: [intent-stage3-is-built]
---

INTENT stage 3, PR D. Spec: `docs/INTENT-STAGE3-SPEC.md` §5. Built on FORK-S3P (row 95, PR 4324) and FORK-S3M (row 97, PR 4326).

Frames enter only at placement:

- A profile is 2-D and reads no plane.
- `Tube` and `HollowTube` are built about their own axis.
- `Extrude` loses `side`: the side is the placement's mates' to say.
- `Node::Datum` retires.

Every construction is built in coordinates of its own and placed against what its datum was related to. That is one mate plus values, and its bits move by one composition, which the migration reports.

Nothing moves a body. `Node::Transform` and `PortKind::PlacedFrom` retire, and each use becomes a `Place`. A copy's names pass through as `Transform`'s did. `Step::Literal` retires, and A6's admission runs on evaluated pose values.

An absolute pose read by a non-root operation (a `Split` tool) is restated over the operand's geometry, or the migration names it and refuses to regenerate (spec Q8). The façade's "sketch on a face" writes profile, construction, placement and combine as one gesture. The tour's `chain`, `diefillet` and `teapot`, the viewer's transform gesture and Python's datum builders and `.transform` follow.

Closes the placement half of `a-boxed-rotation-refuses-not-rigid-at-every-placer`. Waits on C, because a construction placed against a face needs a mate reading its plane and values for the freedoms it leaves.

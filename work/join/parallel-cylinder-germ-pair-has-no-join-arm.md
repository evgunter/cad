---
id: parallel-cylinder-germ-pair-has-no-join-arm
kind: issue
title: Two parallel cylinder walls meeting in rulings reach the join's germ-pair dispatch with no arm (CurvedBooleanUnsupported)
status: open
opened: 2026-10-02
priority: P1
cost: H
refs: [non-circle-conic-edge-refuses-against-every-curved-face, slab-cut-cylinder-refuses-sector-side]
---


Found by REACH's ellipse-rim lane (2026-10-02), measured on its
merge of main after PR 3627.

## Measured

The lower part of the tilted drum cut (`crates/sweep/tests/conic_edge_curved_face.rs`:
radius 0.5, height 1, cut through `(0, 0, 0.5)` at 0.3 rad) against a
rod of radius 0.2 standing across its rim at `(0.5, 0)`,
`z ∈ [0.2, 0.45]`: the crossing layer and the sector side pass, and ∪,
∩ and ∖ refuse `CurvedBooleanUnsupported { operand: A, face: FaceKey(5v1),
kind: Cylinder }`, raised at `boolean::join`'s germ-pair dispatch (the
`(a_s, b_s)` no-arm case; instrumented: the pair is Cylinder ×
Cylinder). The two walls are parallel and meet in two rulings, a
section the frame dispatch already accepts as straight
(`GermFrameUnsupported`'s doc: "a cylinder pair's is rulings exactly
when its axes are parallel"), but the join step itself has no
cylinder × cylinder arm. Narrower rods across the same rim (radius 0.1
at `(±0.45, 0)` and `(0, 0.48)`) stop earlier at the pierce-ring door
(`work/tang/pierce-ring-has-no-join-arm.md`). Pinned by
`a_rim_crossing_reaches_the_join`.

## What a fix has to supply

The join-split arm for a parallel cylinder pair: the chord is a
ruling of both walls, so each side's split rides its own wall's
ruling — the plane × cylinder ruling arm's shape on both sides.

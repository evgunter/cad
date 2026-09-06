---
id: EXCH-H1
kind: unit
title: degree-1 line promotion and the ExtrudedPoint rung in nurbs_iso_derive
status: spec
opened: 2026-09-03
branch: exch/h1-degree-one-line
parent: step-import-degree-one-line-promotion
refs: [388, 389]
---

The two halves of `step-import-degree-one-line-promotion`, landed
together because each is unwitnessable without the other: a certified
line-recognition limb in `recognize_curve` (zero-radius cylinder
composite, `√sup` metre residual, convexity-derived segment
obligation) and the `ExtrudedPoint`/`PlacedSegment` arm in
`topo::pcurves::nurbs_iso_derive` (TRIM's file, edited at the
announced one-arm seam per both programs' keep_outs — EXCH dispatched
first). Retires dm1 edge `#389`'s zero-candidate gap; does NOT flip
dm1 first-class (that is #390's lane). Spec `docs/EXCH-H1-SPEC.md`.
Pre-draw difficulty M, task class NUMERIC.

---
id: TESS-2
kind: unit
title: the rational patch bound encloses the DESCRIBED patch: knot refinement inside the ring
status: review
opened: 2026-09-20
refs: [nurbs-face-bound-unsound-on-a-random-rational]
priority: P0
cost: H
pr: 3080
branch: tess/2-refinement-in-the-ring
---


Spec: `docs/TESS-2-SPEC.md`. Full v6 dual. Block TESS-B1 slot 1
(record on `tess/b1-block`). An ANNOUNCED TERRITORY CROSSING:
`crates/geom-brep/src/patch_bound.rs` (and possibly
`crates/geom-core/src/spline/algebra.rs`) are PROPS'; PROPS is paused
and Ev said not to wait (in chat, 2026-09-20). Carries
`rational-cells-hull-the-f64-refined-net-so-the-described-patch-escapes`
and closes `nurbs-face-bound-unsound-on-a-random-rational`.

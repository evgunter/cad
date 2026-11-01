---
id: TESS-2
kind: unit
title: the rational patch bound encloses the DESCRIBED patch: knot refinement inside the ring
status: closed
opened: 2026-09-20
refs: [nurbs-face-bound-unsound-on-a-random-rational]
priority: P0
cost: H
pr: 3080
branch: tess/2-refinement-in-the-ring
closed: 2026-09-22
---


Spec: `docs/TESS-2-SPEC.md`. Full v6 dual. Block TESS-B1 slot 1
(record on `tess/b1-block`). An ANNOUNCED TERRITORY CROSSING:
`crates/geom-brep/src/patch_bound.rs` (and possibly
`crates/geom-core/src/spline/algebra.rs`) are PROPS'; PROPS is paused
and Ev said not to wait (in chat, 2026-09-20). Carries
`rational-cells-hull-the-f64-refined-net-so-the-described-patch-escapes`
and closes `nurbs-face-bound-unsound-on-a-random-rational`.

## Closed (2026-09-22, PR 3080 merged at `9bd61de7e`)

Knot refinement inside the ring: `CurvePlan::apply_ring` replays the
f64 schedule's Step list with outward-rounded ring ratios (the convex
form, chosen on a width measurement); `TensorNet::refine_u/v`,
`patch_bound::refine_chain`; `rational_cells` and the integral branch
of `patch_cells_refined` assemble from ring-refined nets, so every
`PatchCell` encloses the DESCRIBED patch — both arms were unsound
before (704,835 exact containment checks on the head, 0 escapes;
12,615 on the reverted tree). Exact-referee rows (7 fixtures × 3
components, bare) red before and green after; the structural zero
contained on all 768 quarter-cylinder cell-channels; the dust re-pin
bracketed with a floor; bilinear census 30,000 → 0 reds. Width:
median ~1e-14, top 1 % to ~5e-12 (a fold of 16 single insertions;
one-pass refinement filed on ENCL). Time: 1.01–1.13× per call after
the refined-grid precompute; 0.90–1.30× on the golden corpus, triangle
counts unchanged. No golden, digest, demo pin or budget baseline
moved. A second defect found and fixed in the fix pass: `apply_ring`
answered a longer line from the prefix it could source — both
directions of extent mismatch now refuse. The sampler's own error is
measured and re-runnable (`sampler_error_dump` + `sampler_error.py`;
maxima 1.4–2.7 ulps across three draws); `SAMPLER_ULPS = 64` stays
PROPS' to re-size. Filed: five more f64-inside-an-enclosure sites
(PROPS), a third `split_points` spelling, one-pass refinement and the
`refine_chain` pair (ENCL), the net skeleton copy and `Option<Ratio>`
(PROPS), the referee nobody runs (TINT). Spec deleted (ledger).

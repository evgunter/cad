---
id: an-integral-patch-with-knots-hugging-its-edges-trips-the-area-gauge
kind: issue
title: quad: an integral patch with knots 1e-9 from both trim edges certifies an area 2e7 wide and trips the A2 area-gauge debug_assert
status: open
opened: 2026-10-22
priority: P2
cost: M
---


## Found (PR 5582's dual review, 2026-10-22; reproduced by the QUAD lane)

Pre-existing: the reviewers reproduced it on the merge base, and the
`integral` lane it lands in is untouched by PR 5582.

`nurbs_patch_face::<f64>` on `cert5_r1_patch_probes`'
`knots_hugging_the_trim_edges_stay_sound` net with UNIT weights:

- knots `[0, 0, 0, 1e-9, 1 − 1e-9, 1, 1, 1]`, degree 2 by linear;
- control `(0,0) (0.5,0.4) (1,0.9) (1.5,0.4) (2,0)` × `z ∈ {0, 1}`;
- whole rectangle, perimeter 8, ε = 1e-7.

It panics in `props::quad::debug_assert_area_gauge` (perimeter arm) at
a CERTIFIED return. The area bracket is 2.0e7 wide, a mean edge
displacement 3.1e5× the perimeter. A reviewer's variant read 4e14.

On the two tiny spans the speed runs to ~1e9, so the integral lane's
area pad (`area_at`/`area_cell`, `norm_hi` of hull boxes over 2-D
cells) is that wide. The rational twin of the same net (weights 2)
reads `[1.13, 3.72]` through the quotient statement's hulls per block.
Wide is not wrong. The A2 tripwire still says a certified return this
wide means the area pass lost the geometry, so the lane should refuse
typed, or tighten, before the tripwire is reached.

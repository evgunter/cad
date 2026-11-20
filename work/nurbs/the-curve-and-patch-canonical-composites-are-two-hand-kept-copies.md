---
id: the-curve-and-patch-canonical-composites-are-two-hand-kept-copies
kind: issue
title: The curve and patch canonical composites are two hand-kept copies that have drifted; the patch twin bounds by coefficient ratios and spells the torus quartic well conditioned
status: open
opened: 2026-11-20
priority: P3
cost: M
---

Found by EXCH (whole-patch promotion, `exch/promote-torus-sphere-cone`,
PR 7172's second review).

`geom_core::spline::compose::canonical_composite` (curves) and
`compose::patch::canonical_patch_composite` (tensor-product patches) state
the same five canonical forms over two channel algebras
(`BernsteinSpans` and `PatchSpans`), by hand, twice. They have drifted
in two ways, both in the patch twin's favour:

- **The cell bound.** `PatchComposite::cell_bound` encloses `N/D` by the
  hull of the coefficient ratios `N_k/D_k` (the forms are homogeneous, so
  numerator and denominator share a degree and `N/D` is a convex
  combination of the ratios); `CompositeForm::span_bounds` still takes
  `hull(N)/hull(D)`, which is looser wherever the weights vary.
- **The torus quartic.** The patch twin forms it as
  `A² + 4R²·W²·(Z² − r²W²)` with `A = |G|² − (R² + r²)·W²`, each term of
  size `(2Rr)²` near the torus; the curve twin keeps
  `(|G|² + (R² − r²)W²)² − 4R²·(X² + Y²)·W²`, which subtracts two terms of
  size `(2R²)²`. On a metre-scale torus the curve twin's enclosure is
  about `(R/r)²` wider (measured on the patch side: 1.7e-12 against
  1.7e-13 m at `R = 24`, `r = 6`).

Owed: one source for the five forms over both channel types (a small
trait both span types implement, or the curve composite as the
one-parameter case of the patch one), with the ratio bound and the
conditioned torus association on both. Moving the curve side moves the
curve arms' certified numbers, so its pins need re-taking with it.

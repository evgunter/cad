---
id: the-rational-cell-substrate-is-built-twice-in-patch-bound-and-enclose
kind: issue
title: the rational cell substrate is built twice: ssi::enclose's CellGrid::refined hand-rolls the TensorChannels<4> refinement and positivity check that patch_bound's refine_split does, and CellNet::pair_terms/pair_scales re-spell TensorCoeffs::diff_u
status: open
opened: 2026-10-12
---



## Found (encl/one-chart-speed, 2026-10-12)

That lane put one quotient-numerator form (`patch_bound::QuotientForm`)
and one speed reading (`patch_bound::pair_norm_sup`) under both
`patch_bound::rational_cells` and `ssi::enclose`'s `CellNet`. What
feeds the form is still built twice.

- **Refinement.** `crates/geom-brep/src/ssi/enclose.rs`
  `CellGrid::refined` forms the four homogeneous channels `(w·P, w)`
  by hand, refines each as a bare `TensorNet` through
  `refine_u`/`refine_v`, and checks `certainly_positive` on the
  refined weights. `crates/geom-brep/src/patch_bound.rs`
  `rational_cells` builds the same four channels as one
  `TensorChannels<4>`, refines them through `refine_split` (the
  equal-split schedule), and checks the refined weights' positivity
  in its own loop (`RefinedWeightLostPositivity`). The two differ in
  their schedules, arbitrary split points against equal splits, and
  in how they refuse (`None` against a typed error). They do not
  differ in what they compute.
- **Differencing.** `CellNet::pair_terms` with `pair_scales` forms
  each adjacent pair's derivative coefficient
  `scale·(w₁·(P₁ − P₀) + (w₁ − w₀)·(P₀ − S))` from Cartesian points.
  That is `TensorCoeffs::diff_u`/`diff_v` on the homogeneous net,
  recentred, which `patch_bound`'s `DNets::build` already calls. A
  `CellNet` cut to a window (`CellNet::cut`, a Bézier block) has no
  knot vector of its own, and that is what kept it on the hand-rolled
  form.

## What is open

One refined-cell substrate that both consumers read: the refined
homogeneous channels with their positivity proven once, and the
form's `k_i`/`g_i` taken from `diff_u`/`diff_v` of them. `CellNet`'s
cut block is the case that has to fit into it.

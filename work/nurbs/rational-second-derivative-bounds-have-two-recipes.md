---
id: rational-second-derivative-bounds-have-two-recipes
kind: issue
title: Rational second-derivative bounds (chords' carrier M, patch_bound's rational cells) use the hull-quotient recipe, not the pair-term one order up
status: open
opened: 2026-10-12
priority: P3
cost: M
---


## What

The rational **speed** bound now has one statement: the pair-term
bound in `crates/geom-core/src/spline/derivative.rs`
(`rational_speed_sup`, `rational_stretch_sup`, the shared
`RationalPairTerm`), read by `NurbsCurve::speed_upper_bound` and
`NurbsSurface::stretch_sup`. Two rational bound sites still use the
older homogeneous hull-quotient recipe (recipe A in
`rational-speed-bounds-have-two-recipes-across-four-sites`: numerator
hull plus `(S − c)` hull times the weight-derivative hull, over
`w_min`) and were left out of that unit:

- `crates/mesh/src/chords.rs:309` `rational_carrier_m_bound`, a
  `sup‖C″‖` bound, which is one derivative order up.
- `crates/geom-brep/src/patch_bound.rs:943` `rational_cells`. Its
  `s_u`/`s_v` interval boxes are both the rational `s_u_sup`/`s_v_sup`
  and the inputs to the `S_uu`/`S_uv`/`S_vv` recurrences behind
  `m_sup`. Routing only the scalar sups through the pair-term
  statement would leave two `S_u` recipes in one cell (the box for the
  recurrence and the pair term for the sup). That is why it was not
  routed alongside the speed bound.

## Shape of the fix

Both sites are recipe A per cell, one order up. The pair-term
analogue at second order would write `S″` (and `S_uv`) as a
convex combination of terms that are affine in `S` and `S′`. Bound
each term at the hull points with `S′` from the pair-term statement.
Route both sites through it, so the first- and second-order rational
bounds each have one recipe. The goldens that pin `m_sup` and the
chord counts move with it (re-baseline them, stating the reason).

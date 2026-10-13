---
id: rational-second-derivative-bounds-have-two-recipes
kind: issue
title: Rational second-derivative bounds (chords' carrier M, patch_bound's rational cells) use the hull-quotient recipe, not the pair-term one order up
status: closed
opened: 2026-10-12
priority: P3
cost: M
branch: nurbs/second-order-pair-term
closed: 2026-10-13
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
- `crates/geom-brep/src/patch_bound.rs` `rational_cells`. Its
  `s_u`/`s_v` interval boxes are both the rational `s_u_sup`/`s_v_sup`
  and the inputs to the `S_uu`/`S_uv`/`S_vv` recurrences behind
  `m_sup`. Routing only the scalar sups through the pair-term
  statement would leave two `S_u` recipes in one cell (the box for the
  recurrence and the pair term for the sup). That is why it was not
  routed alongside the speed bound.

Two readers take `rational_cells`' first-order readings as they stand, so
they read recipe A per cell rather than the pair-term statement:
- `crates/geom-brep/src/offset_meters.rs` `patch_regularity`, which
  folds `PatchCell::s_u_sup` / `s_v_sup` into its `SupSpeed` pair;
- `crates/mesh/src/nurbs_cert.rs` `cell_readings`, which reads the
  cells' `s_u` / `s_v` boxes beside the second-order ones.

They move with this row: once a cell's `S_u` box comes from the
pair-term statement, both read it unchanged.

## Shape of the fix

Both sites are recipe A per cell, one order up. The pair-term
analogue at second order would write `S″` (and `S_uv`) as a
convex combination of terms that are affine in `S` and `S′`. Bound
each term at the hull points with `S′` from the pair-term statement.
Route both sites through it, so the first- and second-order rational
bounds each have one recipe. The goldens that pin `m_sup` and the
chord counts move with it (re-baseline them, stating the reason).

## Closed (2026-10-13)

The second order is derived beside the first in
`crates/geom-core/src/spline/derivative.rs` ("The second order"):
`W·C″ = Σ N_{j,p−2}·k²ⱼ·(b_{j+1} − bⱼ)(C) − 2·W′·C′`, where the `bᵢ`
are the first-order pair terms. So `sup‖C″‖` is at most the vertex
max of the scaled second differences, plus `2·max|dᵢ|·S₁`, over the
least weight; `S₁` is the first-order bound and `dᵢ` the weight
rates. Per cell, the surface reads
`W·S_uu = … − 2·W_u·S_u` and `W·S_uv = … − W_v·S_u − W_u·S_v`.

Both orders read one set of pair terms, `RationalSpanTerms` and
`RationalCellTerms`; no third per-pair builder was written.
- `mesh` chords' `rational_carrier_m_bound` now reads
  `NurbsCurve3::accel_upper_bound`.
- `patch_bound`'s rational cells take every box and sup off the cell's
  vertex sets.
- The readers, `offset_meters` `patch_regularity` and `nurbs_cert`
  `cell_readings`, take the new cells unchanged.

Witness rows and the mutant table are in the PR body.


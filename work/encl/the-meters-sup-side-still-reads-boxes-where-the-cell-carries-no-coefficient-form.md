---
id: the-meters-sup-side-still-reads-boxes-where-the-cell-carries-no-coefficient-form
kind: issue
title: the offset meters' sup side still folds boxes where PatchCell carries no coefficient form: the rational arm's chart speeds and cell_normal's sup on ‖S_u × S_v‖
status: open
opened: 2026-10-08
priority: P3
cost: M
---


## Found (ENCL, `encl/offset-cert-coefficient-norms`, 2026-10-08)

That unit moved the offset certificate's vector upper bounds onto
coefficient norms (D4 ¶2): `Composite::y_sup`, `Composite::m_tilde_sup`
(`crates/geom-brep/src/offset_fit.rs`), and the integral arm's chart
speeds, `PatchCell::s_u_sup`/`s_v_sup` (`crates/geom-brep/src/patch_bound.rs`,
`integral_cells_on` through `window_norm_sup`). Two sup-side readings in
the meters stay box folds, because the cell they read carries no
coefficient form of the vector:

- **The rational arm's speeds.** `rational_cells` assembles `S_u` by the
  quotient rule over cell hulls, `(Ã_u − (S − c)·w_u)/w`, and fills
  `s_u_sup` with `norm_sup(&s_u)`. A coefficient form exists —
  `S_u = (A_u·w − A·w_u)/w²`, a vector B-spline product over a positive
  scalar, which is `geom_core::spline::compose::tensor::coefficient_norm_bound`'s
  shape — but the arm would have to form those products
  (`PatchSpans` on the refined nets) rather than hulls.
- **`cell_normal`'s `sup`** (`crates/geom-brep/src/offset_meters.rs`):
  `norm_sup(&m).min(gram_sup)`, with `m = cross(s_u, s_v)` a box cross
  product and `gram_sup` the upper end of `EG − F²`, whose `F²` enters
  through its box-assembled LOWER end. Neither is a coefficient reading.
  On the integral arm `m` is a polynomial and its coefficients could be
  formed as `offset_fit`'s `Composite` forms `M̃`; `cell_curvature`
  (the collapse meter) reads this `sup` as `‖m‖`'s upper end.

Both feed predicates (the regularity lever and the collapse headroom),
not a stored certificate number, so a rotation can move a meter's
margin by the box fold's factor (measured on the integral arm's speeds
before the fix: `1.9e-3` of their scale on the bowed fixture under 16
rotations, in `the_vector_upper_bounds_do_not_move_under_a_rotation`).

## What is open

Whether to form the coefficient products for the rational arm's speeds
and for `m`, and what that costs per cell; the floors stay
box-assembled either way (`a-rigid-map-can-still-refuse-a-sound-approx-face-at-its-edges-or-meters`).

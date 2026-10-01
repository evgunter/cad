---
id: the-offset-certificate-reads-vector-norms-off-per-coordinate-cell-hulls
kind: issue
title: the offset certificate's Y limb, its M-tilde divisor and the regularity meter's speeds read a vector norm off per-coordinate cell hulls, so a rigid map moves them
status: open
opened: 2026-10-01
priority: P3
cost: M
---


## Found (§5 sweep of SSI's frame-invariant limb-2 lane, branch `ssi/frame-invariant-bound`, 2026-10-01)

**Inferred from the code, not reproduced.** SSI's plane × NURBS limb 2
moved under a rigid map because `tensor::SurfaceResidual::sup_bound`
folded per-coordinate coefficient hulls into a Euclidean norm. A box of
one vector field reads between 1× and √3× its norm depending on how the
field sits against the axes. That lane now reads each vector
coefficient's norm (`geom_core::spline::compose::tensor`,
`coefficient_norm_bound`: `max_k |n_k| / |d_k|` over a one-signed
scalar denominator, the convex-hull property applied to the norm), and the
bound moves only by rounding. The same shape remains in the offset
certificate and its meters, as `norm_sup` over three per-coordinate
cell hulls:

- `Composite::cell_bound` (`crates/geom-brep/src/offset_fit.rs`): the
  `τ` term's `‖Y‖` upper bound, `norm_sup` of `self.y[d].cell_hull`.
  `Y = Ẽ × M̃` is the limb the `Approx` face's `hull_sup` carries.
- `Composite::m_tilde_sup` (same file): the upper bound on `‖M̃‖`, the
  divisor of `e_floors`' lower bound on `‖E‖`.
- `decompose` in `offset_fit.rs`'s test module repeats the `‖Y‖`
  reading for its probe, and follows whatever `cell_bound` does.
- `cell_normal` (`crates/geom-brep/src/offset_meters.rs`): `sup` is
  `norm_sup(&m).min(gram_sup)`, where `m` is the cross-product box and
  `gram_sup` reads `E = ‖S_u‖²` and `G = ‖S_v‖²` off `norm_sq` of the
  derivative boxes.
- `patch_regularity` (same file): `speed_u`/`speed_v` are `norm_sup` of
  each cell's `s_u`/`s_v` box.

`work/encl/a-rotation-can-refuse-an-approx-face-that-certifies-near-eps.md`
and `map_approx`'s re-fit answer the rotation drift of `hull_sup` after
the fact. Reading the `Y` limb from coefficient norms would remove the
fold component of that drift at its source, which might leave the
re-fit with less to do (one designer's estimate, not measured).

## What does not convert

Lower bounds stay box-assembled: `e_floors`' componentwise mignitude,
the regularity floor and `sqrt_down(gram.lo())`. A convex function's
minimum over a cell is not at a coefficient, so a floor cannot be read
from coefficient norms. The meters' frame sensitivity through those
floors is
`work/encl/a-rigid-map-can-still-refuse-a-sound-approx-face-at-its-edges-or-meters.md`'s.

## What is open

For each upper bound above, decide whether to read it from the vector
coefficients' norms (the `Y` and `M̃` channels are Bernstein forms over
the cell, as the tensor composite is), and measure the change in the
`Approx` rigid-map drift.

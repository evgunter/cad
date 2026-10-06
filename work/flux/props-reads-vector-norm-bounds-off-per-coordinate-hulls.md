---
id: props-reads-vector-norm-bounds-off-per-coordinate-hulls
kind: issue
title: props: the area pads and the curve second-derivative bound read a vector norm off per-coordinate hulls, so they depend on the frame
status: open
opened: 2026-10-01
priority: P4
cost: M
---


## Found (§5 sweep of SSI's frame-invariant limb-2 lane, branch `ssi/frame-invariant-bound`, 2026-10-01)

**Inferred from the code, not reproduced.** SSI's plane × NURBS limb 2
moved under a rigid map because it folded per-coordinate coefficient
hulls into a Euclidean norm. A box of one vector field reads between 1×
and √3× its norm depending on how the field sits against the axes, so a
bound assembled that way is sound but moves under a rotation, and a
verdict re-derived near its threshold can flip. That lane now reads each
vector coefficient's norm (`geom_core::spline::compose::tensor`,
`coefficient_norm_bound`: `max_k |n_k| / |d_k|` over a one-signed
scalar denominator, the convex-hull property applied to the norm). The same
shape is here:

- `nonrational_second_derivative_sup`
  (`crates/geom/src/curves/second_derivative.rs`): the hull of each
  coordinate's second-difference net over the WHOLE domain, squared and
  summed. This is exactly the shape the tensor composite had, and the
  three maxima can come from different control points.
- `rational_patch_face` (`crates/geom-brep/src/props/quad.rs`): the
  area cell's `g_hull` and the `pad_d` pads through `norm_hi`, each
  `sqrt_enclosure` of the three channel squares of a bracketed
  cross-product vector.
- `trimmed_patch_face_rounds` (same file): `lune_g`, the same assembly
  over the lune's cross-product hull.

Lower bounds are not this class: a convex function's minimum over a
cell is not at a coefficient, so a mignitude floor cannot be read from
coefficient norms.

## More sites (§5 sweep of SSI's rotation chart-speed lane, branch `ssi/rotation-chart-speed`, 2026-10-02)

**Inferred from the code, not reproduced.** The same fold, through
`norm_hi` or a hand-written `√(Σ cᵢ²)` over a cell's bracketed
vectors, at sites the list above does not name, all in
`crates/geom-brep/src/props/quad.rs`:

- `nurbs_patch_face_rounds`: the area cell's `g_u`, `g_v` (`norm_hi`
  of the `∂(S_u×S_v)` boxes) and `g_hull`.
- `area_cell`: the same three readings over a `Collapse::Over` cell.
- `trimmed_patch_face_rounds`: `sup_g` over the trim box, and the
  chart's metric rates `rate_u`, `rate_v` (`norm_hi` of the `S_u`/`S_v`
  hulls), which turn chart variation into metres.

The point readings beside them (`g_mid`, `area_at` at a single
`Collapse::At`) are enclosures of one vector, not hulls of a field, and
`norm_lo` and `boundary_chord_perimeter_lo` are lower bounds: neither
is this class.

## What is open

For each site, decide whether frame sensitivity costs anything here (a
re-derived verdict near its threshold, or tightness), and if so read the
bound from the vector coefficients' norms.

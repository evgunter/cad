---
id: chord-reads-derivative-norm-bounds-off-per-coordinate-enclosures
kind: issue
title: chord: the NURBS chord and patch certificates read derivative norms off per-coordinate enclosures, so they depend on the frame
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

- `cell_readings` (`crates/mesh/src/nurbs_cert.rs`): `norm_sq` of each
  of the cell's signed componentwise enclosures of `S_uu`, `S_uv`,
  `S_vv`, `S_u` and `S_v`, whose `√hi` (`cell_component`) bounds each
  partial's norm.
- `rational_carrier_m_bound` (`crates/mesh/src/chords.rs`): per span, a
  magnitude bound on each coordinate of the second derivative (`s2`,
  from each coordinate's own window hulls), squared and summed, then
  read `√hi` rounded up.

Lower bounds are not this class: a convex function's minimum over a
cell is not at a coefficient, so a mignitude floor cannot be read from
coefficient norms.

## What is open

For each site, decide whether frame sensitivity costs anything here (a
re-derived verdict near its threshold, or tightness), and if so read the
bound from the vector coefficients' norms.

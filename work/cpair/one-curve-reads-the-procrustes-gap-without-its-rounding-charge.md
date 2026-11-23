---
id: one-curve-reads-the-procrustes-gap-without-its-rounding-charge
kind: issue
title: the pose's one-curve decision reads the Procrustes gap without charging its rounding
status: closed
opened: 2026-10-19
closed: 2026-10-19
pr: 5172
branch: germ/ellipse-pair-locus-bound
priority: P3
cost: E
---


`geom_brep::curve_pair::pose::one_curve`
(crates/geom-brep/src/curve_pair/pose.rs) decides
`curve_pair_pose_one_curve` on `Margin::of(one_curve_gap(e, f))` with
no rounding charge. Every coplanar conic pair is read through it,
topo's Boolean included (`carrier_cross` reads `curve_pair::meet`).

The computed gap's rounding, `u` the unit roundoff and `m` a conic's
larger semi-axis magnitude:

- the semi-axis vectors round by `u·|a|` and, through `v̂ = n̂ × û`,
  by `4u·|b|`, so each `L` by `√17·u·m`;
- the computed `Q`'s columns are unit to within about `4.5u` (both
  PR 5172 reviewers measured 3–4.4u), which moves `L_f·Q` by under
  `4u·m_f` from the exactly orthogonal `Q` along them;
- the products with `Q` round by `4u·m_f`;
- the subtraction and the norms by `5u·g`.

So the computed gap is within `12.1u·(m_e + m_f) + 5u·g` of one the
bound holds for, inside `rounding_charge(m_e + m_f + g)`. A Zero read
on the bare gap can sit up to that far under a gap that bounds the
Hausdorff distance: about `2.7e-13` at `m = 100`, a quarter of
ε = 1e-12, and past ε at `m ≈ 400`. At Interval the enclosure carries
the rounding already. The fix is that charge on the `f64` read, with
`one_curve_rows.rs`'s 0.6z / 1.5z rows re-read at 1e-12.

A first cut charged `|a_e| + |b_e| + |a_f| + |b_f|`. At r = 100,
ε = 1e-12 that came to about ε alone, and two derivations of one
ordinary ellipse escalated. Charge the larger semi-axes, as above.

Found by germ's `coplanar-ellipses-one-curve-bound-compares-frames-not-loci`.

The same read was topo's: `carrier_cross`'s own one-curve margin read
the locus gap bare until main routed it through `curve_pair::meet`, so
this one site is the only reader left, and every caller now shares the
charged gap.

## Fixed

PR 5172: `pose::one_curve_gap` returns the gap with
`rounding_charge(m_e + m_f + g)` added, so no caller can read it bare.
Pinned at ε = 1e-12, each red with the charge taken out and green with
it: eight ellipse pairs whose computed semi-axis vectors agree bit for
bit while the exact ones differ, Hausdorff distance up to `1.0105ε`
(`coplanar_ellipses_one_locus.rs`); a near-circle at r = 10³, gap read
bare `9.69e-13`, Hausdorff `1.071e-12` (`one_curve_rows.rs`, on the
one-curve reading, since the pose escalates that pair earlier on the
planes' offset); and an ellipse of semi-axes `100 + 1e-12`, `100` with
`u_ref` one ulp off unit against a circle of radius 100, Hausdorff
`1.0036e-12`. A degenerate side (zero semi-axis) reads a NaN gap, which
escalates.

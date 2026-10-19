---
id: one-curve-reads-the-procrustes-gap-without-its-rounding-charge
kind: issue
title: the pose's one-curve decision reads the Procrustes gap without charging its rounding
status: open
opened: 2026-10-19
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
- the computed `Q`'s columns are unit to within `8u`, which moves
  `L_f·Q` by `4u·m_f` from the exactly orthogonal `Q` along them;
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

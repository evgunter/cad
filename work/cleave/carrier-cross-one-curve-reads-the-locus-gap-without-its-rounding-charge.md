---
id: carrier-cross-one-curve-reads-the-locus-gap-without-its-rounding-charge
kind: issue
title: carrier_cross's one-curve decision reads the locus gap without charging its rounding
status: open
opened: 2026-10-14
priority: P3
cost: E
---


`topo::boolean::carrier_cross::one_curve` (crates/topo/src/boolean/carrier_cross.rs)
decides `bool_carrier_cross_one_curve` on `Margin::of(e.locus_gap(f))`
with no rounding charge. `geom_brep::Conic::locus_gap`'s docs
(crates/geom-brep/src/implicit.rs) bound the computed gap's rounding at
`12.1u·(m_e + m_f) + 5u·g`, `m` the larger semi-axis magnitude, so a
Zero read on the bare gap can sit up to that far under a gap the bound
holds for: about `2.7e-13` at `m = 100`, a quarter of ε = 1e-12, and
past ε at `m ≈ 400`.

The pose's two-ellipse arm (`curve_pair_pose_one_curve`,
crates/geom-brep/src/curve_pair/pose.rs `conic_conic`) charges
`rounding_charge(m_e + m_f + g)`; the fix is the same charge here, with
the k-ε body rows (`a_ruling_lying_on_a_wall`, the
`two_ellipses_are_one_curve_only_within_the_band_of_each_other` unit
row) re-read at 1e-12.

Found by germ's `coplanar-ellipses-one-curve-bound-compares-frames-not-loci`.

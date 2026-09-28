---
id: chart-uniform-breaks-lean-on-exact-dedup
kind: issue
title: The chart certificate's uniform breaks are skipped against the operands' knots by exact dedup alone
status: open
opened: 2026-09-28
---


Filed by the sweep on `encl/domain-uniform-grid`, which gave the
domain-uniform refinement grid one home
(`geom_core::spline::algebra::domain_grid_points`, with an explicit
`GridSkip` rule).

## What

`geom_brep::ssi::certify::nurbs_limbs` builds its `extra` break list
as `t0c + (t1c − t0c)·i/SSI_CERT_SPANS` over the carrier's domain with
no skip at all, and hands it to
`geom_core::spline::compose::tensor::surface_curve_residual`, whose
merged break list (`extra` ∪ the pcurve's and the carrier's interior
knots, then `sort_by(total_cmp)` + `dedup`) is the only thing that
drops a grid point sitting on a knot. `dedup` is exact `f64` equality,
so a stated carrier or pcurve knot one ulp off a 32nds grid point
leaves two breaks an ulp apart: the mechanism
`work/nurbs/refine-dir-hairline-knot-insertion.md` measures for
`refine_dir`, on this lane's break list. Unmeasured here.

It is not the same function as `domain_grid_points` (the skip set is
the union of TWO vectors' knots, and the skip happens in the consumer),
which is why the homing unit left it alone.

## What a fix looks like

Whatever guard the hairline row settles on, applied at the merge in
`surface_curve_residual` (NURBS ground) or to `extra` against both
operands' knots before the call (SSI ground). First step: a probe with
a carrier knot at `1/16 + 1 ulp` measuring `hull_sup` against the same
carrier with the knot at `1/16`.

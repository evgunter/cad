---
id: chart-uniform-breaks-lean-on-exact-dedup
kind: issue
title: The chart certificate's uniform breaks are skipped against the operands' knots by exact dedup alone
status: closed
opened: 2026-09-28
priority: P4
cost: E
closed: 2026-10-01
pr: 3668
branch: ssi/probes
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
knots, then `sort_by(total_cmp)` + `dedup`) and
`compose::to_bezier_spans_extra`'s exact `==` filter against the
vector's own interior runs (`compose.rs:350-353`) are the only things
that drop a grid point sitting on a knot. `dedup` is exact `f64` equality,
so a stated carrier or pcurve knot one ulp off a 32nds grid point
leaves two breaks an ulp apart: the mechanism
`work/nurbs/refine-dir-hairline-knot-insertion.md` measures for
`refine_dir`, on this lane's break list. Unmeasured here.

It is not the same function as `domain_grid_points` (the skip set is
the union of TWO vectors' knots, and the skip happens in the consumer),
which is why the homing unit left it alone. The GRID half is already
the helper's: `certify.rs:505-508` computes exactly
`domain_grid_points(carrier.knots(), SSI_CERT_SPANS, …)`'s points
(same domain, same `t0c + (t1c − t0c)·(i/N)` expression) and could
call it; only the skip set, spanning two vectors' knots, does not fit.

## What a fix looks like

Whatever guard the hairline row settles on, applied at the merge in
`surface_curve_residual` (NURBS ground) or to `extra` against both
operands' knots before the call (SSI ground). First step: a probe with
a carrier knot at `1/16 + 1 ulp` measuring `hull_sup` against the same
carrier with the knot at `1/16`.

## Closed (2026-10-01, PR 3668)

**Measured.** The fixture is the `z = 1/2` plane against the rational
quarter-cylinder wall, with the carrier the exact rational arc (radius
`1 + δ`) with one knot inserted. Limb 2's bound is quoted in units of
ε (1e-9), at `δ = 0` (the rounding floor, where a hairline shows most)
and at `δ = ε/2`:

| carrier knot | lane `hull_sup`, δ=0 | δ=ε/2 | composite on an exact pcurve, today → guarded, δ=0 | δ=ε/2 |
|---|---|---|---|---|
| none | 0.003031 | 0.733641 | 0.003182 | 0.733727 |
| `1/16` | 0.002683 | 0.733423 | 0.002831 | 0.733508 |
| `1/16 + 1 ulp` | 0.002732 | 0.733440 | 0.002881 → 0.002834 | 0.733525 → 0.733503 |
| `1/16 − 1 ulp` | 0.002893 | 0.733559 | 0.003044 → 0.002849 | 0.733645 → 0.733503 |
| `1/16 + 4 ulp` | 0.002697 | 0.733468 | 0.002848 → 0.002841 | 0.733554 → 0.733519 |
| `3/8 + 1 ulp` | 0.001294 | 0.732869 | 0.001382 → 0.001369 | 0.732869 → 0.732869 |
| `0.313` (off grid) | 0.001499 | 0.732869 | 0.001599 → 0.001599 | 0.732869 → 0.732869 |

The hairline costs at most **2.0e-4 ε** (`1/16 − 1 ulp`). That is
below what inserting one knot at all costs (3.5e-4 ε), and about a
fifteenth of the composite's rounding floor (≈ 0.003 ε, which is the
`insert_once_ring` widening). Because both readings carry the same
insertion widening, the knot-at-`1/16` row against the hairline rows
isolates the hairline from it.

**In the plane × NURBS lane the `extra` grid never mattered.** The
lane's pcurve is `chart_image`'s degree-1 interpolant on the 33-point
schedule, so its interior knots are exactly the carrier domain's 32nds,
and `extra` is a subset of the pcurve's own knots. A skip on `extra`
leaves the merged break list bit-identical there: the lane columns read
the same before and after the change, to the printed digits. The
hairline that remains in that lane is the carrier's knot against the
**pcurve's** knot at the grid point (`1/16 + 1 ulp`: +4.9e-5 ε). That
pair is merged inside
`geom_core::spline::compose::tensor::surface_curve_residual`, which is
NURBS ground and the `refine-dir-hairline-knot-insertion` family. No
SSI-side guard on `extra` can reach it.

**Where it could matter is a pcurve off the grid.** The "exact pcurve"
columns stand in for that case: the marcher's OQ4-aligned fit, whose
pcurve shares the carrier's knots. There a guard on `extra` removes the
hairline, as the arrows show.

**Fixed anyway, since it is cheap.** `nurbs_limbs` builds `extra`
through `chart_breaks`, which is
`range_grid_points(domain, SSI_CERT_SPANS, GridSkip::WithinUlps(SLIVER_CLEARANCE_ULPS), carrier ∪ pcurve interior knots)`.
That is `domain_grid_points`' own body, with the skip set widened to
both curves. The hairline row has not flipped its own sites yet: they
are still `BitEqual`, and the flip it names is `WithinUlps(SLIVER_CLEARANCE_ULPS)`,
the rule `bezier_blocks` already uses. This site takes that rule.
`ssi::certify::tests::chart_breaks_skip_a_grid_point_beside_either_curves_knot`
pins it. `ssi::certify::refined`, the tube's box chain, keeps
`BitEqual` with the hairline row's other two sites, and the tube limbs
read the same in every row above.

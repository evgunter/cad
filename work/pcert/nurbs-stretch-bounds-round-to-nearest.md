---
id: nurbs-stretch-bounds-round-to-nearest
kind: issue
title: pcurve_cache's nurbs_stretch_bounds mints SupSpeed from a round-to-nearest norm and ratio, so the tagged sup can sit below the true one
status: closed
opened: 2026-10-01
priority: P3
cost: E
closed: 2026-10-14
---


(SSI orchestrator, from the chart-speed design weighing, 2026-10-01.)

`crates/geom-brep/src/pcurve_cache.rs`'s `nurbs_stretch_bounds` takes a
control-net difference's `v.norm()` and multiplies it by `ratio`, both
rounded to nearest, and mints the result as a `SupSpeed`. The torus
arm's `major_radius + minor_radius` in `chart_stretch_sup` has the same
shape. A value tagged as a certified upper bound can therefore land an
ulp or so below the true sup. This is the same class as
`work/ssi/ssi-certify-stretch-divides-by-a-norm-with-no-sqrt-up.md`,
which SSI fixes by giving `norm_sup` (outward: ring squares and sums,
then `sqrt_up`) one home in `geom_core`. Once that lands, this site can
call it.

The priority is low because D4 ¶2 calls the f64 lane "a conservative
estimate", so this is weaker than the SSI instance, where the value
divides a certified lower bound. Inferred from reading the code; not
measured.

## Closed 2026-10-14: an estimate by contract, and ulp-scale

The site has moved: `nurbs_stretch_bounds` now calls geom's
`NurbsSurface::stretch_sup`, whose doc states it: "At `Interval`
certified, at `f64` the same expression in round-to-nearest". Per D4 ¶2
the f64 lane is a conservative estimate and the Interval lane is the
enclosure. The outward `norm_sup` this item waited for exists
(`geom_core::interval::norm_sup`).

**Measured.** Each f64 `stretch_sup` was logged beside the same net
lifted to `Interval`. The census covered geom, geom-brep, topo, half of
sweep (5,462 of 10,959 ci tests: a worker restart cut it short) and all
of step-import:

| run | readings | f64 below the Interval enclosure's `lo` | (hi − f64)/hi: median / p99 / max |
|---|---|---|---|
| geom … sweep (half) | 1,444 (10 poisoned, degree > 28) | 0 | 3.9e-16 / 5.2e-9 / 1.0 |
| step-import | 364 | 0 | 1.6e-16 / 9.0e-15 / 9.0e-15 |

The f64 value always lies inside the certified bracket. Its gap to the
upper end is about one ulp at the median. The larger gaps are the
Interval enclosure's own width on wide nets, not an f64 under-read.
Every f64 consumer uses it as a lever or as a Lipschitz term inside a
bound that is already computed in round-to-nearest, so rounding only
this read outward would certify nothing at f64. The torus arm's `R + r`
is the same: at most half an ulp below at f64, and outward at Interval.

The one f64 consumer whose wording claims a certificate is step-import's
whole-patch promotion; that question is filed for EXCH as
`work/exch/surface-promotion-claims-a-certificate-from-a-round-to-nearest-bound.md`.

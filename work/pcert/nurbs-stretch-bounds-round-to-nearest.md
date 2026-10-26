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
Interval enclosure's own width on wide nets, not an f64 under-read:
- p99 is 5.2e-9, from `surfaces_stretch_sup::the_bound_covers_random_rational_nets`,
  on nets whose sup runs 1e4 to 5e6;
- the max, 1.0, is one reading, geom's
  `surfaces_stretch_sup::six_decades_of_weight_are_covered`, `v`
  channel. There the f64 value is exactly 0 and the enclosure is
  `[0, 3.0e-15]`: a net with no extent along `v`, whose true stretch is
  0. f64 did not under-read, and the upper end is finite.

The torus arm's `R + r` behaves the same: at most half an ulp below at
f64, and outward at Interval.

**Two f64 consumers word their result as certified.**
- **step-import's whole-patch promotion.** Filed for EXCH as
  `work/exch/surface-promotion-claims-a-certificate-from-a-round-to-nearest-bound.md`.
- **topo's `chart_region::certified_arms`, spline arm.** It reads
  `chart_stretch_inf`'s `sup_u` and `sup_v`, which are this sup, into
  `trace_sup = (sup_u/inf_u)² + (sup_v/inf_v)²`. Its skew discount `ρ`
  rises as the trace falls, so a sup read low raises the arm that the
  function's name calls certified.
  - At Interval, `nurbs_stretch_bounds` is outward, and the arm is a
    bound.
  - At f64, D4 ¶2 covers it: the per-axis infs, `area_inf` and the
    trace and determinant arithmetic around it are round-to-nearest
    too, so rounding only this read outward would certify nothing
    there.
  - Whether `certified_arms` should claim a certificate at f64, or stop
    saying so, is the owner's question: filed for CHART as
    `work/charttail/certified-arms-reads-a-round-to-nearest-stretch-at-f64.md`.

**Re-taking the census.** The probe instruments geom's
`NurbsSurface::stretch_sup` itself: every suite that reads a stretch
logs through it, so it cannot be an `#[ignore]`d row. It is on the
branch `pcert/census-probes` (f44ce42a). Run there:
`PCERT_STRETCH_LOG=/tmp/stretch.log cargo nextest run --release --profile ci -p geom -p geom-brep -p topo -p sweep -p step-import`.
Each line reads `STRETCH f_u lo_u hi_u f_v lo_v hi_v <test>`. The
re-take reproduces the max row exactly (`six_decades_of_weight_are_covered`,
`v`: `0`, `[0, 3.04e-15]`).

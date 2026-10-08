---
id: nurbs-stretch-bounds-round-to-nearest
kind: issue
title: pcurve_cache's nurbs_stretch_bounds mints SupSpeed from a round-to-nearest norm and ratio, so the tagged sup can sit below the true one
status: open
opened: 2026-10-01
priority: P3
cost: E
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

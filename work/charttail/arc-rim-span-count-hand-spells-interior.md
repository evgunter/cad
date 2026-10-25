---
id: arc-rim-span-count-hand-spells-interior
kind: issue
title: topo pcurves' arc-rim span count hand-spells the interior-knot filter KnotVector::interior already names
status: open
opened: 2026-09-28
priority: P4
cost: E
---

`topo::pcurves::nurbs_iso_derive` (`crates/topo/src/pcurves.rs:812`,
the arc-cap-rim arm) counts the chart's `u` interior knots as
`ku.knots().iter().filter(|k| **k > d0 && **k < d1).count()` against
`ku.domain()`. That is `ku.interior().len()`
(`geom_core::spline::KnotVector::interior`, `knots.rs:806`): the raw
interior slice, multiplicities kept, which is what the `/ 2 + 1`
arithmetic after it reads (a rational-quadratic arc chart carries each
interior break at multiplicity two). The substitution is bit-for-bit;
it names the count rather than re-deriving the clamp runs.

Found by the ENCL `domain-grid-homing-residue` sweep for hand-spelled
interior-knot filters, which settled the `props/quad.rs` instances.
It is CHART's: TRIM's 2026-09-20 cut gave `pcurves.rs` to CHART
(`work/trim/program.md` and `work/curved/program.md` `keep_out`).
Two places still say TRIM owns the file and were left for their
owners: `work/exch/program.md`'s `keep_out` and
`work/chart/angular-arms-are-an-untagged-lever-beside-a-typed-rate.md:51`.

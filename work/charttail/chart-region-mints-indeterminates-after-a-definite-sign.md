---
id: chart-region-mints-indeterminates-after-a-definite-sign
kind: issue
title: chart_region mints Indeterminates by hand after a definite sign, at three sites
status: review
opened: 2026-10-01
priority: P1
cost: E
branch: charttail/decided-gates
pr: 7168
---


## What

`crates/topo/src/chart_region.rs` asks the funnel and then builds an
`Indeterminate { margin: MarginDiag::INVALID, .. }` of its own after a
DEFINITE sign, three times:

- `carrier_agreement`: `decide("chart_region_carrier_tilt", …)`,
  `Ok(Sign::Negative)` (a max of absolute values decided negative) →
  `ChartRegionError::Escalated(Indeterminate { .. })`;
- `cylinder_pair_overlap`'s `norm_gate` closure: the same shape,
  `Ok(Sign::Negative)` → a hand mint;
- `cylinder_pair_overlap`'s axis-dot classification: `Ok(Sign::Zero)`
  (the dot of near-parallel units, which the tilt gate already
  bounded away from zero) → a hand mint.

Each escalation reaches the caller and is on no frame's escalation
log: the funnel recorded the verdict, and the op returned an
escalation the bracket never saw.

## Shape

`geom_core::k_stats` has the gates: `decide_nonzero` for the axis-dot
site (a side read off a sign, with no side at zero), and the two
"nonnegative quantity decided negative" sites want a gate that admits
`Zero | Positive` — `decide_positive` with the margin's sense chosen at
the site, or a third gate if the site argues for one. Found by the
`linalg/decided-not-minted` sweep of `MarginDiag::INVALID` literals
across `crates/*/src`; the `linalg` branch fixed the same shape in
`topo`'s `sector_shape`.

## Re-measured and fixed (CHARTTAIL, 2026-11-20, branch `charttail/decided-gates`)

Against main `131457c3`, two of the three sites above were already
gone: `carrier_agreement` and `cylinder_pair_overlap`'s `norm_gate` go
through `k_stats::decide_magnitude` (CLEAVE step 1, PR 3979). The
third, the axis-dot classification (`chart_region_cyl_axis_sense`),
now goes through `k_stats::decide_nonzero`, so its Zero is the funnel's
rejection, on the frame's log, carrying the margin decided zero.

That Zero is no longer reported as poison. The tilt gate's Zero does
not make it a contradiction: the dot is levered by `hyp`, which is at
least the larger radius, so a cylinder whose size lies inside the zero
band decides both rows Zero honestly. The margin is what the classifier
saw, and the refusal says so.

`definite_diag`'s three callers, the riding row's sites, go through
`decide_positive` in the same change.

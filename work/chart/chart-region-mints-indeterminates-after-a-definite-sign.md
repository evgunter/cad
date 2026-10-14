---
id: chart-region-mints-indeterminates-after-a-definite-sign
kind: issue
title: chart_region mints Indeterminates by hand after a definite sign, at three sites
status: open
opened: 2026-10-01
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

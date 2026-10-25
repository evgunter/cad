---
id: chart-definite-diag-labels-an-interval-lower-end-as-an-f64-value
kind: issue
title: topo: chart_region's definite_diag reports an interval margin's lower end as a point Value, the wrong lane's reading
status: open
priority: P3
cost: E
opened: 2026-09-29
---


(Filed by the ENCL implementer lane on PR 3418's review. The defect predates that PR; the PR only respelled the mint.)

`topo::chart_region::definite_diag` (`chart_region.rs`, near `:661`) builds the escalation for a definite margin whose outcome cannot be certified. It builds it as `geom_core::MarginDiag::value(margin.value().lo())`, a `Bounds::lo` read of a generic `T`.

At `f64` that is the margin. At `Interval` it labels the enclosure's lower end with the point lane's kind (`MarginKind::Value`), so the refusal renders one endpoint as if the classifier had seen a number. `MarginDiag`'s contract is "what the classifier saw", and a reading that claims the wrong lane breaks it.

- **Fix shape:** take the reading from the decision itself. `k_stats::decide_reported` returns the `Decided` whose `margin` is the lane's own view, so the escalation can echo that and drop the bracket read.
- **Knock-on:** the site then leaves `scripts/gates/reporting-margin-door.sh`'s mint list.

## Its words (TOPO, the fix pass of PR 3506)

PR 3506 gave `geom_core::IndeterminatePayload` an arm for a margin at or
inside the zero band ("margin m lies within the zero band (±z)"), so
`definite_diag`'s `Ok(Sign::Zero)` echo (`chart_region_area`,
`chart_region_cyl_band_area`, `chart_region_cross_order`) now renders
that, where before it claimed the margin "lies inside the ambiguity band
(z, e)", false for a margin the band decided zero. Its
`Ok(Sign::Negative)` echo still renders "lies inside the ambiguity
band": a margin below `−e`, which that band does not hold. Carrying the
`Decided` (the fix shape above) lets the escalation say which verdict
the conservative deduction could not certify.

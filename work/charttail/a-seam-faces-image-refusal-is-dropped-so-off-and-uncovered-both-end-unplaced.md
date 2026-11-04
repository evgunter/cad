---
id: a-seam-faces-image-refusal-is-dropped-so-off-and-uncovered-both-end-unplaced
kind: issue
title: A seam face's image refusal is dropped, so off and uncovered both end Unplaced
status: open
opened: 2026-11-04
priority: P2
cost: C
---


Filed by `pcert/spline-chart-incidence` (the §5 sweep). `topo/src/pcurves.rs`
`seam_face` reads `pcurve_of(body, half_edge, band).ok()?`, and its callers
(`boolean/ops.rs`'s `DescribeRefusal::failed`, `splitting/finish.rs`'s
`SplitFinishError::Undescribed`) map the `None` to `EdgeDescribeFailure::Unplaced`.
Since `pcert/spline-chart-incidence`, `pcurve_of` on a spline wall ends a carrier
shown off the wall `CarrierOffChart` and one read in band `Escalated
(ChartIncidence)`; the `.ok()?` drops both, so off, in band and uncovered all end
`Unplaced`. Carry the mint's refusal to the caller's ending instead.

---
id: pcurve-fit-refusal-drops-the-domain-doors-reason
kind: issue
title: PlaneNurbsRefusal::PcurveFit discards the domain door's typed SplineError
status: open
opened: 2026-09-12
---

## Finding

Filed by the SCALAR lane for `D290` (the knot rescale as a `KnotVector`
door), which the spec fenced away from this question: "whether
`PcurveFit` should carry the `SplineError` is TRIM's question and not
this unit's — leave the refusal vocabulary as it is."

`crates/geom-brep/src/edge_nurbs.rs`, `chart_image`: the last line maps
`on_carrier_domain(&image, t0, t1)` to `PlaneNurbsRefusal::PcurveFit`
with `map_err(|_| ..)`, discarding the `SplineError` the domain door
returns. After D290 that error is typed twice over — a
`KnotVectorIssue::DomainInvalid { lo, hi }` when the carrier's `(t0, t1)`
is collapsed, reversed or non-finite, or the clamp clause a rounding
collapse tripped — and `PcurveFit`'s `Display` text still says only
"the chart image could not be interpolated through the schedule's foot
points (a degenerate parameterization)", which is the interpolation's
failure mode, not the domain's. A caller reading the refusal cannot tell
a degenerate carrier interval from a singular collocation system.

Two answers are on the table, both TRIM's: give `PcurveFit` a payload
(the `SplineError`, or a two-arm reason: interpolation vs domain), or
split a `CarrierDomain { lo, hi }` refusal off it. `PlaneNurbsRefusal`
is `Copy + PartialEq`, and `SplineError` is neither `Copy` nor `Eq`, so
the payload choice is a design choice for the enum's derives too. The
same producer feeds `crate::PcurveFittedLane::general_image`, so the
mint's vocabulary moves with it.

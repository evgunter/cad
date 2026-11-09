---
id: the-cone-face-at-rest-pin-names-carrier-off-chart-by-variant
kind: issue
title: review_m3_pr4's cone-face-at-rest pin names CarrierOffChart by variant instead of reading IncidenceRead::of
status: closed
opened: 2026-11-05
priority: P3
cost: E
closed: 2026-11-09
pr: 6622
---


Filed by the ONECOPY `onecopy/m7-8-plain-names` lane (PR 6535) on its review's pointer.

## What

`crates/topo/tests/review_m3_pr4.rs`, `assert_cone_face_refuses_at_rest`
(near line 719): the cross-lane topo verdict on a cone face over a
brick's lines picks out the derivation's refusal by naming the variant,
`PcurveCertifyError::CarrierOffChart { chart: Cone, .. }`. That is the
shape that sent `cert_m3r1_probes::m3_the_plain_names_report_the_corrupt_m7_8_wall_edge_by_edge_and_nothing_else`
red after PR 6498. That probe named `ResidualExceeded`, and when the
derivation's endings were reordered the refusal moved to
`CarrierOffChart` without the fact it reports moving. "Is this refusal
a decided incidence miss" has one home, `geom_brep::IncidenceRead::of`
(the classifier every incidence ending matches on). A variant pin keeps
a second copy of its answer.

The pin is green today. It goes red the day a cone's off read ends as
`ResidualExceeded { check: ChartIncidence }`, which the classifier
already reads as `Off(None)`, even though nothing the test means has
moved.

## What closing it takes

Match the refusal through `IncidenceRead::of(error)` (an `Off(_)`
read), keeping the chart-kind check on the face's own surface, as the
re-pinned M7-8 probe does (PR 6535).

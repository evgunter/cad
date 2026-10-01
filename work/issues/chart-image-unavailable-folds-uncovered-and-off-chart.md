---
id: chart-image-unavailable-folds-uncovered-and-off-chart
kind: issue
title: EdgeCurve certification folds chart_pcurve's uncovered and off-chart refusals into one ChartImageUnavailable worded 'wrong locus'
status: open
opened: 2026-10-01
---


Found by the D36 sweep (`pcert/d36-unsupported-carrier-split`). The
ground is PCERT's (`python3 scripts/work.py territory` places
`crates/geom-brep/src/certify.rs` there); filed here because that lane
does not write `work/pcert/`.

`geom_brep::certify`'s conventional-description arm
(`crates/geom-brep/src/certify.rs`, the `None => chart_pcurve(..)`
match in the chart-image mint, ~line 2106) maps every non-escalated
`chart_pcurve` refusal to `CertifyError::ChartImageUnavailable
{ chart, carrier }`, whose `Display` says "the locus this description
claims is not one this chart can state" — a wrong-locus claim — and
whose `decision()` routes it `CertCheck::ChartImage` /
`RefusedArm::SignCertain`.

Since D36 `chart_pcurve` says which of two things is true:
`PcurveCertifyError::UnsupportedCarrier { class, .. }` — the carrier
can lie on the chart and no lane covers the pair yet (a sphere's
general circle, a cone's tilted section, a torus's Villarceau circle, a
spline carrier on an analytic chart: valid input, D9 row 2) — and
`CarrierOffChart { why, .. }` — the carrier cannot lie on the chart
(the wrong-locus case the text describes). The fold reports the first
as the second: an edge described as a general circle on its own sphere
is told its locus is wrong. The payloads (`class`, `why`) are dropped
too.

Wanted: carry the distinction through (an uncovered arm worded as the
frontier, the off-chart arm keeping today's wording and its `why`), or
the whole `PcurveCertifyError` nested; re-check the routing each arm
gets in `decision()`.

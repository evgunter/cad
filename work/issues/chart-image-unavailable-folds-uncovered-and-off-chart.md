---
id: chart-image-unavailable-folds-uncovered-and-off-chart
kind: issue
title: EdgeCurve certification folds every chart_pcurve refusal into one ChartImageUnavailable worded 'wrong locus' and drops its why
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

Evidence from `pcert/general-circle-fitted-route` (2026-10-01): the
pcurve MINT now routes a sphere's general circle through the fitted
lane (`topo::pcurves::analytic_derive`, `FittedLane::sphere_circle_image`
then `certify_fitted`), so the class is no longer uncovered at rest. This
door is the one place it still reads as uncovered: an edge DESCRIBED
conventionally (`EdgeDescriptionSpec::Chart`, no stated image) as a
general circle on its sphere still refuses here, because the image
derivation is the closed-form door and this certification runs at
`Decide`, without the fitted door. No producer is known to reach it —
the fillet corner's contact circles are `TangentIntersection`s — so the
wording fix above is what is owed; a route for that description would
need this door to take the fitted lane.

Moved by `pcert/projected-image` (2026-10-08): the general circle on
its sphere is no longer uncovered at this door either. `chart_pcurve`
images it as `Pcurve::Projected` and certifies it at every `Decide`
scalar, because a circle's incidence is the closed-form
`off_sphere_sup` and its envelope needs no fitted door. Of the
uncovered classes the fold could still meet, `MirrorTorusSpiric` was
routed on `pcert/mint-routes-spiric-nofitted` (PR 4897) and
`NoFittedClass` on `pcert/fitted-row-reads-carrier-and-chart` (PR
4900, both 2026-10-12). PR 4900 then deleted `UncoveredClass` and
`PcurveCertifyError::UnsupportedCarrier`.

**What remains (2026-10-12).** The uncovered-versus-off-chart half of
this item is gone, because `chart_pcurve` no longer raises an
uncovered refusal. Its non-escalated refusals are now
`CarrierOffChart` (wrong locus, which the text says correctly, though
its `why` is still dropped), `CarrierGrazesChart` (a circle within the
band of a cone or torus that is none of its circles, a defect the text
also fits), `UnsupportedChart` (a spline chart, which this arm should
not reach for a conventional description) and the projected lane's own
refusals. Still owed: carry `why`, or nest the whole
`PcurveCertifyError`, and re-check the routing each arm gets in
`decision()`.

**Since PCERT E1 (`pcert/lane-choices`, 2026-10-25).** `CarrierGrazesChart` is gone, and the projected lane's refusals no longer reach the fold. The mint arm in `crates/geom-brep/src/certify.rs` (the `chart_pcurve` call's `map_err`) now reads `chart_pcurve`'s refusals as follows:

- **D3's lane choices are their own arms.**
  - `PcurveCheck::ChartClass` maps to `CertCheck::ChartClass`.
  - `PcurveCheck::GeneralLane` maps to `CertCheck::GeneralLane`. This is the projected lane's sector or nappe reading, relabelled at the mint by `general_lane_refusal` for a circle's lane choice and for a spline alike.
  - `PcurveCertifyError::NoGeneralLane` maps to `CertifyError::NoGeneralLane`. It is a circle on a cone or torus that is none of its circles, or a pose the projected lane cannot partition.
  - All three end lever-only "not yet".
- **The fold (`ChartImageUnavailable`) still takes:**
  - `CarrierOffChart`, the wrong locus, which the text says correctly, though its `why` and margin are still dropped;
  - `UnsupportedChart`, a spline chart;
  - the check-4-shaped refusals no mint raises.

Still owed: carry `why` (and `OffMargin`) or nest the whole `PcurveCertifyError`, and re-check each arm's routing in `decision()`.

---
id: a-plane-or-pair-lane-on-a-spline-wall-ends-an-off-carrier-as-its-limbs-bound
kind: issue
title: A plane or pair lane on a spline wall ends an off carrier as its limbs' bound or an unsettled foot
status: open
opened: 2026-11-04
priority: P1
cost: M
---


Filed by `pcert/spline-chart-incidence` (the §5 sweep). The edge certificate's
spline lanes (`edge_nurbs.rs`: `plane_nurbs_limbs`, `nurbs_pair_limbs`,
`plane_nurbs_incidence`, `exact_arc_rim`/`after_exact`, reached from
`certify.rs`'s `Resolved::PlaneNurbs`, `OnPairNurbs` and `PairNurbs`) read the
carrier against its wall by upper bounds only: the arc-rim pick and its row
certificate, the fitted chart image's Newton feet, limbs 1 and 2. None reads a
lower bound first, so:

- a carrier off its wall whose foot will not converge ends
  `FootPointInconclusive` → `FittedEnding::FootUnsettled` ("not yet"), not off;
- `ExactRim::Refused(None)` reads "no image along a boundary row holds it", whose
  doc admits it covers a rim off the chart;
- the hull limbs' refusals are honest bounds but end an off carrier as one that is
  on and bent.

## Fix

Read `geom_brep::read_edge_incidence` (which reads a spline chart since
`pcert/spline-chart-incidence`, by the control hulls' lower bound) where a lane's
limb or foot refuses, before its ending is chosen, as `nurbs_iso_derive` does:
off ends `CarrierOffChart`, in band ends in the lever, and the lane's own ending
stands otherwise.

Banded P1 at review (PR 6498, fix Y8b): this is the closed
`a-spline-chart-has-no-incidence-test-so-a-lane-choice-refusal-cannot-tell-off-from-uncovered`'s
own class, an upper bound ending an off carrier as an uncovered one, on the
build and import doors (the edge certificate) rather than the mint.

## Evidence from `pcert/spline-certificate-door`

That unit made the pcurve certificate doors read a spline chart's incidence
first (`pcurve_cache.rs` `read_shown`), so an exact image these lanes certify
through `PcurveCache::certify` now refuses `CarrierOffChart` (or escalates
`PcurveCheck::ChartIncidence`) for a carrier off its wall. Two lanes carry that
refusal into their own vocabulary rather than ending it as the read:

- `edge_nurbs.rs` `exact_arc_rim` folds it into `ExactRim::Refused(Some(ChartIncidence))`,
  and the fitted path's limb refusal then ends `PlaneNurbsRefusal::ArcRim`;
  `ArcRimRefusal::decision` reads `PcurveCheck::incidence_limb`, which that unit
  maps `ChartIncidence` to `SsiLimb::HullValueChart` (the map residual's limb, which
  the read bounds from below), so the refusal reads as the exact image's check
  rather than the fitted bound, but still as a limb, not `CarrierOffChart`
  (`geom-brep/tests/onpair_plane_nurbs.rs`
  `an_arc_rim_off_its_wall_refuses_with_both_causes`);
- `certify.rs` `Resolved::SharedRow` folds it through `row_image_refusal` into
  `RowRefusal::Image(Some(ChartIncidence))` / `ImageEscalated`, ending the same way.

The read is already taken at the pcurve door in both, so the fix there is to end
a refusal whose `IncidenceRead::of` is `Some` as that read, not to read again.

Two reroutes follow from that unit's `PcurveCheck::incidence_limb`
mapping `ChartIncidence` to `SsiLimb::HullValueChart` (review fix GG5):

- `certify.rs` `RowRefusal::decision` (a shared row refused on one of its
  walls) and `arc_rim_check` (an exact image escalated) now read an
  incidence refusal as `CertCheck::PlaneNurbsHullValue`, where before it
  fell to `PlaneNurbsHull`;
- topo `validate.rs` `classify_certify`'s `ArcRim` arm words it as that
  limb.

`ArcRimRefusal` now carries the exact image's refusal margin (`read`),
which `decision` keeps (`SignCertain(read)`) rather than dropping it.


---
id: a-plane-or-pair-lane-on-a-spline-wall-ends-an-off-carrier-as-its-limbs-bound
kind: issue
title: A plane or pair lane on a spline wall ends an off carrier as its limbs' bound or an unsettled foot
status: open
opened: 2026-11-04
priority: P2
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

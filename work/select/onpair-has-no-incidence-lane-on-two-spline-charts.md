---
id: onpair-has-no-incidence-lane-on-two-spline-charts
kind: issue
title: OnPair cannot meter incidence between two spline charts: the plane × spline lane's incidence has no NURBS × NURBS twin
status: open
opened: 2026-10-15
priority: P3
cost: M
---


Filed by TANG's `tang/onpair-spline-incidence` unit, which gave `OnPair` an incidence lane on a plane × spline pair and left this pair out of scope.

## What happens

`OnPair` between two spline charts (two described NURBS walls, an `Approx` fit on either side) refuses `CertifyError::Unimplemented`. `plane_nurbs_pair` does not take the pair, and `resolve` refuses each spline side, which has no implicit form (`crates/geom-brep/src/certify.rs`, the `OnPair` arm of `run_checks`). Pinned by `crates/geom-brep/tests/onpair_plane_nurbs.rs::two_spline_charts_still_refuse_the_pair`.

So a locus that two spline walls under-determine, such as a smooth wall–wall seam stated as a pair rather than as one wall's iso, has no description that names the pair.

## The model

The plane × spline lane is `geom_brep::plane_nurbs_incidence`. It derives the carrier's chart image (`edge_nurbs::chart_image`) and meters limbs 1 and 2 through `ssi::certify::incidence_limbs`, with no transversality and no tube. It reaches certify through `NurbsLane.incidence`, and the `OnPair` arm routes the pair as `Resolved::OnPairNurbs`.

The twin here has the same shape as `nurbs_pair_limbs` without its normal-angle hook or limb 3:
- the chart image on each wall;
- `incidence_limbs` over both NURBS operands;
- a `NurbsLane` pointer and an `OnPair` arm tried after `nurbs_pair`.


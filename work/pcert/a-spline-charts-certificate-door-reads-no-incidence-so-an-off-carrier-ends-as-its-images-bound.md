---
id: a-spline-charts-certificate-door-reads-no-incidence-so-an-off-carrier-ends-as-its-images-bound
kind: issue
title: A spline chart's certificate door reads no incidence, so an off carrier ends as its image's bound
status: open
opened: 2026-11-04
priority: P1
cost: M
---


Filed by `pcert/spline-chart-incidence` (the §5 sweep of
`a-spline-chart-has-no-incidence-test-so-a-lane-choice-refusal-cannot-tell-off-from-uncovered`).
That unit gave a spline chart an incidence test (`pcurve_cache::spline_incidence`,
the control hulls' lower bound) and put it in front of `topo::pcurves::nurbs_iso_derive`'s
lane choices through `read_edge_incidence`. The other three incidence doors still read
nothing on a spline chart:

- `pcurve_cache.rs` `read_shown` returns `Ok(None)` for `Surface::Nurbs | Approx`,
  so `PcurveCache::certify` and `recertify` meter a stored or stated image on a
  spline face by its own certificate alone, an upper bound. A carrier off its
  wall ends `ResidualExceeded`, `FittedLimb`, `FittedCertificate { NotYet |
  FootUnsettled }` or `IsoUnsupported`, never `CarrierOffChart`.
- `read_incidence` (the stated route, `certify.rs` `canonical_of`'s
  `Resolved::Chart { image: Some(_) }` arm) and `read_point_incidence` pass the
  same way, so a stated image on a spline face ends `ChartResidual`.
- `topo::pcurves::nurbs_iso_derive`'s own-chart arm returns the stated image
  unread; `validate_pcurves`' re-certify loop reads the stored row's
  certificate, so `validate::off_face_ending` never fires on a spline face. At a
  dual scalar the own-chart image's `FittedLaneUnsupported` is excused by
  `not_owed`, so an off carrier there passes.

## Fix

Read the spline arm in `read_shown` (the span read `read_edge_incidence` takes),
which mends the certificate door, the stated route and the at-rest re-certify at
once. The cost is one incidence read per spline-face certification (about a
dive of 65 samples on an on-chart carrier), which is why it was not taken in the
lane-choice unit, whose reads sit on refusal paths and the general fall-through
only. Time the spline-minting slow rows before and after.

## The dual scalar (review fix Y8a)

A public door does validate at `Dual`: `topo::pcurves::validate_pcurves<T:
AtRestPolicy>` (`topo::pcurves` is a `pub mod`, and `pncad::topo` re-exports
`topo::*` less the construction tier). There, an own-chart `General` image
refuses `FittedLaneUnsupported`, which `not_owed` excuses, so an off carrier
passes. That is no escape of its own, and the row stays P1.
`docs/DUAL-DESIGN.md` DL3 makes a dual evaluation validate nothing: its
gates are structurally absent ("a dual's door mints and never checks"), and
the base-scalar evaluation beside it, whose value channel is bit-identical,
is the validation of record. `topo::validate_geometric` does not compile at
`Dual` (its `CertifiedBounds` bound), and the sensitivity report refuses a
dual pass with no validated base evaluation. The off carrier is therefore
read where the base scalar reads it, which is this row's subject.

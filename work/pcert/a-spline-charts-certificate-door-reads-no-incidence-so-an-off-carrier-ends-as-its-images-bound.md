---
id: a-spline-charts-certificate-door-reads-no-incidence-so-an-off-carrier-ends-as-its-images-bound
kind: issue
title: A spline chart's certificate door reads no incidence, so an off carrier ends as its image's bound
status: closed
closed: 2026-11-04
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

DL3's "absent by type" holds for `validate_geometric`, not for
`validate_pcurves`: that door compiles at `Dual` and answers clean there on
an off carrier. The row rests on DL3's other half, that a dual's answer is
not the validation of record.

## Closed (2026-11-04): a spline chart's certificate doors read incidence

`pcert/spline-certificate-door`. `pcurve_cache.rs` `read_shown` reads the spline
arm over the edge's span (`read_edge_incidence`'s read). `incidence_ended` is the
one home of when a certificate door reads it: an analytic chart before the door,
a spline chart where the door refuses, or where the certificate's span is not
shown to cover the span asked about. Every door goes through it:

- `PcurveCache::certify`, `recertify`, `certify_fitted` and `certify_general`;
- the edge certificate's stated chart image (`certify.rs` `stated_door`, at the
  mint and at the sampled checks);
- `read_incidence` and `read_point_incidence` read outright, the point by
  `spline_incidence::spline_point_incidence`, one walk for the one point.

A carrier off its wall at a read sample ends `CarrierOffChart` by each door's
term (the kernel's defect at a build, the stored culprit at rest through
`validate::off_face_ending`, the damaged file at import), and one in band ends
`Escalated` (`PcurveCheck::ChartIncidence`) in the read's lever, never as the
image's own bound.

**Read on refusal.** On a spline chart a certificate that holds bounds every
carrier point of its own span within the band of the wall (its envelope is a
sup), so where that span covers the one asked about a read adds nothing. Taken
first, the read cost up to 10× on the loft rows (5376 reads of 65 samples in
one elbow row). Taken on refusal, it is off the hot path and an on-chart carrier
that certified on main certifies unchanged. A row re-certified for an edge its
span does not cover is read over the edge (review R1).

**The stated route's meter is sampled**, not a sup, and so is the read: a
carrier off its wall only between both sets of samples gets an edge certificate
there, as on main (review R2). Every production producer closes with a mint
whose envelope catches it; the construction tier does not. Filed:
`a-stated-chart-image-on-a-spline-face-is-certified-by-a-sampled-meter` (P1).

**The own-chart arm** (`topo::pcurves::nurbs_iso_derive`) reads nothing: it
returns the stated image, and every door that stores or validates it ends a
refusal by the read. `restate`'s no-door `Fitted` arm (a dual) now does too.
Pinned by `topo/tests/a_spline_faces_stated_image_ends_by_its_carriers_incidence.rs`,
whose lamina states every edge as its own face's image, read at rest unstored.

**Dual.** `validate_pcurves` at `Dual` no longer answers clean on an off
carrier: an own-chart `General` or `Fitted` image refuses for want of a fitted
door, and the read then ends it `CarrierOffChart`, which `not_owed` does not
excuse (`at_a_dual_an_off_carrier_ends_off_the_chart_not_at_the_fitted_door`,
`at_a_dual_an_off_carrier_is_a_finding`). DL3 still makes the base scalar's
evaluation the validation of record.

**Analytic charts.** `certify_fitted` and `certify_general` now read an analytic
chart's carrier before refusing it `ImageMismatch`, so an off carrier offered
there ends `CarrierOffChart` (`an_analytic_chart_reads_the_carrier_before_the_fitted_lane`).

**The read's foot.** Each sample tries its foot before the walk: Gauss–Newton on
the chart's `f64` shadow from the last sample's foot, its point evaluated in the
scalar and decided as a kept cell's centre is. An on-chart sample reads in a few
evaluations instead of a dive; in band, the foot arms the walk's band prune from
its first visit. A chart too narrow to shrink its rectangle inside is walked
without it (review R3).

Rows:

- `geom-brep/src/pcurve_cache/spline_door_rows.rs`: every door at `f64` and
  `Interval`, on, off and in band; the dual's `certify_general`; R1 and R3; the
  analytic fitted door. The `spline-shown-unread` mutant reds every off and
  in-band reading;
- the topo row above, the same doors through a body, the import term and the
  dual; the same mutant, applied by hand, reds every reading.

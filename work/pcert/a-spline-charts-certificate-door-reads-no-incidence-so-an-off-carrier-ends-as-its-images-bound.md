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
arm over the edge's span (`read_edge_incidence`'s read), and every certificate
door on a spline chart ends a refusal by it (`ended_by_incidence`):

- `PcurveCache::certify` and `recertify`, `certify_fitted` and `certify_general`;
- the stated route (`certify.rs`: `canonical_of` reads an analytic chart first
  as before, and a spline chart's read ends the residual meter's refusal,
  `stated_refusal`);
- `read_incidence` and `read_point_incidence` read it outright, the point by
  `spline_incidence::spline_point_incidence`, one walk for the one point.

A carrier off its wall ends `CarrierOffChart` by each door's term (the kernel's
defect at a build, the stored culprit at rest through `validate::off_face_ending`,
the damaged file at import), and one in band ends `Escalated`
(`PcurveCheck::ChartIncidence`) in the read's lever, never as the image's own
bound.

**Read where the image refuses, not first.** On a spline chart a certificate
that holds bounds every carrier point within the band of a point of the wall
(its envelope is a sup over the span), so a read before it adds nothing there.
Taken first, the read cost up to 10× on the loft rows (5376 reads of 65
samples in one elbow row). Taken on the refusal, it is off the hot path, the
endings are the same for every off or in-band carrier, and an on-chart
carrier that certified on main certifies unchanged. The stated route's meter is
sampled, not a sup, so a carrier off its wall only between its samples passes
there as on main, and the row's own certificate at the mint (an envelope) ends
it.

**The own-chart arm** (`topo::pcurves::nurbs_iso_derive`) reads nothing: it
returns the stated image, and every door that stores or validates it certifies
it through `certify` or `certify_general`, which end a refusal by the read.
Pinned by `topo/tests/a_spline_faces_stated_image_reads_its_carrier_first.rs`,
whose lamina states every edge as its own face's image, read at rest unstored.

**Dual.** `validate_pcurves` at `Dual` no longer answers clean on an off
carrier: an own-chart `General` image refuses at check 3 or check 4
(`FittedLaneUnsupported`), and the read then ends it `CarrierOffChart`, which
`not_owed` does not excuse (`at_a_dual_an_off_carrier_ends_off_the_chart_before_the_fitted_door`,
`at_a_dual_an_off_carrier_is_a_finding`). DL3 still makes the base scalar's
evaluation the validation of record.

**The read's foot.** Each sample now tries its foot before the walk:
Gauss–Newton on the chart's `f64` shadow from the last sample's foot, its point
evaluated in the scalar and decided as a kept cell's centre is. An on-chart
sample reads in a few evaluations instead of a dive; in band, the foot arms the
walk's band prune from its first visit.

Rows:

- `geom-brep/src/pcurve_cache/spline_door_rows.rs`: every door at `f64` and
  `Interval`, on, off and in band, and the dual's `certify_general`; the
  `spline-shown-unread` mutant reds every off and in-band reading;
- the topo row above, the same doors through a body, the import term and the
  dual; the same mutant, applied by hand, reds every reading.

---
id: plane-nurbs-refusal-ending-groups-the-producers-refusals-into-none
kind: issue
title: PlaneNurbsRefusal::ending ends the image producer's refusals in a grouped None
status: closed
opened: 2026-11-03
priority: P1
cost: M
closed: 2026-11-03
pr: 6474
---


(PCERT implementer, from the review of PR 6448, `pcert/certify-arms-end`,
fix S1; the same defect class as `pcurve-certify-error-arms-with-no-ending`,
which that PR closed for `PcurveCertifyError`.)

## What

- `geom_brep::PlaneNurbsRefusal::ending` (`crates/geom-brep/src/edge_nurbs.rs`)
  answers `self.decision()` for every arm it does not name, and
  `PlaneNurbsRefusal::decision` returns `None` in one grouped arm for
  `FootPointInconclusive`, `CarrierDomain`, `CarrierEnclosure` and
  `Unsupported` (with `TubeNotOneArc` and `PcurveFit`, which `ending`
  names first). The edge door reports those four with no recourse.
  `AnalyticRung3Refusal::ending` has the same grouped `None` over
  `NoOffsetBound`, `CarrierEnclosure`, `NoTubeEnclosure` and `Unsupported`.
- Their decisions exist elsewhere: the pcurve door ends the same producer
  refusals since PR 6448 (`PcurveCertifyError::decision`), as follows:
  - a foot point that will not settle by limb 1's decision
    (`SsiError::FootPointInconclusive`'s ending);
  - the carrier domain by `CARRIER_DOMAIN_RECOURSE`;
  - the carrier enclosure "not yet";
  - the placeholder wall by `PLACEHOLDER_RECOURSE`.
- `CertifyError::NoGeneralLane { gap: Pose, .. }` (`crates/geom-brep/src/certify.rs`)
  states the pose lever in its payload ("Recourse: {POSE_LEVER}") and ends
  by `CertCheck::GeneralLane` ("not yet"): two recourses that disagree.
  `PcurveCertifyError::NoGeneralLane { gap: Pose, .. }` does the same
  (`PcurveCheck::GeneralLane`'s routing is "not yet"). At a real door the
  lever is the true one: `topo` `m6_3_chart_completion::the_pole_fence_refuses_the_arc_over_the_pole_and_mints_the_arc_clear_of_it`
  pins that an arc over a sphere's pole carries a recourse, and splitting
  it at the pole is a way through, so "There is no way through yet" is
  false there. PR 6448 left both doors' pairs as they were (its every-arm
  row allows this one arm two recourses).

## Repair shape

Name each arm's decision in an exhaustive match, ending by the pcurve
door's table where the refusal is the same producer's. For the pose gap,
decide whether the decided pose ends in its lever (`LeverOnly`) rather
than `GeneralLane`'s "not yet", at both doors, and state one recourse.

## The pose gap: still open (review of PR 6474, M4 and M5)

PR 6474 first ended the pose gap in its lever at both doors, and the
review reverted it: the pair stays (the lever in the payload, D3's "not
yet" as the ending), and this question stays here. The lever's premise
— that every `LaneGap::Pose` is a carrier *through* a pole, the cone's
apex or the torus's core — does not hold at every instance:

- `pcurve_cache::general_lane_refusal` relabels every `SectorRefused`
  and every `Sector` escalation of the projected lane as the pose gap
  (`NoGeneralLane { gap: Pose }`, `Escalated { check: GeneralLane }`),
  including poses that pass *beside* the singular set in band and never
  through it. The lane's own rows reach them:
  - `pcurve_cache::harmonic_sup_rows::a_circle_near_a_pole_the_general_lane_cannot_image_ends_not_yet`:
    "a great circle tilted mid-band off a meridian" refuses
    `NoGeneralLane { chart: Sphere, carrier: Circle, gap: Pose { piece: 18, channel: Azimuth } }`
    at `f64` and at `Interval`, at a build and at rest, and "a small
    circle on a vertical plane mid-band off the axis" is read the same
    way;
  - `pcurve_cache::envelope_lemma_fuzz::projected::a_spline_net_the_projected_lane_cannot_partition_ends_not_yet`:
    "an arc mid-band beside the pole" (the sphere's `beside` net).

  For these, "split the edge at it" names a passage the edge does not
  make; re-aiming the chart's axis may still be a way through.
- The same apex fact ends two ways. Through the analytic rung-3
  certificate the cone's apex reaches `AnalyticRung3Refusal::of_offset`
  as a sector refusal and ends `NoOffsetBound` "not yet" (the roster's
  `NoOffsetBound { operand: Cone, why: SectorChannel::Lever.describe() }`);
  through the lane choice it is `NoGeneralLane { gap: Pose }`, whose
  payload states the pose lever.

So the decision needs the gap told apart first: a pose *through* the
singular set (where the lever is a way through) from one only in band
beside it (where it may not be). Until then the pair stands.

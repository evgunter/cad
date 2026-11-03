---
id: the-pose-gap-cannot-tell-through-the-singular-set-from-beside-it
kind: issue
title: The general lane's pose gap is one label for a carrier through a pole/apex/core and one only in band beside it, so it cannot end in the pose lever
status: open
opened: 2026-11-03
priority: P2
cost: M
---


Moved by the PCERT orchestrator from `plane-nurbs-refusal-ending-groups-the-producers-refusals-into-none`, which PR 6474 closed with the question still open.

## The question

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

Banded P2: until the gap is told apart, the pair (lever in the payload, D3's "not yet" as the ending) is honest; nothing ends wrongly, it only ends less helpfully.

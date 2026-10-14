---
id: two-coplanar-ellipses-have-no-meeting-method
kind: issue
title: The curve-pair reader refuses two coplanar ellipses that are not one curve, and refuses a near-tangent coplanar circle and ellipse instead of placing the touch
status: open
opened: 2026-10-14
priority: P2
cost: M
refs: [the-curve-pair-reader]
---

Found while building the curve-pair reader (`work/fuse/the-curve-pair-reader.md`, stage (b)).

Two poses of `geom_brep::curve_pair::pose` (`crates/geom-brep/src/curve_pair/pose.rs`, `conic_conic`) answer with a typed refusal where a closed form could answer:

- **Two ellipses in one plane that are not one curve** refuse `MeetFault::Unsupported`. The one-curve reading (`pose::one_curve`, PR 5058's Procrustes margin `one_curve_gap`) decides coincidence. Past it there is no metre residual: the second ellipse's implicit along the first is of degree two, but its distance has no closed form. Two routes are open:
  - a residual whose floor and ceiling per metre of distance are certified near the ellipse, for the subdivision's frame;
  - the certified lower bound on an ellipse's distance that `splitting::containment`'s `ConicArc::hit` already reads.

  The boolean's crossing layer answers `Unread` on this pose. A total reader still owes it an answer.
- **A near-tangent circle and ellipse in one plane** go to `certified_subdivision`, which refuses `Unsettled::Tangency` at a double root, where the circle pair's arm places the touch by the root gap. The subdivision refuses when a piece narrower than the band is neither clear nor monotone. The arm could instead place the touch on that piece by the degenerate formula, as the root gap does for two circles.

**Why P2.** `carrier_cross` answered `None` on this pose before it read `meet`, so its switch moved no verdict there; it answers `Unread`, and its caller keeps its frontier door. The census confirm is different. Its own walk (`census::curved::reads_meet`, retired in stage (c)) decided two coplanar ellipses that are not one curve. Reading `meet`, it refuses typed on both edges. The unit row `census::curved::tests::the_reading_answers_a_meeting_on_every_kind`, two ellipses at right angles meeting near `(a, a)`, `a² = 4/5`, moved from "meets" to that refusal. The topo and sweep `ci` profiles move no body-level verdict, so no body door is known to reach it. An edge-edge record on two elliptical rims in one plane would now refuse its confirm (`CarryRefusal::Unread` through `records::landings`).

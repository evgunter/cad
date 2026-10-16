---
id: two-coplanar-ellipses-have-no-meeting-method
kind: issue
title: The curve-pair reader refuses two coplanar ellipses that are not one curve, and refuses a near-tangent coplanar circle and ellipse instead of placing the touch
status: open
opened: 2026-10-14
priority: P3
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

**Why P3 under a P1 unit.** `carrier_cross` reads `meet`, and answers `Unread` on this refusal, as its own `(Ellipse, Ellipse)` arm answered `None` before. The caller keeps its frontier door either way, so no verdict moved here.

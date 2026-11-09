---
id: arc-trim-margin-stayed-in-containment-when-arc-trim-moved-beside-check-9
kind: issue
title: Span membership is spelled two ways: arc_trim's chord distances (check 9) and span_place's parameter·meter
status: open
opened: 2026-11-09
---


Found by the delta review of PR 6609 (`onecopy/loop-walks`, head 894d822f), finding F2.

PR 6609 moved `arc_trim` and `ArcTrimRows` beside check 9 in `topo/src/validate.rs`. Two loose ends remain:

1. `arc_trim_margin` stayed at `topo/src/splitting/containment.rs:~1049`, although its only consumer is `validate.rs:~9998`. Move it beside its consumer.
2. Its doc says it is "for a caller that has decided the ends by another metric first (an ellipse's exact end distances, [`ConicArc::hit`])". `hit` does not call it, so the sentence is stale. Correct it.

Also still open from that review (S4, style): span membership is spelled two ways. Name both, and route one through the other.

## Narrowed (2026-11-09, PR 6641)

Items 1 and 2 are done. `arc_trim_margin` is a private fn beside
`arc_trim` in `topo/src/validate.rs`, and the stale caller is gone from
its doc.

What remains is S4, in the review's words (the 6609 delta review): "two
spellings of 'is this point on the arc's span'. Check 9 (`arc_trim`)
reads chord distances. The walk and the region doors now read
`span_place`'s parameter·meter." The two sites:
- `topo/src/validate.rs` `arc_trim`, check 9 and `window`'s arc arm. It
  takes a POINT and decides its distances from the ends, then a sum of
  chordal defects.
- `geom-brep/src/curve_face/plane_crossing.rs` `span_place`, the
  plane-crossing lane and `topo/src/boolean/torus_region.rs`. It takes a
  PARAMETER and decides `(t − t₀)·meter` and `(t₁ − t)·meter`.

They ask one question from different inputs, and neither routes through
the other honestly as they stand:
- Routing `arc_trim` through `span_place` needs the point's parameter.
  Check 9 reads distances precisely so that it never inverts a point to
  an angle (its docs: an in-band radius would escalate a point half a
  turn away).
- Routing `span_place` through `arc_trim` needs `arc_trim`'s chord
  monotonicity, which holds on a circle only. `span_place` serves ellipse
  spans too.

A fix is a shared decided-membership type that both produce, or a
ruling that the two metrics are two questions. That is a design choice,
not a move.

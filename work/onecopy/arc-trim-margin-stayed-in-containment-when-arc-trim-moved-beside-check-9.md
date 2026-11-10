---
id: arc-trim-margin-stayed-in-containment-when-arc-trim-moved-beside-check-9
kind: issue
title: Span membership is spelled two ways: arc_trim's chord distances (check 9) and span_place's parameter·meter
status: closed
opened: 2026-11-09
closed: 2026-11-09
pr: 6687
branch: onecopy/needle-basin
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

## Findings (onecopy/needle-basin)

**There were three spellings, not two.** `geom-brep`'s
`curve_pair::Span::place` already read a point's foot against a span:
an end by the raw chord to its point (`curve_pair_span_end_chord`),
else the side by the foot's parameter metered at the span's SUP speed
(`curve_pair_span_end_side`), with ellipses' `Past` walked. Check 9's
`arc_trim` and the plane door's `span_place` were the other two.

**The question's own reading is `Span`'s.** "At an end" is a distance
in metres, which the chord reads directly on any carrier, so the chord
does not rely on `arc_trim`'s circle-only monotonicity. "Inside or
past" is a side, which the parameter reads without inverting the point.
Metered at the sup speed, a side the band calls Zero lies within the
band in arc length and so within it in chord. `span_place`'s
parameter·INF-meter could not route there: its Zero over-reaches by the
speeds' ratio, a wrong AtEnd on a thin ellipse (filed and fixed as
`a-plane-crossing-past-the-band-of-a-thin-ellipses-fast-end-read-at-the-end`).

**Routed.** Check 9's `window` arc arm calls `Span::place` on the
point. A circle's foot is its radial projection, so no angle is
inverted by a walk, and `arc_trim`, `arc_trim_margin` and
`ArcTrimRows` are gone. The plane door and `torus_region::arc_place`
call `Span::place_param` through `span_place`, which shares
`ends_place` with `Span::place_at`. The K name
`plane_crossing_root_in_span` is retired.

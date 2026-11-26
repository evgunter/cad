---
id: the-ring-composites-refusals-and-end-readings-are-translated-per-caller
kind: issue
title: The ring composite's refusals and end readings are translated at each of its callers, with divergent mappings, and two callers decide its deferred ends again on their own margins
status: closed
opened: 2026-11-25
priority: P1
cost: M
refs: [point-in-solid-refuses-a-sphere-face-bounded-by-a-spline-chord]
rides_with: the-face-door-reads-no-spline-arc-of-a-sphere-face
closed: 2026-11-26
pr: 7801
branch: inside/face-door-spline-arm
---


Filed on the fix pass of PR 7755 (`inside/sphere-chord-region`), from its
review.

## What

The ring composite (`geom_brep::curve_face::composite::crossings`,
reached as `topo::props::AtRestPolicy::spline_face_crossings`) answers
`Result<SpanCrossings<T>, CompositeRefusal>`. Each caller translates the
refusal into its own vocabulary, and the translations disagree:

| refusal | `splitting::classify` (`composite_fault`) | `boolean::reduce` (`composite_count`) | `region::spline_landings` |
|---|---|---|---|
| `Escalated` | `MeetFault::Escalated` | off-face check, then the lane's `escalated` | path set aside in band |
| `Unsettled` | `MeetFault::Unsettled` | off-face check, then `SplineCrossingUnsettled` | `Tangency`/`Crease` graze; the rest block |
| `FaceDegenerate` | `MeetFault::Unsupported` | `InvalidInput` | blocked |
| `Unread` | `MeetFault::Unsupported` | `ClassificationInvariant` (a kernel fault) | blocked (a limit) |

`boolean::slit` (`spline_crosses`) reads only whether it answered.

Since PR 7755 the composite also reports how the span stands at each
end (`SpanCrossings::ends`, `EndReading`): a near end's meeting is the
end's, on a stretch it states. `region::spline_landings` reads that and
nothing else. `classify` and `reduce` read `roots` alone and decide the
span's end vertices on their own margins (the split's vertex sides, the
lane's own end handling). That is the second-decision shape PR 7755's
review found losing a crossing in the region door. `classify` guards
one case of it (`MeetFault::Contradiction`: sides opposite, no root).
It has not been measured whether a crossing deferred to a near end can
reach either caller unguarded.

## Owed

One home for what a composite refusal means (which are band readings,
which are limits, which are kernel faults), read by every caller. Each
caller that decides a span's ends should read `SpanCrossings::ends`
rather than decide them again, or say at its site why its own reading
is at least as conservative.

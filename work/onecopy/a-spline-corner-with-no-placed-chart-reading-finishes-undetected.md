---
id: a-spline-corner-with-no-placed-chart-reading-finishes-undetected
kind: issue
title: A solid touching itself at a corner whose face has no sector arm, a placeholder chart, or no certified curve or pcurve there finishes undetected: corner_normal names SectorFaceError::Unsupported and Unplaced as the curved reading's abort (Curved(NoSectorArm), Curved(Unplaced)), which goes to SelfTouch::SecondOrder and is skipped
status: open
opened: 2026-11-10
priority: P3
cost: E
---


Split off from
`work/onecopy/a-solid-touching-itself-at-a-spline-crease-corner-finishes-undetected.md`
by the orchestrator's ruling on it: that row routes the `Crease` arm, and
these two arms are their own row, not folded in.

## What

`census::corner_normal` (`crates/topo/src/census.rs:9138`) names two of
`sector_face::resolve`'s refusals as the curved reading's abort
(`corner_cause`, `census.rs:9167`; named causes since PR 6726):

- `SectorFaceError::Unsupported` (`crates/topo/src/sector_face.rs:263`)
  reads as `TouchUnread::Curved(CurvedUnread::NoSectorArm)`: the face's
  carrier has no sector arm, or its spline chart is a placeholder.
- `SectorFaceError::Unplaced` (`crates/topo/src/sector_face.rs:288`) reads as
  `TouchUnread::Curved(CurvedUnread::Unplaced)`: the spline face's
  half-edge at the vertex has no pcurve, or its edge no certified curve.

`self_touch_verdict` sends a curved abort (`TouchUnread::curved_unread`)
to `SelfTouch::SecondOrder`, which `sweep_self_touch_points` skips as
undetected (`census.rs:8610`, `SelfTouch::SecondOrder(_) => continue`).
The census module's undetected list names the class ("a spline corner
face on a placeholder chart or with no certified curve or pcurve there").

Neither is a reading of the geometry: both are a body the corner reader
was not handed enough of. The name now says which, but the touch still
goes undetected.

## The decision

Whether these arms stay undetected (and `corner_normal` says why), or
refuse: by which `TouchUnread` (a structural refusal such as `Corrupt` or
`Unanalysed`), and so whether tier 3 refuses such a body with
`BoundaryCrossesItselfUndecided`. Either way, pin it with a row: a body
touching itself at such a corner, and the gate's outcome on it.

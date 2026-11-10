---
id: a-spline-corner-with-no-placed-chart-reading-finishes-undetected
kind: issue
title: A solid touching itself at a corner whose face has no sector arm, a placeholder chart, or no certified curve or pcurve there finishes undetected: corner_normal reads SectorFaceError::Unsupported and Unplaced as Unreadable, which goes to SelfTouch::SecondOrder and is skipped
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

`census::corner_normal` (`crates/topo/src/census.rs:9072`) reads two of
`sector_face::resolve`'s refusals as `TouchUnread::Unreadable`:

- `SectorFaceError::Unsupported` (`crates/topo/src/sector_face.rs:263`):
  the face's carrier has no sector arm (`sector_face.rs:460`), or its
  spline chart is a placeholder (`sector_face.rs:491`).
- `SectorFaceError::Unplaced` (`crates/topo/src/sector_face.rs:288`): the
  spline face's half-edge into the vertex has no pcurve, or its edge no
  certified curve (`sector_face.rs:506`, `:511`).

`self_touch_verdict` sends `Unreadable` to `Link::tangent_of` and then to
`SelfTouch::SecondOrder`, which `sweep_self_touch_points` skips as
undetected (`census.rs`'s `SelfTouch::SecondOrder => continue`). The
census module's undetected list names the class ("a spline corner face on
a placeholder chart or with no certified curve or pcurve there").

Neither is a reading of the geometry: both are a body the corner reader
was not handed enough of.

## The decision

Whether these arms stay undetected (and `corner_normal` says why), or
refuse: by which `TouchUnread` (a structural refusal such as `Corrupt` or
`Unanalysed`), and so whether tier 3 refuses such a body with
`BoundaryCrossesItselfUndecided`. Either way, pin it with a row: a body
touching itself at such a corner, and the gate's outcome on it.

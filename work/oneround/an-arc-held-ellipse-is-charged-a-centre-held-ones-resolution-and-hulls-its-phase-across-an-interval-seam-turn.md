---
id: an-arc-held-ellipse-is-charged-a-centre-held-ones-resolution-and-hulls-its-phase-across-an-interval-seam-turn
kind: issue
title: an arc-held ellipse is charged a centre-held one's resolution, and an Interval seam turn that straddles hulls its phase
status: open
opened: 2026-11-23
priority: P2
cost: M
refs: [7203]
---


Two things the reviews of PR 7203 (`oneround/anchored-conic`) left open,
moved here from the unit's row when it closed.

- `carrier_resolution::resolution`
  (`crates/geom-brep/src/carrier_resolution.rs`) charges an ellipse held
  on its arc `u` times its semi-axis, as it charges one held at its
  centre, and its module doc says an ellipse resolves its arc "only to
  about `u·A`". Near an anchor at the seam's own vertex the point holds
  to its own coordinates' rounding, so the charge is conservative there.
  Whether any reader should read less is not decided.
- A seam turn decided on an `Interval` that straddles
  (`Ellipse::quarter_turned_where`, `seamed_toward`) hulls the anchor's
  phase to `([0, 1], [−1, 0])`, and the derived centre is then about a
  semi-axis wide, where an ellipse held at its centre keeps its centre
  exact through the same turn. Not probed.

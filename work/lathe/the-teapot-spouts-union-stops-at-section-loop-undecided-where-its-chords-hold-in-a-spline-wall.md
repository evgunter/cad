---
id: the-teapot-spouts-union-stops-at-section-loop-undecided-where-its-chords-hold-in-a-spline-wall
kind: issue
title: The teapot spout's union stops at SectionLoopUndecided where its section's chords are held by a spline wall, at ε 1e-9 and 1e-6, where main gets past the join
status: open
opened: 2026-11-26
priority: P0
cost: M
refs: [curved-demo-joins-build]
---

## What

The tour's teapot wall 3 (`demos/tour/src/teapot.rs` `wall3_pinned`)
is the lofted spout ∪ the vessel. On main `5e04ac6561` and on the
one-band stack `a8a9d3a9b6`, at ε 1e-9 and 1e-6, it gets past the join
and stops at the result's volume
(`quad/a-rational-spline-face-with-a-general-row-has-no-volume-lane`).

On PR 7414's head, after main's merge, the join's section-loop ladder
reaches a chord whose holding face is the spout's NURBS wall. The
measured point is `q ≈ (−0.0755, 0.0800, 0.0102)`, held in `FaceKey(7v1)`.
`join.rs` `holding_face_normal` read that face's missing normal arm as
a stale key and refused `JoinDesync`: a kernel-fault class for what is
a limit. It now reads it as one (`HoldingUnread::Spline`, a
`Refusal::Limit`, as an unread trim is), and the chord is passed. The
loop then reads no side, and the union refuses typed, at ε 1e-9 and
1e-6:
`Join(SectionLoopUndecided { face })`. At 1e-12 the wall stops where it
did before, at the at-rest arc image
(`sectarms/a-curved-partners-arc-is-imaged-at-rest-no-finer-than-the-mints-finest-rung`).

So the merge's geometry, with the vessel's belly one band, puts a
section chord inside a spline wall, where neither parent did. The join
gets less far than on main.

## Not yet known

Which chords of the loop are held by the spline wall, and whether any
chord could decide the loop if a spline face's outward normal at a
point were read (`face_normal::spline_outward_normal_at` takes the chart
point, which the holding read does not carry).

## Pin

`wall3_pinned` pins `SectionLoopUndecided` at ε 1e-9 and 1e-6.
`walls::CURVED_JOIN_STOPS` names this row for teapot wall 3.

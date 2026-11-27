---
id: the-teapot-spouts-union-stops-at-section-loop-undecided-where-its-chords-hold-in-a-spline-wall
kind: issue
title: The teapot spout's union stops at SectionLoopUndecided where its section's chords are held by a spline wall, at ε 1e-9 and 1e-6, where main gets past the join
status: review
opened: 2026-11-26
priority: P0
cost: M
refs: [curved-demo-joins-build]
pr: 7824
branch: lathe/teapot-spout-section-loop
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

## Found

Measured 2026-11-27 at ε 1e-9 and 1e-6 on `lathe/teapot-spout-section-loop`.
The loops that reach the chord reading are on the vessel's side: four
region faces (`10v5`, `12v5`, `13v9`, `1v9`) with sixteen chords
between them. Every one of the sixteen is held strictly inside one of
the spout's four NURBS walls (`7v1`–`10v1`), and each placement there
read `In` at an in-trim foot. So every chord was passed as a limit, and
no chord was left to decide.

The point can be carried. `contain::placement_unheld` already returns
the in-trim foot a spline face's `In` placement was decided at, and
`holding_face_normal` dropped it. It now keeps it
(`contain::face_placement_charted`) and reads the spline wall's normal
there (`join::spline_held_normal`, over
`face_normal::spline_outward_normal_at`). This is the same reading
`slit::spline_floor` and `vtxfac`'s pierced spline face already take,
under the same premise: the placement certifies that no second
in-trim foot lies within the band
(`work/splice/the-spline-placement-read-certifies-no-second-in-trim-foot`).
Where that normal refuses, the refusal is the normal's own
(`HoldingUnread::Normal`). Where the placement read no foot (a spindle
sheet), the chord is still passed as a limit (`HoldingUnread::Spline`).

With the foot read, all sixteen chords decide. `12v5` and `13v9` read
In, `10v5` and `1v9` read Out, and they agree within each loop. Wall 3
gets past the join and stops at the result's volume
(`quad/a-rational-spline-face-with-a-general-row-has-no-volume-lane`),
as it did on main before the one-band merge.

What is not taken: a chord whose REGION is a spline face still reads
no normal (`HoldingUnread::Spline` on the region arm). The chord's
point lies on that region's boundary, so its placement there is
`OnEdge` and carries no foot. The teapot does not reach that arm.

## Pin

`wall3_pinned` pins the volume lane's `QuadratureUnsupported` at
ε 1e-9 and 1e-6. `walls::CURVED_JOIN_STOPS` names that row for teapot
wall 3. `join::holding_rows`
`a_chord_held_inside_a_spline_face_reads_its_normal_at_the_foot` pins
the reading in both operands.

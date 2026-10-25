---
id: an-adjacent-pair-through-a-joint-straight-within-the-band-escalates-on-chord-side
kind: issue
title: An adjacent pair through a joint straight within the band escalates on chord_side, though its only possible contact is the shared vertex
status: open
opened: 2026-10-21
priority: P2
cost: M
refs: [validate-reads-in-band-carriers-before-spans-in-line-line-arc-arc, an-adjacent-pairs-shared-vertex-is-recomputed-as-a-root, 5542]
---

## What

Found in the FULL review of PR 5542 (CPAIR's one-line PR 3, which reads
`seg::line_line` as check 9 reads two straight edges). The review's
NOTE witness from PR 5536, built by the path builder, is
`at(0,0).angle(0).line(1).line_to(0,2).line_to(−2, 0.9ε).angle(0).line(1).continue_to(−0.5, 1.8ε).continue_to(Start.arrives_tangent())`.
Validation escalates `SegmentPair(0, 5)` on `chord_side` (`1.8ε`) at
the default ε, on main and with PR 5542
(`crates/profile/tests/rejections.rs`,
`an_adjacent_pair_through_a_joint_straight_within_the_band_escalates`).

Segment 0, `(0,0)→(1,0)`, and segment 5, `(−0.5, 1.8ε)→(0,0)`, are
adjacent: they share `Start`, through a joint straight within about
`3.6ε`.
- **The sides.** Each far end stands in band of the other's carrier,
  and the shared vertex reads zero on both, so no carrier side
  separates them (`crates/profile/src/seg.rs`, `line_line`).
- **The gap.** Their projections on either axis touch at the shared
  vertex, so the gap reads zero, not apart (`gaps_apart`).

So `line_line` escalates on the in-band side. Whether the joint is read
straight (collinear: `collinear_overlap` reads a touch at the vertex)
or turned (a definite side: `touches_from` reads a touch at the vertex),
the verdict is the same: the pair's only contact is the shared vertex,
which the pass discounts. The escalation is loud, not wrong.

## Proposed shape

Where the pair's overlap on one segment's axis reads Zero both ways
(`collinear_overlap`'s `extent ∓ noise`, through `geom_core::one_line`),
and the touching ends are one stored vertex both segments share
(`coincident`), the pair's contacts are a Touch at that vertex and
nothing else.

**Soundness.** Let the overlap be on `s1`'s axis `û`, touching at
`s1.a`'s projection, with `|extent| + noise` within the band's zero. A
contact point `p` lies on both segments, so its projection lies in both
projected intervals, within the band of `s1.a`'s. On `s1` the
projection is the arc length along it, so `p` is within the band of
`s1.a` along `s1`. Every contact is therefore within about the band of
the shared vertex, which is that vertex's touch as the pass already
reads it. No angle between the segments enters, so the rule holds at a
straight joint, a turned one and a reversal alike.

**One reading across both doors.** topo's check 9 (`lines_meet` in
`crates/topo/src/validate.rs`) reads two straight edges the same way, so
an adjacent pair of edges through a joint straight within the band
reaches the same in-band side there (not measured here). The rule
should land in both doors, or in a helper both call, as one reading, as
`gaps_apart` and the carrier short-circuit now are.

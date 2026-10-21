---
id: an-adjacent-pair-through-a-joint-straight-within-the-band-escalates-on-chord-side
kind: issue
title: An adjacent line pair escalates at its joint, not its pair
status: closed
opened: 2026-10-21
priority: P3
cost: M
refs: [validate-reads-in-band-carriers-before-spans-in-line-line-arc-arc, an-adjacent-pairs-shared-vertex-is-recomputed-as-a-root, 5542]
closed: 2026-10-21
branch: nurbs/adjacent-shared-vertex
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

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to PROFILE (`work/profile/`) when PATHS closed. PATHS's live rows went to four programs opened at this close and cut along the priority seam; PROFILE holds the goal (`docs/doc-ledger/paths-leaves-the-tracker.md`). The item's id, content and history are unchanged.

## Closed

**An adjacent line pair escalates at its joint, not its pair.** Re-rated
P3: it is real but narrow. No loop that escalated on main validates now;
the escalation moves to the question it is, and a definite refusal later
in the pass is read.

- **The rule.** `profile::seg::line_line`, given the stored vertex the
  pair shares (`validate::judge_pair`, by index), asks
  `touches_only_near_the_shared_end` where a carrier side escalates: the
  overlap along the first segment, its upper bound `extent + noise` read
  on `collinear_overlap`. Read no more than Zero, every shared point is
  within the band of the vertex, and the pair's contacts are a Touch
  there, which the adjacency discount reads. The overlap's lower bound
  does not enter, so the item's "Zero both ways" was more than it needs.
- **Why the joint still escalates.** The pair escalates on a side only
  where the short leg's far end stands in band of the long leg's
  carrier, `sin θ·Ls`; were that Zero, the collinear arm would decide
  the pair. The joint pass reads `chord_side` of the leaving leg's far
  end, then `path_junction_turn`, `|sin θ|` levered by the shorter arm:
  `sin θ·Ls` again. So every rescued pair escalates at its joint, which
  `seg`'s `every_pair_the_shared_end_touch_rescues_escalates_at_its_joint`
  pins over a sweep of `θ` and `Ls` across the band. A change to the
  joint pass that lets one through reds there, and owes this rule's
  soundness its review then.
- **What it buys.** A crossing later in pass order was masked:
  `(0,0) (1,0) (1,1) (1.2,0.5) (−0.1,h)`, `h` in band, escalated
  `SegmentPair(0, 4)` on `chord_side` on main and never read the
  crossing of segments 1 and 3; it now refuses `NonSimple` there
  (`rejections::a_crossing_after_an_adjacent_pairs_in_band_side_refuses`).
  The review's witness escalates at its joint, `SegmentPair(5, 0)` on
  `chord_side`, as its `0.9ε` sibling did.
- **The straddle is not asked, deliberately.** Where the pair reads
  collinear and `collinear_overlap` straddles its charge (lower bound in
  band, upper Zero), the rule holds and the joint reads a continuation
  (`SameCarrier`), so asking it there would accept a pair main
  escalated. That is correct by the rule's argument and deliberately not
  accepted: it lives where the overlap's noise passes ε, about
  `3·10⁵` m out at ε `10⁻⁹`: 8 of 24 716 rescues in the coupling sweep
  when it was asked on any escalation.
  The witness is
  `seg::a_straight_continuation_straddling_its_overlaps_charge_still_escalates`
  (a `5.4·10⁵` m leg into `(4195.4, 1159.4)`, a `960` m leg on); a
  candidate if a real body ever needs it.
- **One door, not both.** topo's check 9 has no adjacent pair to read:
  it pairs a ring's edges with its outer loop's, and a vertex the two
  loops share is its arm 1's contact (`ring_outer_vertex_gap`) before any
  edge pair reaches `lines_meet`. topo has no same-loop pair reader
  (global self-intersection is tier 3's named residue). Pinned by
  `topo` `check_9_refuses_a_ring_sharing_a_vertex_through_a_straight_joint_at_that_vertex`.

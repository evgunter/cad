---
id: validate-reads-in-band-carriers-before-spans-in-line-line-arc-arc
kind: issue
title: "The simplicity pair pass escalates two arcs on nearly one circle on carrier_circles_identity, however far apart their spans lie"
status: open
opened: 2026-10-07
priority: P2
cost: M
---


Found by the class sweep of
`a-straight-arrival-off-an-arc-departure-escalates-in-carrier-line-circle`,
which fixed two instances in `crates/profile/src/seg.rs`: `line_arc`
now asks `arc_clear_of_carrier` before an in-band
`carrier_line_circle` escalates, and `joint` answers "no contact" on
either span's definite miss of a secant candidate before the other's
in-band reading escalates.

## The shape

The simplicity pair pass (`seg::pair_contacts`, called by
`validate.rs` `judge_pair`) decides a CARRIER question first and
escalates on an in-band verdict before asking whether the contact it is
about lies on both segments. Each sibling below is measured with a
scratch probe at the default ε, every margin placed at
`ε·(1 + k)/2`, and each refuses `Escalated` although the two segments
are at least 1/3 apart:

| classifier | fixture | refusal |
|---|---|---|
| `line_line` | `chain` (0,0) (2,0) (2,2) (4,2) (4,b) (6,−1) (6,3) (0,3): `(4, b)` in band of segment 0's carrier, 2 past its end | `SegmentPair(0, 3)`, `chord_side` 5.5e-9 |
| `line_arc`, foot off the LINE span | `chain` (−1,0) (1,0) (1,−1) (5,−1) (5,5) (3,5) (3,3, −(3 − b)) (1,3) (−1,3): the major arc's carrier band above y = 0 at x = 2, the line ending at x = 1, 1/3 below the arc | `SegmentPair(0, 6)`, `carrier_line_circle` −5.5e-9 |
| `arc_arc` | two half-discs, `chain` (0,1, 1) (0,−1, 0) and (2+b,−1, 0) (2+b,1, −1): circles externally in band at (1, 0), each arc on its far side | `SegmentPair((0,0), (1,1))`, `carrier_circles_external` 5.5e-9 |

The fixed `line_arc` certificate reads the arc's side only: the arc's
span definitely excludes the circle's point facing the line and both
arc endpoints lie definitely on the centre's side. The line's side
(foot definitely off the line's span and the line's nearer endpoint
definitely outside the circle) needs a radial margin no predicate in
the inventory names; `arc_arc` needs the same per arc against the other
circle; `line_line` needs the endpoint's distance along the other
carrier.

## The tangent arms: measured, and split out

`line_arc`'s decided-Zero arm, and `arc_arc`'s two tangent arms, took
one candidate at the tangency point and span-checked it alone. That
silent miss was filed and fixed on its own, at P0:
`validate-settles-a-tangent-pair-on-one-candidate-and-misses-a-touch-within-eps`.

A candidate that a span definitely misses now leaves the pair's ends to
read (`seg::end_touches`). This holds in the tangent arms and in the
secant arms, where a shallow crossing had the same hole. That fix does
not touch the siblings above: each still escalates on an in-band
carrier reading before any candidate is formed.

## line × line: resolved (2026-10-21)

`seg::line_line` reads two straight segments as check 9 reads two
straight edges (`topo::validate`'s `lines_meet`), so the two doors give
one answer:
- first the gap between the two spans' projections on either one's
  axis, through `geom_core::one_line::on_one_line_2d` (`gaps_apart`,
  predicate `segment_gap`), no contact where it is apart past the band;
- then the four `chord_side` readings, no contact where both of one
  segment's ends lie definitely on one side of the other's carrier,
  however the other two read;
- then, where both of one segment's ends read within the band's zero
  of the other's carrier, the overlap along that carrier
  (`collinear_overlap`), however the other pair of sides reads;
- only then does an in-band side escalate.

The points are the loop's stored vertices (`build_loop_seg`), so
`produced` is zero. The collinear arm reads the same home
(`collinear_overlap`). The table's fixture validates at `f64` and
`Interval` as authored, re-seamed and reversed
(`rejections::an_end_in_band_of_a_carrier_but_past_its_segment_is_no_contact`),
and so does an end in band of a carrier over an overlapping projection
(`…_over_an_overlapping_projection_is_no_contact`), and so do two
segments in band of one line a metre apart along it
(`two_segments_in_band_of_one_line_a_metre_apart_along_it_are_no_contact`).
A short segment continuing a long one straight within ε is simple
(`a_short_segment_continuing_a_long_one_straight_within_eps_is_simple`),
and a loop returning to a vertex that way touches itself
(`a_loop_returning_to_a_vertex_along_a_straight_continuation_touches`).
This is PR 3 of
`work/cpair/two-spans-projections-on-one-line-are-read-four-ways`
(PR 5542).

**Not covered here.** An adjacent pair through a joint straight within
the band shares a vertex, so its projections touch and its sides read
zero there. It still escalates, and is filed as
`an-adjacent-pair-through-a-joint-straight-within-the-band-escalates-on-chord-side`.

`line_arc` (foot off the line's span) and `arc_arc` are resolved by the
radial certificates below.

## Narrowed (2026-10-21) to the identity arm

The table's three rows are resolved. What is left is the arm no radial
margin reaches.

- **line × line**: the section above (PR 5542).
- **line × circle**, the foot off the line's span: where
  `carrier_line_circle` is in band, `segment_clear_of_circle` answers
  no contact if the centre's foot reads definitely past the segment
  (`circle_foot_past_segment`, charged for the foot's making) and both
  ends read outside the circle (`circle_side`). The table's fixture
  validates (`rejections::a_line_ending_short_of_an_in_band_graze_is_no_contact`).
- **circle × circle**, external and the internal sibling the table
  lacked: where `carrier_circles_external`, `_internal` or `_chord` is
  in band, `arc_clear_of_circle` answers no contact if either arc's
  span reads definitely past its point nearest the other centre with
  both ends outside the other circle, or past its farthest point with
  both ends inside (`circle_extreme_past_arc`, charged). The table's
  fixture and two internal ones validate
  (`rejections::arcs_away_from_an_in_band_{external,internal}_tangency_are_no_contact`).
  Branch `nurbs/seg-arc-apart`.

**The remainder.** Two arcs on carriers in band of one circle
(`carrier_circles_identity` in band, not decided cocircular) escalate
whatever their spans. Each arc stands within about ε of the other's
circle everywhere, so no radial margin separates them, and the
certificate is not consulted there. Separating them needs a reading of
the two spans along the shared circle (an angular gap, the circle's
analogue of `segment_gap`), with a charge for the two carriers'
disagreement.

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to PROFILE (`work/profile/`) when PATHS closed. PATHS's live rows went to four programs opened at this close and cut along the priority seam; PROFILE holds the goal (`docs/doc-ledger/paths-leaves-the-tracker.md`). The item's id, content and history are unchanged.

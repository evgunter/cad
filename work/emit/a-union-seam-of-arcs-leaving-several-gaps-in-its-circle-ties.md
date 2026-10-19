---
id: a-union-seam-of-arcs-leaving-several-gaps-in-its-circle-ties
kind: issue
title: A union seam whose arcs leave several gaps in their circle ties: no rule picks which gap the seam starts after
status: closed
opened: 2026-10-19
closed: 2026-10-19
priority: P3
cost: E
design: true
---

## What

N2 ranks vertices with one run, where the run holds no edge, along "a
seam in it whose pieces lie on one open carrier".
`emit_union::rank_along_seam` reads that through `Track::open_offsets`
(`crates/editor-core/src/names/emit_topo.rs`): on a periodic carrier
(circle, ellipse) the seam ranks only where its pieces leave exactly
one gap in the period, and reads each vertex's offset from the end of
that gap. Pieces that cover the whole period tie (the approved closed
carrier case), and so do pieces that leave two or more gaps.

The several-gap case is the common one. A wall crossing an arc seam
twice swallows the stretch between the crossings, so the arc's pieces
leave two gaps: the wall's, and the one between the arc's own ends.
Both are geometry, and nothing in the pieces says which one the seam
"starts after"; reading from one or the other reverses the ranks. The
carrier's own parameter origin would pick one, but that is where the
kernel cut the period, which N2 refuses to read.

Witness: `tests/emit_union_seam_rank.rs`,
`crossings_of_seams_with_no_start_tie_in_every_member_order`. The run of
three seams there has an arc of the unit circle cut into two pieces
(two gaps), an arc of the wall's circle cut into two by `y`'s side (two
gaps), and two vertical lines (two carriers); the two crossings tie in
all six member orders. In the sibling test, which ranks, the same
unit-circle seam still leaves two gaps and is skipped; the group ranks
along the wall's arc, which is one piece between the two crossings.

## The question

Should a seam whose arcs leave several gaps rank, and from where?

- **Tie** (what is built). No new rule; a group ranks only along a seam
  with one start.
- **The gap the group's own vertices do not bound.** The seam is read
  as the arc it was before the cuts that made the group's vertices. It
  holds when exactly one gap is bounded by no vertex of the group;
  otherwise it ties.
- **The longest gap.** Purely geometric, but a near-equal pair of gaps
  is decided by the band and may escalate, and the rule has no reading
  in terms of the seam's own history.

## Closed (branch `emit/seam-rank-open-carrier`, PR 5347)

Settled by Ev's approval on PR 5107 (2026-10-14, "makes sense; that
can be a separate pr"): the approved recommendation ranks along "one
open curve (an arc)" and names "a wall crossing an arc seam twice" as
the fixture, which is this case. The second option above is that
approval's reading, and is built: a periodic seam whose pieces leave
several gaps reads from the end of the one gap no vertex of the group
bounds, where the arc ran before the cuts that made the group's
vertices. One such gap ranks; none, or two or more, tie. A gap is
bounded by a vertex when the end of a piece beside it is that vertex,
by key, never by place. One gap in all still reads from its end.

- `emit_topo::Track::open_offsets` / `seam_start`; the tie carries its
  reason, `NoStart` (`OffCarrier`, `WholePeriod`, `EveryGapBounded`,
  `SeveralFreeGaps`).
- `tests/emit_union_seam_rank.rs`,
  `crossings_of_an_arc_cut_twice_rank_from_the_gap_they_leave_free_in_every_member_order`:
  the only seam of the run that reads is `w`'s arc, cut by `x` between
  the crossings and by `y`'s side elsewhere; `x`'s arc leaves two gaps
  free of the crossings and ties, and the walls meet on two lines. The
  crossings rank alike in all six member orders. Reading from the gap
  between the crossings instead swaps the two ranks, and the test goes
  red; so does reading from the reading piece's start.
- Same file, `crossings_of_seams_with_no_start_tie_in_every_member_order`:
  with `w`'s wall cut where `y`'s side falls inside its half, its arc
  too leaves two free gaps, and the crossings tie in every order.
- `emit_topo::track_cover` unit tests: the two-gap arc reads alike from
  either piece, in either order, either way round and across the cut;
  each tie asserts its reason.

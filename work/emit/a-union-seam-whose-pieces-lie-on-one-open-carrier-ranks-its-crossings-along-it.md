---
id: a-union-seam-whose-pieces-lie-on-one-open-carrier-ranks-its-crossings-along-it
kind: issue
title: Crossings of a union seam whose pieces lie on one open carrier rank along that carrier's parameter; a closed carrier ties
status: closed
opened: 2026-10-15
closed: 2026-10-19
priority: P3
cost: M
---


## What

N2 ranks crossings of one line by one face, and vertices with one run,
"along a line they share … or, where the run holds no edge, a straight
seam in it". `emit_union::rank_along_seam` reads only a seam whose pieces
all lie on one straight line. That limit dates from before the names code
had a curved carrier reading (`Track`, PR 5073 and PR 5107); it was not a
decision against curved seams.

Ev, on PR 5107 (2026-10-14, answering why the clause is straight-only):
"makes sense; that can be a separate pr". That approved the recommendation
to rank along a seam whose pieces lie on one **open** carrier, by that
carrier's own parameter, straight or curved, and to keep a **closed**
carrier tied, since it has no start but where the kernel put the seam's
vertex and N2 refuses to read a name off that.

## Owed

- `rank_along_seam` reads a seam whose pieces lie on one open carrier,
  through `EdgeCarrier` / `Track`. A closed carrier and pieces on
  different carriers tie.
- N2's clause re-worded to "a seam in it whose pieces lie on one open
  carrier" (the wording Ev approved on PR 5107).
- A fixture: a wall crossing an arc seam twice, in every member order.
- `crossings-of-a-union-seam-of-several-curves-tie` (closed on PR 5107)
  stays closed: pieces on several curves still tie.

## Closed (branch `emit/seam-rank-open-carrier`)

Built. `emit_union::rank_along_seam` ranks along the first seam of the
run whose pieces lie on one open carrier through every vertex of the
group. A straight seam is read as before (`rank_crossings` along its
first piece). A curved seam whose pieces are all curved and readable
goes through `Track::open_offsets` (`emit_topo.rs`): each piece's
span along the first piece's carrier, and each vertex's offset in the
carrier's direction. On an open carrier the offset is from that
piece's start. On a periodic one it is from the end of the one gap the
pieces leave, or, where they leave several, of the one gap no vertex
of the group bounds (closed with
`a-union-seam-of-arcs-leaving-several-gaps-in-its-circle-ties`).
`rank_along_track` then ranks the offsets through the tail
`rank_crossings` uses (`rank_reads`), oriented as
`crossed_edge_orientation` orients the seam. These tie, each with its
reason (`NoStart`): pieces that cover the whole period (a closed
carrier), pieces or vertices off the carrier (several carriers), and
several gaps of which none, or more than one, is free of the group's
vertices; so do a spline first piece (`EdgeCarrier::Unread`) and a
seam mixing straight and curved pieces. The seam ranked along is the
first of the run, in its name order, that reads, straight or curved.

Every verdict along a curved carrier (`Track::sign`: the flush cover,
`holds`, `runs_with` and the seam's gaps) meters a parameter stretch by
the carrier's greatest speed (`geom::carrier_speed::speed_range`), not
by the reading edge's speed at its middle, so which piece reads cannot
change a verdict on an ellipse.

N2's clause in `names/README.md` now reads "a seam in it whose pieces
lie on one open carrier". `RoleSeg::EdgeCrossing`'s doc and
`ON_SEAM_LINE`'s doc say the same.

Evidence:

- `tests/emit_union_seam_rank.rs`: three scenes, each in all six
  member orders. One open arc ranks; an arc cut twice ranks from the
  gap its crossings leave free, the only seam of its run that reads;
  and a run whose arcs each leave two free gaps ties. The ranking tests
  pin which crossing takes rank 0; reading from the wrong gap, or from
  the reading piece's start, turns them red.
- `emit_topo::track_cover` unit tests: one gap reads from its start,
  across the parameter cut and from either piece; two gaps read from
  the free one; each tie asserts its reason; and on the ellipse of
  semi-axes 3 and 0.5 with one gap at the fast end, all three pieces
  read alike at every gap width from 1e-10 to 1e-7 (with the old
  per-reader speed, at 5e-10 two escalate and one ties).
- Measured, with a probe counting every call of `rank_along_seam` by
  outcome over the whole editor-core suite, slow set included (3001
  tests): only the three seam-rank fixtures reach it, 18 groups: 12
  ranked along a curve (one of them a two-gap arc read from its free
  gap in 6), 6 tied, every curved seam that tied for
  `SeveralFreeGaps`. No other group reached it, no curved seam that
  tied before now refuses, and no stored name moved.

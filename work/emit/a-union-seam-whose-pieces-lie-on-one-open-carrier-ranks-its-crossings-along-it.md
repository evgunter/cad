---
id: a-union-seam-whose-pieces-lie-on-one-open-carrier-ranks-its-crossings-along-it
kind: issue
title: Crossings of a union seam whose pieces lie on one open carrier rank along that carrier's parameter; a closed carrier ties
status: closed
opened: 2026-10-15
closed: 2026-10-14
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
stretch along the first piece's carrier, and each vertex's offset in
the carrier's direction. On an open carrier the offset is from that
piece's start. On a periodic one it is from the end of the one gap the
pieces leave. `rank_along_track` then ranks the offsets, oriented as
`crossed_edge_orientation` orients the seam, as the straight case is.
These tie: pieces that cover the whole period (a closed carrier),
pieces or vertices off the carrier (several carriers), a spline
carrier (`EdgeCarrier::Unread`), a seam mixing straight and curved
pieces, and pieces that leave two or more gaps. The last case has no
settled start, and is filed as
`a-union-seam-of-arcs-leaving-several-gaps-in-its-circle-ties`
(`design: true`).

N2's clause in `names/README.md` now reads "a seam in it whose pieces
lie on one open carrier". `RoleSeg::EdgeCrossing`'s doc and
`ON_SEAM_LINE`'s doc say the same.

Evidence:

- `tests/emit_union_seam_rank.rs`,
  `crossings_of_one_open_arc_rank_along_it_in_every_member_order`: a
  disc's wall crosses the arc where a second disc's wall leaves a
  block's top, twice. All six member orders publish one table, and the
  two crossings share one run name with ranks 0 and 1 of 2. With
  `rank_along_seam` as it was on main, the test fails on order [0, 1, 2]
  with the two crossings in one tied row.
- Same file, `crossings_of_seams_with_no_start_tie_in_every_member_order`:
  with the block cut back so every seam of the run leaves two gaps or
  lies on two lines, the crossings tie in all six orders.
- `emit_topo::track_cover` unit tests: one gap reads from its start,
  across the parameter cut and from either piece; a whole circle, one
  closed piece, and two gaps give no offsets; a piece on another circle,
  or a point off the circle, gives none.
- Measured, with a probe counting every call of `rank_along_seam` by
  outcome: the whole default editor-core suite (2998 tests) and the
  suites `name_words_corpus`, `emit_union_member_order`,
  `emit_union_rim_piece_ranks`, `emit_union_curved_flush`,
  `union_pinch_member_order` and `wire_legal_union_refusals` with the
  slow set included (51 tests) reach it only from the two new fixtures:
  6 groups ranked along a curve, 6 tied. No other group reached it, so
  no stored name moved.

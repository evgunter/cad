---
id: a-union-seam-of-arcs-leaving-several-gaps-in-its-circle-ties
kind: issue
title: A union seam whose arcs leave several gaps in their circle ties: no rule picks which gap the seam starts after
status: open
opened: 2026-10-19
design: true
---

priority: P3
cost: S

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

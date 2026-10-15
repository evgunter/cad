---
id: crossings-of-a-union-seam-of-several-curves-tie
kind: issue
title: A union's crossings of one seam that is several curves tie: rank_along_seam ranks only a seam whose pieces lie on one line
status: closed
opened: 2026-10-01
priority: P3
cost: M
closed: 2026-10-14
branch: emit/union-curves
refs: [edge-pieces-are-named-by-their-ends]
---

## What

A union's vertex cites a union seam by its head (N2), so where one face
crosses a seam several times the crossings share a name and N2 ranks
them along the crossed edge. `emit_union::rank_along_seam`
(`crates/editor-core/src/names/emit_union.rs:893`) ranks them only where
the seam's pieces lie on one straight line; a seam of several curves
(the two lines a cylinder lying across a plate makes, a circle cut in
pieces) has no one carrier, and the crossings tie (line 952).

## The question

What "the crossed edge" is for a union seam that is several curves: the
piece each crossing ends, which a crossing is the end of two of; or a
tie, which this records. No fixture reaches it; the area-overlap
fixtures of `wire_legal_union_refusals` reach the one-line case and
rank.

## Closed (branch `emit/union-curves`, 2026-10-14)

The names README settles it, and the code already does what it says.
N2: crossings of one line by one face with the same sense "are ranked
along a line they share by its carrier's own parameter, in its stored
orientation: the run's least edge, or where the run holds no edge, a
straight seam in it. An equal pair, or a run with no such line, ties."
A seam of several curves is no line its crossings share, so they tie.
The README states no reading of "the piece each crossing ends". Such a
reading would be a new rule, and every crossing ends two pieces.

`emit_union::rank_along_seam` follows it. It ranks along the first
seam whose pieces lie on one straight line through every vertex of
the group. Otherwise it mints the group as one tied row
(`mint_candidates`). A circle cut in pieces is one carrier but no
straight seam, and it ties by the same clause.

Nothing built. No fixture reaches a same-sense pair on such a seam. A
plane crossing a closed seam enters and leaves in turn, and the two
lines a cylinder lying across a plate makes are run opposite ways by
the plate's loops, so one wall crosses them with opposite senses. This
is reasoned from the construction, not measured.


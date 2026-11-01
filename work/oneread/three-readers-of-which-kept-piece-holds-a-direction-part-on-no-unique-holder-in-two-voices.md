---
id: three-readers-of-which-kept-piece-holds-a-direction-part-on-no-unique-holder-in-two-voices
kind: issue
title: Three readers of which kept piece holds a direction at a point (piece_holding, sector_piece, piece_beside) part on no unique holder in two voices
status: open
opened: 2026-11-01
priority: P3
cost: M
refs: [a-fate-cut-end-on-an-edge-of-its-faces-division-lies-in-no-piece, a-fate-cut-placement-limit-refuses-in-the-voice-of-a-kernel-bug]
---


Found by PR 6237's review (Q1, N3, N4), filed by its fix pass. Not
refactored there.

## What

"Which piece of a divided face holds direction `d` at point `p`" has
three readers in `crates/topo/src/boolean/fate.rs`, all through
`corner_holds`:

- `piece_holding` (~1178): at a vertex of the kept face's division, or
  a pierce's ring vertex, through `place`;
- `sector_piece` (~1662): at the sector's own vertex, through `place`;
- `piece_beside` (~1235): inside an edge of the division, each piece's
  half-edge read as a straight corner.

They part on what no unique holder means. `d` decided along a kept edge
refuses `ClassificationInvariant`, the voice of a kernel bug, at a
vertex (`piece_holding`'s "a dropped sector held at a vertex of its
kept face's division runs along a kept edge", ~1215; `sector_piece`'s
"a sector's bisector runs along an edge of its divided face", ~1699),
and the placement limit `Limit::BesideNoPiece` inside an edge. One of
the two voices is wrong for the same geometry.

Two unexercised corners of `piece_beside` / `lies`, each at worst a
spurious refusal, never a wrong piece:

- **N3:** one piece bordering `p` through two coincident edges of its
  own loops (a slit, or a ring touching the outline there): `contfp`
  names one edge, so `lies` records only that one and `piece_beside`
  reads only its half-edge(s).
- **N4:** at a point some pieces read `OnVertex` and others `OnEdge`,
  `lies` returns `AtCorners` and drops the on-edge pieces, so
  `piece_holding` reads only the pieces with a vertex there.

## Owed

One reading of "the piece holding `d` at `p`" for the three sites,
whatever `p` is on (vertex, edge, interior), with one answer for "no
unique holder": a `Limit` where the reading has no rule, a broken
invariant only where the state cannot arise. Pin N3 and N4 with a
witness each, or show they cannot arise.

## Slate triage

2026-11-01: moved from `work/cleave/` by `git mv` when CLEAVE handed its track back. It is ONEREAD's story: one question ("which kept piece holds `d` at `p`") answered by three readers. Id and body unchanged. (CLEAVE orchestrator)

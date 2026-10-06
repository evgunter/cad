---
id: an-edge-edge-crossing-vertex-of-a-union-is-spelled-by-member-order
kind: issue
title: A union vertex where a member edge crosses another member's edge is spelled by member order (Flush::crossing falls through to the fold's spelling)
status: open
opened: 2026-10-01
priority: P2
cost: M
refs: [a-member-the-fold-discards-whole-is-cited-nowhere-though-it-lies-flush]
---


## What

Found by a designer lane while weighing the covered-member fork (fork-log row 37), on `origin/main` f0807b14. Neither answer to that fork fixes it.

**Setup.** `a` = x 0..2 and `b` = x 0.5..1.5, both over y 0..1. `a` spans z 0..1 and `b` spans z 0..2, so `b` sticks out of `a`'s top. The pair is declared flush on the y-walls and the bottom cap.

**Spellings.** At the vertex where `b`'s lateral edge (x 1.5, y 0) crosses `a`'s top rim edge, the spelling depends on member order:
- order `[a, b]`: `Seam { b's LateralEdge, a's RimEdge(End) }`
- order `[b, a]`: `Seam { a's Cap(End), b's LateralEdge }`

**Mechanism.** At this vertex the finished edges lie within two member-edge lines, an edge–edge crossing. There `emit_union::Flush::crossing` returns `None`, so the fold's per-step spelling is published, and that spelling is fold history. N2's union paragraph says the parent and its seam partners depend only on the finished body.

**Pinned.** `crates/editor-core/tests/emit_union_rim_piece_ranks.rs` holds the case as `r5poke` (`b` operand A in the pairwise judgement) and `r5pokehi` (`a` operand A). `KNOWN_ABSENT` pins 4 absences in each: the crossing vertex on `a`'s `RimEdge(End)` and the rim piece it ends (`Fragment(Ends)` citing the vertex), each spelled one way per order.

## Next

1. Give `Flush` an order-free spelling for an edge–edge crossing, for example the two member edges' `Ends`-style pair sorted in name order. Check that against N2's vertex rules first: if N2 does not decide it, it is a design question.

## After PR 4134 (2026-10-06)

Ev's ruling on PR 4134 names every crossing by its sense. Only crossings
of one edge by one face with the same sense are still ranked along the
edge (`OrderAlong`), and a vertex where two edges cross carries each
edge's sense against the other operand's closed body. Once the sense is
built (`a-second-crossing-by-one-face-renames-the-first-and-its-pieces`,
branch `emit/crossing-sense`), re-measure this row against the narrower
case that is left.

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

**Not caught.** This case is not in `crates/editor-core/tests/emit_union_rim_piece_ranks.rs`'s corpus, so `KNOWN_ABSENT` does not see it.

## Next

1. Add the two-member poke-out case to the rim-piece corpus.
2. Give `Flush` an order-free spelling for an edge–edge crossing, for example the two member edges' `Ends`-style pair sorted in name order. Check that against N2's vertex rules first: if N2 does not decide it, it is a design question.

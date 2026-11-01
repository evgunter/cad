---
id: a-split-touching-a-wall-at-a-vertex-refuses-a-crossing-vertex-emission
kind: issue
title: A split through a moat that touches a pocket or block wall at a vertex refuses Emission("a split's crossing vertex does not hold exactly one piece of the edge it crosses")
status: closed
opened: 2026-10-20
priority: P2
cost: M
closed: 2026-10-21
---


Found by the re-review of PR 5486 (2026-10-20). It refuses the same way on
the PR head and on its merge base, so it is not a regression of that PR.
Two documents reproduce it, both variants of the moat witness
`an_island_in_a_ring_the_merge_made_cites_the_moat` in
`crates/editor-core/tests/split_pinch_keeps.rs`:

- a diamond moat that touches the pocket wall at its vertices;
- a moat that reaches the block wall at a vertex.

Each split refuses `Emission("a split's crossing vertex does not hold
exactly one piece of the edge it crosses")` from
`crates/editor-core/src/names/emit_topo.rs`. Both are legal bodies, so
under D9 (PR 5411) real input reaching this refusal is a defect: either
the naming rule mis-reads a vertex that a wall and a moat share, or the
kernel records the crossing wrongly. The fix also unblocks the vertex-touching
probe for `plane_walls`' closure join (the third `unreachable!` there),
which no test can reach until this split evaluates. It overlaps
`names-emission-raises-panic-or-take-their-true-ending` for this one raise.

Closed by PR 5515 (2026-10-21). Root cause: the vertex pass read a pair original's `SplitEdge` birth record, which an earlier subtract wrote, as this split's crossing. It now checks the table first: a table-named vertex is `OnToolVertex` (N2). Both documents are pinned, the second recovered from PR 5486's probe, along with a kite variant. The remaining `split_sense` refusal sees only vertices the split minted. It is probably unreachable (`Joined::Whole/Set` cover whole operand edges; `Seam` lies along none), but that is not proven, so it stays typed.

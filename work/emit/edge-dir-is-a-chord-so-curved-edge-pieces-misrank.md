---
id: edge-dir-is-a-chord-so-curved-edge-pieces-misrank
kind: issue
title: emit_topo::edge_dir is a chord, so pieces of an arc over pi misorder and pieces of a closed edge always tie
status: open
opened: 2026-09-30
priority: P1
cost: M
refs: [edge-pieces-are-named-by-their-ends]
---


## What

`emit_topo::edge_dir` (`crates/editor-core/src/names/emit_topo.rs:1831`)
is the chord `end − start` of an edge. The rankers that order the pieces
of an operand edge (`FromA(e)`, `FromMember(m, e)`) and repeated
crossing vertices project onto it. On a curved parent that is not a
direction along the edge:
- on an arc over π the projection is not monotone along the arc, so
  pieces can misorder or tie;
- on a closed edge (a full circle) the chord is the zero vector, so
  every piece ties.

Found by a designer lane on the curved-seam fork (fork-log row 22), not
yet measured with a fixture.

## Relation

If Ev takes the designer recommendation that retires `OrderAlong` for
all edge pieces (`curved-seam-pieces-have-no-ranking-direction`), this
row closes with it. Otherwise it needs its own rule.

## Ruled (2026-10-01, PR 3553)

Ev took the recommendation of fork-log row 22 ("the recommendation
sounds good, including the change to what was decided in 512!"). The
rule is `crates/editor-core/src/names/README.md` N2. Edge pieces no
longer project onto `edge_dir`: they take `Ends`. The one ordinal left,
on crossing vertices, is by the crossed edge's own curve parameter, not
by projection onto a chord. `edge-pieces-are-named-by-their-ends`
implements it and closes this row.

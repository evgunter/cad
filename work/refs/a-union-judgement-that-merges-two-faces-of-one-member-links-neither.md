---
id: a-union-judgement-that-merges-two-faces-of-one-member-links-neither
kind: issue
title: A union whose judgement merges two faces of one member refuses as an emission bug
status: open
opened: 2026-10-17
priority: P1
cost: M
---

## What

A union's end pass (`emit_union`'s `Parents::of`) links member faces
by the pairs its pairwise judgements recorded (`Links::judged`). For
each merged face of a judgement, `judged` pushes a pair for every
member-`a` face and member-`b` face the merge holds, and nothing for
two faces of the SAME member it holds. When a judgement's merge glues
two faces of one member into one face, those two member faces stay
two parents, the finished face descends from both, and `Parents::of`
refuses `NamingError::Emission { what: "a union's face descends from
member faces no judged pair links" }`: a kernel-bug framing on a legal
recipe.

`crates/editor-core/src/names/README.md`, "A union's face is named for
its PARENT", says two member faces are linked "when the pairwise
judgement of their two members (DM4) merged or covered them"; it has
no case for two faces of one member, so what such a pair is (one
parent, as the pair boolean names it, or something else) is the
question this row owns.

## Witness

`wire_legal_union_refusals::a_bar_through_a_sunk_boss_refuses_at_the_unions_parents`
pins it (wire, 2026-10-17, branch `wire/merged-chord-rule`):
- plate `[0,3]² × [0,1]`; boss of radius 0.3 about (1.5, 1.5), z 0.5
  to 1.5, its wall the two semicircular pieces of
  `circle_split(.., 2, ..)`; the two joined by a `Union`;
- a bar x 1.4 to 1.6, y 0.5 to 2.5, z 0.8 to 2.0, joined to that
  union by a second two-member `Union`.

The bar cuts each boss piece in two, and the second union's merge
glues the two cosurface pieces on each side of the bar: each merged
wall face holds both pieces, both faces of the first member. Before
the chord rule
(`work/wire/a-merged-face-with-several-same-side-constituents-has-no-chord-rule.md`)
the fold step's naming refused `MergedChordConstituents { several: 2 }`
first, which masked this arm; with it, the fold step names its seam
chords and the end pass refuses here. The subtract of the same slab
names (`emit_union_borders::a_slot_across_a_sunk_boss_divides_its_merged_wall_by_the_slot_walls`):
a subtract is named by the pair emitter alone (`eval/wire.rs`'s
`wire_subtract`), and only a union or intersect reaches `name_union`.

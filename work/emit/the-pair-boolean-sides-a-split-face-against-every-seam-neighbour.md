---
id: the-pair-boolean-sides-a-split-face-against-every-seam-neighbour
kind: issue
title: The pair boolean sides a split face's pieces against every seam neighbour, so a boss on one piece is a SideOf partner and a curved one refuses as an emission bug
status: open
opened: 2026-09-26
priority: P2
---


## What

When a pair boolean splits a face, `emit_topo::name_fragment_group`
(emit_topo.rs:846) takes every operand face across any piece's seam edges as
a `SideOf` partner (emit_topo.rs:883). It then reads each partner's plane with
`face_plane` (emit_topo.rs:896). So two things go wrong:
- A face of the other operand that only borders one piece is a partner. For
  example, a boss standing on the piece, or a notch in it. That makes the
  pieces' names depend on features that divide nothing.
- When that bordering face is curved, `face_plane` refuses with
  `NamingError::Emission { "face_plane: non-planar carrier in planar
  pipeline" }`. That is a bug-framed refusal of a legal recipe.

PR 3241 fixed the same defect in the union's own pass (`emit_union::dividing`
takes only the seams of features that divide the parent, sets a curved
partner aside where the planar ones tell every face apart, and refuses
`NamingError::SplitReference` where they do not). The pair boolean's per-step
rule is unchanged.

## Evidence

`emit_union_dividing::a_curved_neighbour_that_divides_nothing_is_no_partner`
(crates/editor-core/tests/emit_union_dividing.rs) uses a plate
`[0,3]² × [0,1]`, a slab at x 0.5..0.7 through its top, and a cylinder boss
`circle_split(2.0, 1.5, 0.3)` standing on the top away from the slab. The
union publishes in four member orders. The two orders that fold the plate last
(`[slab, boss, plate]`, `[boss, slab, plate]`) split the top in one pair step
whose other operand holds both the slab and the boss. That step sides against
the boss's curved wall and refuses with the `Emission` above. The row pins
those two refusals.

## Fix direction

Give `name_fragment_group` the union's rule: partners from the seams that
divide the group, and a curved partner set aside or refused typed. Then the
two orders publish, and the row's pinned refusals go.

---
id: a-union-glues-same-sense-cosurface-walls-without-merging-them
kind: issue
title: A union glues same-sense cosurface walls into one body without merging them, so its output fails the maximal-faces precondition as an operand
status: open
opened: 2026-10-01
priority: P0
cost: M
refs: [cosurface-disjoint-curved-walls-refuse]
---


Found by a designer lane weighing REACH's
`cosurface-disjoint-curved-walls-refuse` (2026-10-01), using a
throwaway probe. Not yet pinned by a test.

## What

Two 6×4×1 plates stacked at `z ∈ [0, 1]` and `[1, 2]` with a sharp
outline, the mating plane declared `Rest` and the four wall pairs
undeclared, union to volume 48 with **10 faces**. Each of the four
same-sense cosurface wall pairs comes back as two coplanar adjacent
faces, not one. Handed to a second union with a disjoint plate, that
result is refused by the maximal-faces gate (`gate_maximal_faces`, its
planar arm, `crates/topo/src/boolean/reduce.rs`):
`UndeclaredCoincidence { pair: [(A, 9v1), (A, 3v1)], SameOriented }`.
With the walls also declared, the same stack gives 6 faces, and the
result is accepted as an operand.

So a union's output is not always a legal boolean operand, which
DESIGN requires of every boolean output. The gate checks only planar
pairs, so an unmerged CURVED cosurface adjacency would ship with no
refusal at all. The shipped two-peg mated body
(`demos/tour/src/twopeg.rs`, which declares only the mating plane and
the cylinders) very likely carries the same defect. That has not been
confirmed.

## Coupling

What licenses merging a same-sense pair (a declaration, or something
the kernel derives) is the open question on
`cosurface-disjoint-curved-walls-refuse`. Its answer decides whether
this output should merge or refuse. Either way, the op should not hand
back a body that its own next op rejects.

## The planar half at the chord join (JOIN-1 fix pass 2, PR 3790)

Where the same-sense pair meets along an edge both operands hold (an
edge-edge site of the vertex-vertex classification), a union now
refuses an undeclared pair at the op: `UndeclaredCoincidence {
relation: SameOriented }` naming the two faces (`recl.rs`
`resolve_edge_edge`, the touching arm). Declared, the output's merge
stage glues it. Pinned on the flush peg in the collar
(`crates/sweep/tests/join1_mechanisms.rs`
`an_undeclared_continuation_refuses_at_the_op`; the delta review's
`join1_delta_probes::the_peg_collar_unions_are_operands`, now not
ignored, builds legal operands with the caps declared) and on the
stacked plates of this row (`a_stacked_plates_union_declares_its_walls`).
The curved half — an undeclared CURVED cosurface adjacency — is not
covered: the check reads the planar carrier ladder only.


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

## The declared-REST zip leaves flush end faces unmerged (TANG, 2026-10-02)

Measured on branch `tang/abutting-rim` (PR 3823), whose diff does not
touch the REST lane's merge. A 4×4×1 block with a 1×1 square bore,
unioned with the 1×1×1 peg that fills it, the four wall pairs declared
`Rest`: the body is `(8, 20, 16)` faces, edges, vertices, one shell,
where the box it is has `(6, 12, 8)`. At each end the peg's square stays
apart from the block's annulus: one plane, same sense, one face per
operand, undeclared. The REST lane merges through
`merge_coplanar_faces_declared` with the declared surface pairs only
(`rest.rs`, `try_rest_union`), so an undeclared same-sense coplanar pair
across the operands is never offered.

The torus peg seated in its socket (`crates/sweep/tests/mate7a_torus_rest.rs`,
`a_declared_torus_rest_pair_passes_the_declaration_door`) builds the
same way on that branch: `(6, 10, 8)`, V − E + F = 4, its end discs
inside the socket's end annuli. Main refuses that union
`Join(UnpairedLooseEnds)`, so there it is not reached. The row pins the
census and cites this file.

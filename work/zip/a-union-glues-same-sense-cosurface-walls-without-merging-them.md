---
id: a-union-glues-same-sense-cosurface-walls-without-merging-them
kind: issue
title: A union glues same-sense cosurface walls into one body without merging them, so its output fails the maximal-faces precondition as an operand
status: closed
opened: 2026-10-01
priority: P0
cost: M
refs: [cosurface-disjoint-curved-walls-refuse]
closed: 2026-10-01
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

## Closed (REACH, branch `reach/cosurface-continuation`)

Closed by the continuation ruling's implementation (PR 3613's design,
built on `reach/cosurface-continuation`). Measured on the row's own
sharp stack (`crates/sweep/tests/reach_continuation.rs`):

- Mating plane only: the union now refuses at the reduction,
  `UndeclaredCoincidence { relation: SameOriented }` naming a wall pair
  (an undeclared continuation), instead of shipping 10 faces.
- Mating plane plus the four wall continuations (what the flush
  detector now reports for aligned pairs): 6 faces, and the result
  unions again, with a clear plate and with a third stacked plate.

The twin the row suspected in `demos/tour/src/twopeg.rs` was real at
the peg ends as well as the walls: each peg's top, flush with Q's top,
shipped as a disc beside an annulus. Declared as continuations, the
discs now merge into the top face (`m9_3_zip`'s ring count went from 2
to 0). The curved half of the row's last paragraph (a gate arm for
curved cosurface adjacency) is filed as
`work/reach/maximal-faces-curved-arm-cannot-tell-a-licensed-curved-skip.md`.

---
id: a-split-through-a-union-pinch-corner-crosses-the-pinch-at-rest
kind: issue
title: A split through the corner where a union pinches leaves a half that tier 3 refuses PinchCornerCrossed
status: dispatched
priority: P1
cost: M
opened: 2026-10-20
branch: splitdoor/pinch-corner-split
---


Filed by WIRE's `wire-nameinv` lane (branch `wire/name-invariance-checks`)
while building `crates/editor-core/tests/name_invariance.rs`'s split corpus.

## What

`union_member_orders_decide_alike::prism_and_cube` is the L-prism and the
tilted cube pinching at the L's reflex corner (1, 1, 1). As a union, split by
the plane x = 1 (`Datum::Plane { origin: (1, 0, 0), normal: (1, 0, 0) }`),
which passes through the pinch corner and the L's reflex edge, it refuses:

    Split(Finish(ResultNotAtRest { side: Above, errors: [PinchCornerCrossed { face, vertex, edge }] }))

The half above the plane fails tier 3. Its section face's corner at the
pinch is crossed by an edge there. The union alone builds and names
(`union_member_orders_decide_alike::a_vertex_row_is_the_unions_once_in_every_member_order`).

The closed `a-split-through-a-vertex-whose-above-runs-nest-crosses-its-section-face`
fixed a split through a vertex whose above runs nest. This case is a split
through a pinch vertex: two cones meet there, each with its own runs.
Unmeasured: whether the two cones' runs are put on one section corner.

The case was left out of the name-invariance split corpus because it
publishes no table to compare.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P2 → P1: a split through a union's pinch corner leaves a half tier 3 refuses: verb breadth beyond the everyday shapes (a split or boolean that refuses, or ships a body its own gate fails, on a legal input), P1 by `work/README.md` Priority.
- Moved to SPLITDOOR (`work/splitdoor/`) by `git mv`, id and body unchanged: the P1 track opened for split and boolean shapes that refuse, or ship a side their own gate fails, where a uniform reading would build: the split door's tier-3′ bar, tangent and pinch poses, and the curved-carrier lanes that read lines and circles only. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

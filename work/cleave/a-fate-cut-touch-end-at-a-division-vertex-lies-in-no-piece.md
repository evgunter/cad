---
id: a-fate-cut-touch-end-at-a-division-vertex-lies-in-no-piece
kind: issue
title: A fate cut's touching end at a vertex of the dropped face's division lies inside no one piece of its face
status: review
opened: 2026-10-27
priority: P1
cost: M
refs: [a-fate-cut-end-between-two-touching-lumps-lies-in-no-piece]
branch: cleave/fate-touch-end-at-a-division-vertex
pr: 6185
---

Disclosed by PR 5812 (`cleave/flush-partners-at-a-slit-build`), the
blind spot of its sweep.

## What

PR 5812 places a fate corner's pierced end that lies at a vertex of
the kept face's division by its direction (`boolean/fate.rs`
`piece_holding`, read through `lies`). `cut`'s touch placement, an
edge's end where the kept vertex touches the dropped face with no
surgery (`End::Touch`, `fate.rs:1528`), still reads `fragment_at`
(`:1560`), which needs the point strictly inside one piece: at a vertex
of the dropped face's division it refuses `ClassificationInvariant` "a
fate cut's end lies inside no one piece of its face", a claimed kernel
defect, and it would mint a lone vertex where the face already holds
one.

No witness reaches it: the two-lump family of PR 5812 places its
touches inside one piece. A fate edge whose end is the lumps' shared
point, with the partner's face divided there by the other lump's
imprint, would.

## Owed

A witness; then place the end at the division's vertex (the chord
ends there, so no lone vertex is minted), or refuse typed.


## Built (branch cleave/fate-touch-end-at-a-division-vertex)

Witness: the cube `[0, 1]³` ∪ the box `[1, 2]² × [0.5, 1.5]`, touching
along `x = y = 1, z ∈ [0.5, 1]`, with the union's records carried; a
partner flush with the cube's top on `z = 1`, which the box crosses, so
the partner's face is divided by the box's imprint with a corner at the
cube's top corner `(1, 1, 1)`, where the cube's top edges (fate edges)
end touching it. On main `336465c32b`, `x − y` (partner above) and
`x ∩ y` (partner below or across) refuse
`ClassificationInvariant` "a fate cut's end lies inside no one piece of
its face" at the touch site (`lies` read `AtCorners`, two vertices),
14 cells per pose at every pose, at ε default, 1e-6 and 1e-12.

`cut`'s touch placement now reads `lies` itself: inside one piece it
mints the lone vertex as before; at vertices of the division the end
is those vertices, and `place` reads the chord's corner there by its
direction, as `piece_holding` does for a pierced corner; `fragment_at`
is gone. Every one of those cells builds, tier 3 and 3′, at the closed
form and with point sampling; no other cell of the probe's grid moves.
Pinned by
`a_keel_on_the_floor_subtracts_as_a_slit::a_fate_edge_touching_a_corner_of_its_faces_division_builds`
(red on main). The edge-interior sibling is filed as
`a-fate-cut-end-on-an-edge-of-its-faces-division-lies-in-no-piece`.

---
id: two-wedges-sharing-a-tangent-plane-face-on-a-ruling-refuse-unpaired-loose-ends
kind: issue
title: Two wedges sharing a wall's tangent-plane face, apex edges on its ruling, refuse Join(UnpairedLooseEnds) in every op and order: an untyped join refusal with no recourse (D4)
status: open
opened: 2026-10-16
priority: P2
cost: M
---


## What

Found by PR 5218's review (the curved half of
`a-vertex-minted-on-an-edge-lying-on-a-curved-face-is-unread`), on main
as well as on the PR's head.

**The review's witness.** On the cylinder r = 1, z ∈ [0, 2]: a piece,
the wedge prism with its apex edge on the wall's ruling at (1, 0),
opening over −90°..−50°; and a tool, the wedge prism with its apex edge
on the same ruling, opening over −150°..−90°, from z = 0.2, height 1.6.
The two share the wall's tangent-plane face x = 1, y < 0 (each has a
side along −90°). They refuse
`Join(BooleanJoin(UnpairedLooseEnds { count: 4 }))` in every op, in both
orders.

That refusal is untyped as to cause and carries no recourse, which D4
does not allow: a pair either builds, or refuses typed with what would
answer it.

**Not reproduced with PR 5218's fixture.** With the `wedge` and `rod`
fixtures of `crates/sweep/tests/a_minted_vertex_on_a_curved_face.rs`
(sides 0.5 long, piece z ∈ [0.5, 1.5]), every op builds in both orders:
for the piece alone, for the piece unioned onto the cylinder, and for
the cylinder alone, each against the tool. So the witness depends on a
detail of the review's fixture (side length, the piece's z range, or how
the line body is built). The first step is to commit the review's exact
fixture as a row.

## Owed

The witness as a committed row, then a typed answer: build, or a typed
refusal with a recourse.

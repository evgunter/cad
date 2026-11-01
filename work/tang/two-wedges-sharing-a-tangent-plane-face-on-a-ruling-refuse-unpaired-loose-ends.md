---
id: two-wedges-sharing-a-tangent-plane-face-on-a-ruling-refuse-unpaired-loose-ends
kind: issue
title: Two wedges sharing a wall's tangent-plane face, apex edges on its ruling, refuse Join(UnpairedLooseEnds) in every op and order: an untyped join refusal with no recourse (D4)
status: closed
opened: 2026-10-16
priority: P2
cost: M
closed: 2026-11-01
pr: 6253
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

**The fixture.** With `rod` and `wedge` of
`crates/sweep/tests/a_minted_vertex_on_a_curved_face.rs` (sides 0.5
long; `wedge(a, b, z0, h)` over `z ∈ [z0, z0 + h]`), the line body is
`rod ∪ wedge(−90, −50, 0.2, 1.6)`, cylinder first, and the tool is
`wedge(−150, −90, 0.2, 1.6)`. The piece's z range is the tool's, so their
caps coincide. That is what PR 5218's first attempt (piece over
`z ∈ [0.5, 1.5]`) missed, and why it built there. Reproduced on main
(`3bdf48c60`) at ε 1e-9, 1e-6 and 1e-12: every op, both orders, refuses
`Join(BooleanJoin(UnpairedLooseEnds { count: 4 }))`.

**Pinned** by
`two_wedges_sharing_the_tangent_plane_face_refuse_unpaired_loose_ends`
in that file, which asserts the exact refusal, so it flips when fixed.

## Owed

A typed answer: build, or a typed refusal with a recourse; the pinned
row is then re-baselined to it.

## Closed

2026-11-01, PR 6253 (`tangrim/sector-passes-descend`), by TANGRIM's
stage 2 of the one bound-side reading: every op in both orders builds,
tier 3 clean, at the closed form
(`a_minted_vertex_on_a_curved_face.rs`
`two_wedges_sharing_the_tangent_plane_face_answer_every_op`, which
replaces the refusal pin). The sector passes now read a bound tied with
the wall's tangent plane one order down against the cylinder itself,
where they read every face as its tangent plane and keyed the gateways
on the first-order `On`.

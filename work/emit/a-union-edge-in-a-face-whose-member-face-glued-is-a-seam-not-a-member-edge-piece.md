---
id: a-union-edge-in-a-face-whose-member-face-glued-is-a-seam-not-a-member-edge-piece
kind: issue
title: A union edge lying within a member edge whose own face was glued is named as a seam, not as a piece of that edge
status: closed
opened: 2026-10-13
closed: 2026-10-13
branch: emit/glued-rim-build
priority: P3
cost: M
---


## What

Found while building the in-face union vertex row of
`a-second-crossing-by-one-face-renames-the-first-and-its-pieces`.

Three blocks: `a` = (0, 2) × (0, 1) × (0, 1); `b` = (0.5, 1.5) ×
(0.25, 0.75) × (1, 1.5), standing on `a`'s top; `c` = (1.2, 2.5) ×
(−0.2, 1.2) × (0.5, 1.8), rising through `a`'s top across `b`'s far
end. `b`'s bottom face is glued to `a`'s top. So the union's edge along
`b`'s bottom rim at y = 0.25, from x = 0.5 to 1.2, lies between `a`'s top
and `b`'s y = 0.25 wall, and lies within `b`'s bottom rim.

The names README's flush bullet says "an edge that lies within one or
more member edges … is a piece of the first minted of them". The
emitter names this edge as a seam. `emit_union::Flush::edge_cover`
looks for member edges only among the rims between two faces of one
member that both descend to the finished edge's two faces. `a`'s top
descends from `a` alone, so `b`'s rim is no candidate.

Where `c`'s wall cuts that edge, at (1.2, 0.25, 1) and (1.2, 0.75, 1),
the vertex lies in `a`'s top face. It is named as the `EdgeCrossing` run
of three seams (a.top/b.wall, a.top/c.wall, b.wall/c.wall), none with a
sense. The name is the same in all 6 member orders (measured). Read as
`b`'s rim instead, it would be `Crossing { b's rim, c's wall, sense }`,
whose sense is read against `c`.

## The question

Is an edge whose member face was glued away a piece of that member edge,
as the flush bullet's "lies within" reads, or a seam, as its two faces
say and as the emitter does? N2's run rule and `edge_cover` both follow
from the answer. The bullet does not say whether "lies within" asks for
the member edge's faces.

## Ruling (Ev, PR 4965, fork-log row 131)

The edge is a seam, as built. The names README's flush bullet now defines "lies within": the edge lies on the member edge's segment and both of its faces descend from that member edge's two faces. An edge whose member face was glued away is the seam of the faces it separates. The vertex where `c` cuts it stays the run of its three seams.

There is no code change. What is owed is pinning the three-block fixture over every member order, with the `b`-sunk and `b`-lifted variants. Then close this row.

## Closed (branch `emit/glued-rim-build`, 2026-10-13)

Pinned as built, no code change:
`emit_union_member_order::an_edge_whose_member_face_was_glued_away_is_a_seam_in_every_member_order`.
- **Glued** (`b` at z = 1): all 6 member orders publish one table. The
  edge (0.5, 0.25, 1)–(1.2, 0.25, 1) is `Seam { a.top, b.wall(y = 0.25) }`.
  The vertex (1.2, 0.25, 1) is the `EdgeCrossing` run of the three seams
  a.top/b.wall, a.top/c.wall, b.wall/c.wall, none with a sense.
- **Sunk** (`b` at z = 0.9): one table over the 6 orders, and the edge and
  vertex carry the glued fixture's names. The names are compared with
  every mint id's digest erased, so node ids compare by ordinal. Each
  fixture is built by the same insertions, so ordinals agree, but a
  digest hashes the node's content, and `b`'s height differs.
- **Lifted** (`b` at z = 1.1): one table over the 6 orders. The edge at
  z = 1.1 is a piece of `b`'s bottom rim (`From { b }` with
  `Fragment(Ends)`), and the vertex is that rim's `Crossing` with `c`'s
  x = 1.2 wall. Neither glued name survives.

The test takes about 2.3 s, so it joins the `ci` slow set. That filter
named the neighbouring all-orders row under a retired name, which this
branch corrects.

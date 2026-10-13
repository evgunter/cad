---
id: a-union-edge-in-a-face-whose-member-face-glued-is-a-seam-not-a-member-edge-piece
kind: issue
title: A union edge lying within a member edge whose own face was glued is named as a seam, not as a piece of that edge
status: open
opened: 2026-10-13
design: true
priority: P3
cost: M
needs_ev: true
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

---
id: a-declared-merge-leaves-a-collinear-valence-two-vertex-an-earlier-cut-made
kind: issue
title: A declared coplanar merge leaves a collinear valence-2 vertex that an earlier fold step's cut made, so a union's finished body depends on member order
status: open
priority: P1
cost: D
opened: 2026-09-24
refs: [declared-flush-union-edge-and-vertex-names-follow-member-order]
---


## The finding

A `Node::Union` folds its members pairwise in list order (DM4). With a
declared flush pair, the finished body's VERTEX SET depends on that
order.

Found by EMIT while working
`work/emit/declared-flush-union-edge-and-vertex-names-follow-member-order.md`,
and measured on `emit/declared-flush-order` with a scratch probe that
prints each fused order's vertices at x = 0.5.

The document:
- `a` = x∈(0,1), `b` = x∈(0.5,1.5), both y,z∈(0,1);
- `a` and `b` declared flush on both caps and both y-walls;
- a slab `s` = x∈(0.49999,0.50001), y∈(−1,2), z∈(0.5,2.5).

What each order gives:

| order | vertices | edges | faces |
|---|---|---|---|
| `[a, b, s]`, `[b, a, s]` | 30 | 42 | 14 |
| `[b, s, a]`, `[s, b, a]` | 32 | 44 | 14 |
| `[a, s, b]`, `[s, a, b]` | refuse (`DeclareResolve`, other rows) | | |

The two extra vertices are (0.5, 0, 0.5) and (0.5, 1, 0.5). Each has
valence 2. Its two edges run along x at z = 0.5 on the slab's bottom,
to (0.49999, y, 0.5) and (0.50001, y, 0.5). They are collinear and
opposed, and both lie between the same two faces: the slab's bottom
cap and the merged y-wall.

## How the vertex is left

1. The step `b ∪ s` cuts `b`'s vertical edge at x = 0.5, y = 0 with
   the slab's bottom. That mints (0.5, 0, 0.5) at valence 3.
2. The step `(b ∪ s) ∪ a` merges `a`'s y = 0 wall with `b`'s, which is
   declared coplanar. That kills the edge between them, the stretch of
   `b`'s old vertical edge below z = 0.5
   (`Body::merge_coplanar_faces_declared`, `crates/topo/src/merge_faces.rs`).
3. The vertex drops to valence 2 and stays.

In the orders that fold `a` and `b` first, the slab cuts the already
merged wall and no such vertex is ever made.

The straight-seam repair (`kev`, gated by
`Body::redundant_subdivision_vertex`) removes a valence-2 collinear
vertex only on the seam the merge group is killing. It does not look at
the group's boundary vertices, which the `kef` leaves at valence 2.

**It is specific to the declaration.** The same slab with `a` alone,
or with an undeclared `b` = x∈(0.5,1.5), y∈(0.2,0.8), z∈(0.2,0.8), at
±1e-5 and at ±1e-7, gives one vertex set in every order.

## Why it matters

Naming cannot make a union's names independent of member order when
its entities are not. In the orders above:
- the seam edge along the slab's bottom is one edge in one pair of
  orders and two edges in the other, so its names differ;
- the two extra vertices are named in two orders and absent in two;
- the merged wall's name binds different vertex sets.

EMIT's union naming is order-free only as far as the finished body is
(`crates/editor-core/src/names/emit_union.rs`, module doc). This is the
body-level half.

## The design question in the fix

A valence-2 vertex between two collinear, opposed edges on the same
two faces is redundant in any order. The repair that
`redundant_subdivision_vertex`'s docs argue for is "collinearity, not
provenance". Applied to every vertex a merge leaves in that shape, it
also removes the flush corners that every order has today, such as
`b`'s corner (0.5, 0, 0) on `a`'s bottom rim.

Removing those changes what the union names. The flush rim would become
one edge x∈(0,1.5), which lies within neither member's edge. The
naming vocabulary has no name for an edge made of two members' edges.
Keeping the corners and removing only the vertices a cut made is
provenance-based, and the kernel's docs refuse that.

So this needs a ruling on the canonical form before it is a unit. The
owner of the choice is ZIP, with EMIT consulted on the naming
consequence.

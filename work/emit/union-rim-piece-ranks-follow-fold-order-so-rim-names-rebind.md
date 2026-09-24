---
id: union-rim-piece-ranks-follow-fold-order-so-rim-names-rebind
kind: issue
title: A declared union ranks a member's rim-edge pieces per fold step, so one FromMember rim OrderAlong name binds different pieces in different member orders
status: open
opened: 2026-09-24
priority: P0
cost: H
---


## What

A member's rim edge that a declared union cuts into pieces is named
`[FromMember(m, RimEdge(..)), Fragment(OrderAlong{rank, of}) ...]`,
with ranks written by whichever fold steps did the cutting
(`crates/editor-core/src/names/emit_topo.rs`, `name_boolean_edges`' descent
groups and `insert_ranked_or_tied`). Different member orders cut the rim
in different steps, so the SAME name ends up on DIFFERENT pieces. A
reference minted in one order silently lands on another piece after the
author reorders the members.

## Measured (EMIT, 2026-09-23; scratch probe over PR 3112's review corpus)

`fam010` (`a` = x∈(0,1), `b` = x∈(0.5,1.5), declared flush on all
four families; third member x∈(0.3,0.4), y∈(-1,0.5), z∈(0.5,3.5)).
Both orders fuse on main.

| order | `FromMember(a, RimEdge(End, seg 0))#OrderAlong{1 of 2}` binds |
|---|---|
| `[a, b, g]` | x = 0.5..1.0 at y = 0, z = 1 |
| `[b, a, g]` | x = 0.4..0.5 at y = 0, z = 1 |

- In `[a, b, g]` the first step splits the rim at x = 0.5 and ranks
  both pieces. `g` then splits one piece, whose own pieces get a
  stacked second rank.
- In `[b, a, g]` a different step ranks a different split.

Counted as names present in two fused orders of one document but bound
to different geometry. Edges were compared by their end points and
vertices by their point, over the corpus's 350 pairs of fused orders at
`emit/shared-rim-several`'s first head (EMIT's own probe). "Rebinds"
counts (pair, name) rebinds, so a name rebound in three pairs counts
three times:

| pairs of orders | pairs | pairs with a rebind | rebinds |
|---|---|---|---|
| both fused on main | 162 | 18 | 24 |
| one fused on main, one fuses with `emit/shared-rim-several` | 124 | 42 | 60 |
| both fuse only with `emit/shared-rim-several` | 64 | 42 | 107 |

Every rebind is a `FromMember` rim-edge piece. No vertex point
rebinds. The middle row is where the shared-rim rule trades a
refusal for a silent rebind: a reference minted in the order that
fused on main now resolves, to another piece, in the order that used
to refuse.

## Why P0

It is a live wrong answer: a name resolves, to the wrong entity,
with no refusal. `declared-flush-union-edge-and-vertex-names-follow-member-order`
(P1) records that these names differ across orders. It also says
"zero cases of a name rebinding"; that was measured on vertices only
and does not hold for rim edges. The seam-edge sibling is
`union-seam-edge-ranks-follow-which-step-split-the-seam` (P0). Both
mechanisms are the same: pieces ranked by whichever step cut them.

## The fix in review: PR #3168 (EMIT, 2026-09-24)

PR #3168 (`emit/rim-piece-ranks`, stacked on #3167) adds two passes over
the finished body at the end of `emit_union::name_union`:
- `rank_member_edges`: the vertices lying on member `m`'s edge `e` cut it
  into cells, numbered along `e` in `m`'s body. A piece publishes
  `FromMember(m, e)#k of n`, where `k` is its first cell and `n` counts
  every cell, including cells another member holds or no member does.
- `cite_member_edges`: a seam vertex that cites a ranked piece of a
  member edge cites the whole edge.

Its measurements, from #3168's review probes over the corpus and the
review fixtures, every order: 0 pairs of fused orders rebind a name,
and no cell that fuses on main refuses. On main (498de1ae6d, the review's
first probe): over the corpus plus the r1–r3 fixtures, 25 of 305 pairs
rebind, 56 (pair, name) rebinds of 20 distinct names; over the corpus
alone, 18 of 158 pairs, 24 rebinds of 8 names. (EMIT's probe above
fuses 162 corpus pairs on main to the review's 158; the 18 and 24
agree.) Ranking over the pieces a member keeps, an earlier draft,
still rebound `r2ends`, `r2endsg` and `r4tri`. The rows are in
`crates/editor-core/tests/emit_union_rim_piece_ranks.rs`.

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
vertices by their point, over the corpus's 350 pairs of fused orders:

| pairs of orders | pairs | pairs with a rebind | rebound names |
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

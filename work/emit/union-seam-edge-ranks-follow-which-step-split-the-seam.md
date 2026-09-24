---
id: union-seam-edge-ranks-follow-which-step-split-the-seam
kind: issue
title: A union names one seam edge's pieces by two different rankers depending on member order, so OrderAlong names rebind silently
status: dispatched
opened: 2026-09-23
priority: P0
cost: D
branch: emit/seam-rank-orientation
---


## What

A seam edge that a later member cuts gets its `Fragment(OrderAlong)`
rank from one of two rankers in `crates/editor-core/src/names/emit_topo.rs`,
and which one depends on member order:

- **The seam is minted already in pieces** (the cutter was folded in
  before the seam): `name_boolean_edges`'s seam-group loop ranks the
  pieces as a seam chain along `n_a × n_b`. `emit_union::collapse`
  re-orients that rank to the canonical pair (`RankRule::Reverse`).
- **The seam is minted whole and a later step cuts it**:
  `name_boolean_edges`'s descent groups rank the pieces as a sub-edge
  chain, `FromA(Seam{..})#OrderAlong`, along `edge_dir(op_body, root)`.
  That is the root edge's orientation in the accumulated operand body.

`collapse` flattens `FromA(Seam{..}#OrderAlong)` and `Seam{..}#OrderAlong`
to the same spelling. So one name can carry a rank measured along two
different directions, and nothing reconciles the two.

## Measured (PR #3121's reviewer; reproduced 2026-09-23)

Fixture `slab_rib_cutall`, a plain `Node::Union`:
- the slab [0,3]×[0,2]×[0,1];
- the inverted-U rib of `crates/editor-core/tests/emit_union_member_order.rs`
  (`slab_rib`);
- a cutter [0.7,2.3]×[0.3,1.7]×[0.95,1.05], which crosses every seam
  edge.

In orders `[slab, rib, cutter]` and `[slab, cutter, rib]`, the same name
`Seam{slab.Cap(End), cutter.Lateral(s1)}#OrderAlong{0 of 2}` binds the
edge at x = 2.3, y∈[0.3,0.5] in one order and y∈[1.5,1.7] in the other.
Rank 1 swaps the other way.

Rebinds, counted over pairs of the six member orders as names present in
both tables but bound to different geometry:

| state | rebinds per differing pair | pairs that differ |
|---|---|---|
| PR #3121 head | 4 | 3 of 15 |
| main before #3121 | up to 12 | 14 of 15 |

So the canonical-pair fix removes most of them, and these remain.

**Related, order-dependent refusal (m3).** A cutter crossing only the
first arm's seam edges, [0.7,0.8]×[0.2,0.8]×[0.8,1.2] (`slab_rib_cut`):
- `[slab, rib, cut]` refuses: "faces … share more than one edge", which
  is `NamingError::SharedRim` with `found: Several`;
- the other five orders fuse and name.

The reviewer measured a refusal in `[rib, slab, cut]` too, with their
own cutter's dimensions. This is evidence for
`shared-rim-several-is-a-missing-rule-legal-declared-unions-reach`
(PR #3120's branch, not on main when this was filed): an ORDER-dependent
instance of that row's missing rule.

## Why P0

It is a live silent rename. A reference to a seam edge's piece survives
a member reorder and lands on the other piece.

## Fix direction (to measure first)

Give both rankers ONE orientation rule, not a third correction in
`collapse`. For example, rank every piece of a seam line along the
canonical pair's `n(first) × n(second)` wherever the root is a seam
edge, or orient a descent chain by its root's canonical direction rather
than its arena orientation. This is the same class as
`name-ordered-positions-in-a-path-have-no-single-home`: a value in a
path that depends on an order the rewrite changes.

Found by PR #3121's review (M1), which corrected that PR's sweep. That
sweep had attributed every three-member difference to fold history.

---
id: name-ordered-positions-in-a-path-have-no-single-home
kind: issue
title: Name-ordered positions in a role path have no single home, and each rewrite re-establishes a different subset
status: open
opened: 2026-09-23
priority: P1
cost: D
---


## What

Several positions in a `RolePath` are in NAME ORDER — their canonical
order is the order of the names they hold, not the order they were
found in. Any rewrite that changes those names (a union's
member-keying collapse, a node-id remap) has to put them back in order,
or it publishes a name that is not the one the emitter would mint for
the same entity in the rewritten space. No single function knows which
positions those are. Each site keeps its own partial list:

- **`emit_topo` (the mint)** sorts and dedups the `Merged` constituents
  (`crates/editor-core/src/names/emit_topo.rs`, the merge-group loop),
  and sorts and dedups a seam junction's lines
  (`name_boolean_vertices`, the `seam_lines.len() >= 2` arm).
- **`emit_union::collapse`, the junction run** re-sorts the collapsed
  lines (`crates/editor-core/src/names/emit_union.rs`, `collapse`'s
  `Seam` run arm), and `seam_line` puts one line's two sides in name
  order.
- **`emit_union::collapse`, `Merged`** re-sorts and dedups the
  collapsed constituents (`collapse`'s `Merged` arm).
- **`emit_union::collapse`, `SideOf`: MISSING.** It collapses each
  partner name and keeps the fold-space order. That contradicts
  `role.rs`'s own rule for the qualifier — "one entry per partner,
  sorted by partner name" (`crates/editor-core/src/names/role.rs`,
  `Qualifier::SideOf`).
- **`refactor::remap_seg`** re-sorts `Merged` and `SideOf` after the
  id rewrite, because "the rewrite may have changed it"
  (`crates/editor-core/src/refactor.rs`, `remap_seg`'s `set` closure
  and its `Fragment(SideOf)` arm). It does NOT re-sort a junction's
  run, or re-order a union `Seam`'s two sides, both of which
  `emit_union` establishes in name order.
- **`FaceName::map_derivation`** (`role.rs`) hands the rewrite the
  whole path and keeps no order itself. Still to check: whether every
  caller's rewrite re-establishes the positions above.

## Witness

The review of `seam-junction-vertex-name-cannot-be-collapsed-by-the-union-fold`
(PR 3112) measured the `SideOf` gap on the `abys` document:
- `a` = x∈(0,1), `b` = x∈(0.5,1.5), both y,z∈(0,1), declared with
  `flush_pairs((a, a), (b, b))`;
- `y` = x∈(-1,2), y∈(0.3,0.4), z∈(0.5,3.5).

`[a, b, y]` and `[b, a, y]` both fuse. `y`'s start cap is split into
two fragments, and they publish as
`[FromMember(y, Cap(Start)), Fragment(SideOf([(a.Lateral(3), s), (b.Lateral(1), t)]))]`
in one order and with the two partners listed the other way round in
the other. The verdicts are the same and the order differs, so it is
the same face under two names. The same gap accounts for the face
rows in `declared-flush-union-edge-and-vertex-names-follow-member-order`.

The remap half is unmeasured: whether any caller's node map is
non-monotone, so that it reorders two names.

## A value that depends on a name order

This is a sibling position, found by `seam-chain-ranks-are-oriented-a-first-so-an-operand-swap-may-reverse-them`.

A seam EDGE's `Fragment(OrderAlong)` rank is measured along `n_a × n_b`,
the pair emitter's A-first line. Once `seam_line` puts the pair in name
order, the rank has to be read from the other end wherever that
reordering swapped the two sides. `emit_union::collapse` now does this
(`seam_line` reports the swap, and the tail loop applies
`of − 1 − rank`). So the swap is not only a reordering: it also changes
the VALUE of a later segment. The one canonicalizer this row proposes
has to own that rule as well.

## Why it matters

Re-sorting `SideOf` in `collapse` changes published names for unions
that fuse today, so it wants its own measured PR, and it is not folded
into 3112. The class fix is ONE canonicalizer: a function from a path
to its canonical form, which the mint, the collapse and the remap all
call. Adding a fourth partial list would not fix it.

## Found by

The segment-shape sweep for `seam-junction-vertex-name-cannot-be-collapsed-by-the-union-fold`,
widened by that PR's review.

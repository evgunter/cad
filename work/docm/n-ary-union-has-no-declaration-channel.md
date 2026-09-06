---
id: n-ary-union-has-no-declaration-channel
kind: issue
title: An n-ary union of members that touch refuses UndeclaredContact with no recourse: the union carries no declare edge
status: open
opened: 2026-09-04
refs: [DOCM-3]
---


## What

Found by DOCM-3's dual review (PR 1803), R2's executed probe
`r2_a_union_of_two_flush_placements_of_one_prototype` on
`docm/3-review-r2`: two flush placements of one prototype under
`Node::Union` refuse `UndeclaredContact`, as the pair boolean would,
but the pair boolean has a `declare` edge through which the detect/
declare protocol (SELECT-DESIGN §3d, register R3) lets the author
answer the refusal, and DM4 gave the union none — "a declaration is a
statement about ONE pair of operands, and a field here would have to
name a fold step, which is a position" (`node.rs`, the variant's doc).
So a flat union can fuse only members that do not touch; touching
members must fall back to the pairwise chain the union exists to
retire.

The refusal payload naming fold-space rows (both reviewers) is fixed
in DOCM-3's fix pass; this item is the channel. The shape to rule on:
a `declare: Option<RecipeNodeId>` on the union whose `Declare` pairs
name entities in MEMBER space (`FromMember { member, of }` names, which
the fold already presents to every step through `member_view`), each
pair resolved at the step where both its members are in the
accumulation; or a ruling that a union is disjoint-only by design and
the refusal says so and names the pairwise spelling. `collapse`'s
`Merged` arm (`names/emit_union.rs`) is unreachable until this is
answered, and says so.

## Where it stands

DOCM's slate; a ruling, not a unit, until Ev weighs in — it revises
DM4's "no declare field" sentence either way.

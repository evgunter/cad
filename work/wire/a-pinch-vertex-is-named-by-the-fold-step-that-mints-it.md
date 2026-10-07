---
id: a-pinch-vertex-is-named-by-the-fold-step-that-mints-it
kind: issue
title: A union's pinch vertex is named by the fold step that mints it, so its name depends on the member order
status: open
opened: 2026-10-06
priority: P1
cost: M
---


## What

Found on TANG's PR 4129, and true on main for two holes as well.

The union of the plate and leaning wedges whose footprints meet at one
vertex of its top (`crates/editor-core/tests/union_pinch_member_order.rs`,
`tilted_holes`) names that vertex differently by member order. The name
comes from whichever fold step minted the vertex
(`names/emit_topo.rs`, `name_boolean_vertices`):

- **Plate first.** The plate's step against the first wedge mints a
  one-segment `Seam` name.
- **Two wedges before the plate.** The plate's step mints a junction of
  four seam lines.
- **All three wedges before the plate.** A junction of six.

Measured with two wedges on main's arms: [1, 2, 0] and [2, 1, 0] carry a
four-line junction, and every other order a one-segment name. Within
one minting configuration the junction name is identical. That much is
pinned by
`the_junction_where_the_wedges_meet_has_one_name_in_every_member_order`.

**The wedges alone** (PR 4129, review 5). A union of the leaning
wedges without the plate mints a different junction name per member
order: three wedges give 3 distinct names over their 6 orders, four
wedges 10 over 24. P − U and U − P against the plate inherit the
divergence. Main refused these unions (`PierceRunsUnordered`, then
`Naming(Emission)`), so the class is newly reached, not newly made.
Each name denotes the right vertex.

No name rebinds to other geometry. But `emit_union_member_order`'s
contract is that a union's names do not depend on the order of its
members, and this one does.

## The shape to give

A union-level canonical name for a vertex the collapse sees minted at
different fold steps. For example, the junction of every seam line
through it in the finished body, whichever step minted it. Pin it with a
row over every member order.

---
id: a-vertex-on-face-row-follows-its-face-not-the-part-it-rests-on
kind: issue
title: A vertex-on-face row follows its face's lineage, not the part of the face the vertex rests on: ∩ keeps it stale, ∖ drops it
status: open
opened: 2026-10-03
priority: P1
cost: M
---


## What

`y` in `a_dangling_null_edge_holding_another_pairs_cut_builds_in_every_op`
(`crates/topo/tests/union_flush_onto_edge_contact.rs`, built by
`pit_holding_spikes`) is a block with a pyramidal pit and a spike in
the pit. The spike's far corners (0.9, 1, 0.05) and (1, 0.9, 0.05)
rest on the unit cube's faces y = 1 and x = 1, inside the pit. With
the cube first:

- `cube ∩ y` builds at its volume and fails 3′
  `StaleContactDeclaration` `VertexOnFace` (13v3, 5v1). 13v3 is the
  corner at (0.9, 1, 0.05) and 5v1 is the cube's face y = 1. That face
  survives where it lies outside the pit, but not around the corner,
  which sits inside the pit.
- `cube ∖ y` builds at its volume and fails 3′ `UndeclaredContact`
  `VertexOnFace` at (0.9, 1, 0.05).

The other four ops pass. Both failures are pinned there as they stand.

## Not the shared vertex

The same `y` against the brick [0.3, 1] × [0.3, 1] × [−0.5, 1], which
never reaches the origin, fails 3′ the same two ways. On main as well
(measured in a main worktree), so the origin's null edges play no
part. The spike alone against the cube passes 3′ in all six ops, and
there the cube's face y = 1 does not survive near the corner.

## Cause (read, not measured)

`ops::remap_contacts` (`crates/topo/src/boolean/ops.rs`) carries a
vertex-on-face row by face lineage (`desc.live_face`). Nothing checks
that the surviving part of the face is the part the vertex rests on.

## Owed

Keep a vertex-on-face row exactly when the result's face around the
vertex is the face the row names, and flip the pin to 3′ passing in
all six ops.

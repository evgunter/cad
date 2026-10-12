---
id: a-vertex-on-face-row-follows-its-face-not-the-part-it-rests-on
kind: issue
title: A vertex-on-face row follows its face's lineage, not the part of the face the vertex rests on: ∩ keeps it stale, ∖ drops it
status: closed
opened: 2026-10-03
priority: P1
cost: M
closed: 2026-10-12
pr: 4650
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

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: a lineage bug in the declared v-on-f rows of remap_contacts (Stale/UndeclaredContact); stage 4 retires those records and refusals. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Released by stage 4 F (2026-10-11)

Stage 4 F (`declared-pairs-retire`) retired every boolean declaration: no author can declare a pair, the glue is what the margins decide, and the declare offer, the flush detector and the carried declarations are gone. What this row waited on has fired; re-read it against that tree.

## 2026-10-11, fixed at the remap (FLUSH, `flush/corner-ruling-tier3`)

The cause is this row's: `carry_rows` chased a vertex-on-face record's
face by key, and a division of the face keeps the key on one piece,
whichever it is. FLUSH's
`a-box-and-a-cylinder-through-its-corner-edge-intersect-past-tier-3-prime`
(a cylinder's ruling along a box edge, the same defect on a curved
face, with the plain booleans and no declaration) reads a record's face
through the face's division rows (the join's and the welds' fragment
logs) and lands it on the piece whose interior holds the vertex, or
drops it where no live piece does (`boolean::ops::Descendants::holder`).
On that branch `a_dangling_null_edge_holding_another_pairs_cut_builds_in_every_op`
passes tier 3′ in all six ops with the pin flipped (`drops` removed).
The row does not need INTENT stage 4: the records are discovered, not
declared. Left for FUSE's orchestrator to close on the merge.

## 2026-10-12, closed (FUSE, PR 4893)

Fixed by FLUSH's PR 4650 (`boolean::ops::Descendants::holder`), now on
main. At ea1088c5, `a_dangling_null_edge_holding_another_pairs_cut_builds_in_every_op`
(`crates/topo/tests/union_flush_onto_edge_contact.rs`) passes 3′ in all
six ops with no pin left. Since PR 4893 the door is the one
`holder(face, rest)` read, which also covers a vertex a later stage
took.

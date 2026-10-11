---
id: a-boolean-clipping-a-carried-contact-at-a-pinch-records-no-contact-for-the-pierce
kind: issue
title: A boolean that clips an operand's carried contact at a pinch, keeping the pierce as two vertices, records nothing for the pierce point, and 3′ refuses the result
status: open
opened: 2026-10-11
priority: P1
cost: M
---


Found by stage 4 F (`declared-pairs-retire`). F carries an operand's own
contact records mechanically into every pair boolean and union fold step,
which closed `a-boolean-drops-its-operands-own-contact-records` (its
six member-order scenes, the folded union, plate ∪ blocks, blocks ∪
plate, plate ∖ blocks, X ∖ plate, X ∩ plate and plate ∩ X all pass 3′
now). Four rows still refuse 3′.

## Witnesses

From `crates/editor-core/tests/union_pinch_member_order.rs`, which pins
each refusal as it stands and names this file:

- `the_plate_against_the_joined_blocks_welds_a_kept_pinch_only`:
  plate ∩ blocks refuses `UndeclaredContact` `EdgeEdgeOverlap`
  (z 0.75) and `VertexVertex` at the pinch (1.5, 1, 1).
- `a_slab_holding_the_contact_welds_only_a_pinch_on_one_fragment`:
  plate ∖ X refuses EdgeEdgeOverlap (z 0.75) and VertexVertex at the
  pinch. X ∪ plate and plate ∪ X refuse EdgeEdgeOverlap (z 1.5) and
  VertexVertex at the pinch.

In each, the result keeps the pierce point as two vertices, or keeps the
two footprints apart. The carried records cover the operand's own
contact. The op's clipping then makes the two vertices at the pinch and
the edge overlap below or above it, and no record covers that pair.
Before F, the same four also refused `VertexOnEdge` on the dropped
records, and the carry removed that.

## Owed

The op records the contact its clipping leaves between two cells that
lie at one point (the pinch's two kept vertices) and the edges'
overlap. That comes from the carried record's lineage through the clip,
not from geometry (the substitution door, `boolean::ops::carry`). When
it does, the four pins turn green.

## Green on a recorded fusion, not on lineage (2026-10-11, FLUSH's PR 4728)

The four pins pass 3′ since FLUSH's PR 4728: plate ∩ blocks, plate ∖ X,
X ∪ plate and plate ∪ X. The witnesses are renamed
`the_plate_against_the_joined_blocks_builds_in_every_op` and
`a_slab_holding_the_contact_is_one_vertex_only_on_one_fragment`, and
`PIERCE_UNRECORDED` is dropped.
- The two pierces of the plate's top at the pinch now mint one ring
  (`vtxfac::mint_ring`).
- Grouping them is a value decision (`bool_contact_vertex`), recorded
  as a `VertexFusion` row with its margin. A vertex-vertex record of
  the piercing operand's cites that row (`ops::ledger`). The carry
  turns it into records of every pair of result cells the two reach,
  the pair at the pinch among them.
- Each pin asserts that record and that row (`ring_fusion_recorded`).

**The remainder, which this row keeps.** That record is backed by a
fresh value decision of the op's. It is not backed by the carried
record's lineage through the clip, which is what "Owed" asks for. The
row stays open on that.

**What backs the edge-edge overlap the pins used to refuse** (traced
2026-10-11 on PR 4728's head, release). Each of the four results holds
one overlap of the two blocks' edges along x = 1.5, y = 1. The census
accepts it end by end.
- **The pinch end (z = 1):** two vertices on one point key, which the
  census reads as structural. Beside them is the ring's vertex-vertex
  record, citing its `VertexFusion` row.
- **The far end, through lineage:**
  - `plate ∖ X`: both edges end at one vertex.
  - `plate ∩ blocks`: one edge ends on the other's interior at z 0.5.
    The vertex-on-edge record there cites `Carried { input: 1 }`, the
    blocks' union's record, which that union decided (`VertexFusion`,
    `OnCarrier`).
  - `X ∪ plate` and `plate ∪ X`: the same at z 2.0, carried from `X`,
    which carried it from the blocks' union, which decided it.

So the overlap's far end is backed by the carried record's lineage
through the clip, and its pinch end is backed by the ring's one key with
a fresh fusion row beside it. What "Owed" asks for is now the pinch end
alone: a record of the pinch's two kept vertices from the carried record's lineage,
rather than from the ring's value decision. The pins assert both ends
(`ring_fusion_recorded`, `far_end_backed`).


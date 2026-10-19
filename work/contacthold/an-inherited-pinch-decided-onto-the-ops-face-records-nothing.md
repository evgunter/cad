---
id: an-inherited-pinch-decided-onto-the-ops-face-records-nothing
kind: issue
title: A cut whose face passes through a pinch an operand already holds decides the copies onto that face and records nothing
status: open
opened: 2026-10-12
design: true
priority: P2
cost: M
---


Filed by the fix pass of PR 4765 (`contacthold/zero-to-distinct-cells-audit`),
from that PR's review probes.

## What

A boolean records the vertex identity behind a pinch it makes
(`topo` `boolean/ops.rs::pinch_rows`, emitted by `carry`), and skips a
key where every vertex descends from copies an operand already held on
one key (`boolean/reduce.rs::PendingRow`). That skip also covers an op
whose own face passes through the inherited pinch: the reduction
decides each copy onto that face (a margin-decided Zero), the result
keeps both copies on one key, and no row is emitted.

## Witness

`crates/topo/tests/records_cite_their_decision.rs::copies_an_operand_already_held_on_one_key_record_no_pinch_row`
pins today's behaviour: the L-prism's intersection with the tilted
cube (two lobes on one key at `(1, 1, 1)`), intersected with a side-8
cube whose face passes through the corner at normals `(1, 1, 0)`,
`(−0.3, 1, −0.5)` and `(1, −0.2, −0.4)`, in either operand order,
keeps the two copies on one key and has no `VertexFusion` row. It
matches main's expectation in
`crates/editor-core/tests/coincidence_door.rs::a_ring_serving_two_pierces_records_their_fusion_unproven`
("Two pierces already on one point key record nothing": a slab whose
face runs through the pinch, intersected with it, records no fusion).

## The question

DESIGN's coincidence discipline (D1, D10) calls "an ON verdict that
would make pieces of one result touch" a coincidence. Here the pieces
already touched in the operand, and the op's verdict puts its own face
through the touch point: the surviving pieces now touch at a point the
new face also passes through, decided by this op's margin. Two
readings:

- **Structure.** The touch is the operand's, already recorded (or not)
  where it was made; this op decided where its face runs, not that the
  pieces touch. No row, as today.
- **A new coincidence.** The lobes' touch now sits on this op's face, a
  placement decided by a margin that a family edit can break (the face
  moves off the point and one lobe is cut differently from the other).
  The row of each copy on the face is owed.

Which one D1 means decides whether the skip stays whole or narrows to
copies the op decided nothing about.

## Answered (2026-10-19, WIRE `wire-citerows`)

The second reading. DM4 (`crates/editor-core/REFERENCES.md`, approved
on PR 5329) makes a node's rows its evaluation's decisions, complete,
and `a-boolean-records-a-zero-decision-only-where-a-record-cites-it`
(cleave) makes every vertex identity a boolean decides with a margin a
row, cited or not. `pinch_rows` and its skip rule are gone; the witness
is now `records_cite_their_decision::a_decision_at_copies_an_operand_already_held_is_a_row`
(4 rows at normal `(1, 1, 0)`), and
`coincidence_door::a_ring_serving_two_pierces_records_their_fusion_unproven`
asserts the slab's intersection records the copies on its face.

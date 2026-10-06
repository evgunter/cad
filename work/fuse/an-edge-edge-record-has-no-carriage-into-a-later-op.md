---
id: an-edge-edge-record-has-no-carriage-into-a-later-op
kind: issue
title: An edge-edge record has no carriage into a later op: CarriedContacts holds no ee rows, and no split lineage places one
status: closed
opened: 2026-10-06
closed: 2026-10-06
pr: 4140
priority: P2
cost: M
---


## The finding

PR 3955 (step 1 of PR 3881's ruling) adds the stored edge-edge record
(`EeContact`, `ContactRecords.ee`): what the substitution door
(`carry` in `crates/topo/src/boolean/ops.rs`) leaves of a v-v record
whose two vertices a join made into edges, the two edges crossing or
overlapping there. The census backs `EdgeEdgeCross` with it and
confirms it stale where the edges do not meet.

A boolean takes an operand's own records back in through
`CarriedContacts`, which holds `vv`, `vf` and `ve` rows and no `ee`
rows, so an edge-edge record on an operand does not re-enter the next
op. Nothing reaches this today: no output stage runs the join yet.

## Owed (with step 2, the join at every output stage)

- `CarriedContacts.ee`, validated at the door like its siblings.
- Edge-split lineage for it (`split_lineage`): when the reduction
  splits either edge, the record must land on the piece that holds the
  crossing, or on every piece that still overlaps the other edge. The
  `(vertex, edge)` rule decides by the split's own vertex against a
  vertex the record names; an edge-edge record names no vertex, so the
  rule for it has to be written, not copied.

## Closed (FUSE, PR 4140, 2026-10-06)

- `CarriedContacts.ee` holds edge-edge rows, validated at the door
  (`carried e-e edge key does not resolve`, `carried e-e pair names
  one edge twice`), and the substitution door (`carry` in
  `crates/topo/src/boolean/ops.rs`) carries them like the join's.
- Edge-split lineage (`ee_lineage`): where the reduction split either
  edge, the record lands on every pair of pieces whose interiors
  still meet, decided by the census's own segment questions
  (`census::segment_interiors_meet`), and a split vertex minted on
  the contact is recorded against the piece it rests on (or the
  vertex it meets).
- Rows: `an_edge_edge_record_carries_onto_the_piece_of_a_split_edge_that_holds_it`
  (red when the door drops carried `ee` rows, and when lineage leaves
  the record on the split edge's own key) and
  `a_carried_edge_edge_row_whose_key_does_not_resolve_refuses_at_the_door`,
  in `crates/topo/tests/vertex_on_edge_records.rs`.
- Not pinned by a row: the arm where a split lands exactly on the
  crossing (the record becomes a `(vertex, edge)` record).


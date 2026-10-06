---
id: an-edge-edge-record-has-no-carriage-into-a-later-op
kind: issue
title: An edge-edge record has no carriage into a later op: CarriedContacts holds no ee rows, and no split lineage places one
status: open
opened: 2026-10-06
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

---
id: transform-and-pattern-drop-a-values-contact-records
kind: issue
title: Transform and Pattern nodes drop a value's contact records, so a touching boolean result refuses at rest one node downstream with no change to it
status: open
opened: 2026-10-02
priority: P1
cost: M
refs: [3856]
---


## What

In `crates/editor-core/src/eval/wire.rs` only instantiate and the boolean fill a value's contact records; every other node's `OpOut::plain` defaults them empty. A `Transform` or `Pattern` of a boolean result whose pieces touch (a declared corner kiss, say) therefore hands on a body with no records, and the at-rest pseudomanifold census refuses it as `UndeclaredContact` though the geometry is unchanged. Owed whichever way the split pinch question is answered (cross-operand records can never become shared keys). A row: transform a declared-touching boolean result and run the at-rest gate.

Found by the TQUERY designer pair weighing `split-halves-have-no-contact-records-so-no-pseudomanifold-self-check` (2026-10-02); read from the tree, NOT run — measure first.

## The kernel half (2026-10-11, CONTACTHOLD)

`topo-surgery-verbs-drop-a-bodys-contact-records` added the topo carry:
`ContactRecords::carried_through` (since CONTACTHOLD's one carry, it
takes an op's `topo::Lineage`) and each surgery verb's `carry_contacts`.
`transform_rigid` keeps every key, so a `Transform` node's carry is
`carried_from(0)`, as `wire_place_in_world` already does. A `Pattern`
grafts copies, so it re-keys each copy's records through its graft keys,
as `product.rs::carry_contacts` does. The editor-core nodes still
default to `OpOut::plain`'s empty records.

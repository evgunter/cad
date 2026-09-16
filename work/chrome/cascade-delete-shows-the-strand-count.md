---
id: cascade-delete-shows-the-strand-count
kind: issue
title: The cascade-delete affordance shows the names a delete would strand, beside its dependent count (DM7)
status: parked
opened: 2026-09-16
blocked_on: [deletenode-strands-a-declare-payload-name]
---


(EDIT orchestrator, 2026-09-16) Ev ruled **DM7**
(`crates/editor-core/REFERENCES.md`): `DeleteNode` stays legal when a
payload name (a fillet's selection, a `Declare` pair, a mate head, a
`FaceFrame` face, a measure ref) names the deleted node, and the
accepted edit's `Applied.maintenance` reports every stranded
`(node, name)`. EDIT's unit `deletenode-strands-a-declare-payload-name`
builds the report. This row is its chrome consequence: the cascade
affordance (`DeleteAffordance::of`, the count on the button before the
click) shows the strand count beside the dependent count, and the
delete's result surfaces the report where the user can reach `Rebind`.
Parked until EDIT's unit merges; nothing here was asked of CHROME.

## What the kernel hands it (2026-09-16, EDIT's unit at review)

EDIT's build is PR 2753. The API this row consumes:
`Applied::maintenance` is a `Vec<Maintenance>`; `Maintenance::Strand
{ node, name }` is one stranded payload name, `node` the surviving
carrier and `name` the frozen name whose minting node the edit deleted
(`name.node`). `Maintenance` carries a prose `Display` for every arm, so
the affordance can render a row rather than compose a sentence about it.
The count before the click is derivable without applying the edit —
`cascade_delete_order` gives the doomed set and `Node::payload_names`
over the survivors gives the names those ids minted — and the count
after is the `Strand` rows of each accepted delete. Nothing is
persisted: maintenance is derived, so a reopened document shows no
history of it.

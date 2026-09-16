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

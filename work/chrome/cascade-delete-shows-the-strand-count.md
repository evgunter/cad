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

## What the kernel hands it, and what the count IS (2026-09-16, EDIT's unit at review)

EDIT's build is PR 2753. **The door's rows and the affordance's number
are not the same quantity, and this row asked for the wrong one.**

**What the door reports.** Per accepted `DeleteNode`:
`Applied::maintenance` carries a `Maintenance::Strand { node, name }`
for every node still in the document that carries a `StableName` whose
minting node THAT edit removed. `Maintenance` renders in prose on
every arm, so the result panel can show rows rather than compose
sentences about them.

**Why that is not the pre-click number.** A cascade is a SEQUENCE of
deletes, and a row can be true of the document between two steps and
about nothing that outlives the cascade. Cascading a declared union's
`Declare` is the case: `cascade_delete_order` answers `[union, decl]`,
step 1 deletes the union and reports **eight** strands on the
`Declare`'s pairs, step 2 then deletes the `Declare` itself. Summing
the door's rows would put "8" on a button whose click strands nothing.
Pinned by `rv_a_cascade_reports_strands_on_carriers_it_then_deletes`
(`crates/editor-core/tests/rv_dm7_probes.rs`), which asserts both
numbers — 8 reported, 0 surviving.

**So the affordance counts the END state**, computed without applying
anything: let `doomed = cascade_delete_order(doc, id)`; the number is
the count of names carried by nodes OUTSIDE `doomed`
(`Node::payload_names`) whose `name.node` is INSIDE it. That is the
quantity a user is being warned about — what will still be in the
document afterwards, pointing at nothing — and it is derivable before
the click, which the door's rows are not.

**Both, in the chrome.** The pre-click count is the end-state number
above; the post-click panel lists the `Strand` rows each accepted
delete returned, which is where `Rebind` is reached from. A user who
sees "8 stranded" during the cascade and "0" at the end has been told
the truth twice about different moments, and the affordance should not
show the first.

Nothing is persisted: maintenance is derived, so a reopened document
shows no history of it.

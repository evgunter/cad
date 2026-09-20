---
id: cascade-delete-shows-the-strand-count
kind: issue
title: The cascade-delete affordance shows the names a delete would strand, beside its dependent count (DM7)
status: open
opened: 2026-09-16
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

## Unparked (2026-09-16, EDIT orchestrator)

The trigger fired: EDIT's unit
`deletenode-strands-a-declare-payload-name` merged as PR #2753 and its
`## Closed` section names the API the affordance consumes (the
`Applied.maintenance` strand report; see that item's `## Built`). This
header was edited from outside CHROME's fence only to keep the tracker
true — the parked-on-closed lint error — and asks nothing of CHROME
beyond what the body already says.

## The second carrier, by the same definition (2026-09-16, EDIT's appearance-strand unit)

DM7 was widened on EDIT's third `[ev]` PR of 2026-09-16 to cover every
reference the document holds under N5 semantics, so the report now has
a second arm — `Maintenance::StrandedAppearance { name }`, an
attachment the document's appearance store still holds under a name
whose minting node the delete removed — and this row's end-state
definition gains it without changing shape: the pre-click count is the
names carried by nodes outside `doomed` whose `name.node` is inside it,
PLUS the keys of `doc.appearance()` whose `name.node` is inside it. The
store is not a node and no cascade deletes it, so an appearance key
counted before the click is still stranded after it — the transient
case that made the payload numbers differ has no analogue here, and the
two counts coincide for this half. The post-click panel lists the new
arm beside `Strand`; it renders its own prose sentence like every
other, and it carries no node, because the store carries the
attachment rather than any node. The kernel's repairs for this row
are `Rebind`, which moves the key to a live name, and
`ClearAppearance`, which retires it and deliberately does not require
a live node — the arm's own doc says which. What a panel offers for
such a row is CHROME's. Nothing here asks CHROME for more than the
count and the list it already owed. Appended from outside CHROME's
fence, as the earlier sections were, only to keep the definition true
of the kernel the affordance reads.

## Widened (2026-09-19, PR #2874)

The delete door has a THIRD row now — `Maintenance::OrphanedDeclare
{ declare }`, a `Declare` whose last consumer the delete removed — and
what the affordance owes is the NET of strands AND orphans over the
doomed set, by the same end-state definition this row already uses.

The orphan's transient is the declare case again, and it is the
CASCADE door's to cancel, not `apply`'s. `apply` is a function of
`(document, edit)` and answers what one delete did; over an ACTION —
`cascade_delete_order`'s whole sequence — the net is the cascade
door's answer, and **nothing in the tree computes it today**. In the
viewer that door is `Session::commit_action`, which composes exactly
this loop; Python has no cascade door at all, so `pncad.pyi`'s
`orphaned_declare` paragraph states the transient in one sentence
rather than pointing at a door the external consumer cannot reach.

The cancellation is one line, and the reviewer measured it: the
subject of a transient orphan row is ALWAYS in the doomed set, so
`rows.filter(|row| !matches!(row, OrphanedDeclare { declare } if
doomed.contains(declare)))` is the whole of it. Pinned by
`dm7_delete_strands::the_orphan_transient_is_cancellable_at_the_cascade_door`
(written on `review/orphan-rv` as
`rv_the_orphan_transient_is_cancellable_at_the_cascade_door` and
adopted into the unit's suite), whose last assertion is that the net
is empty and that nothing computes it. The sites that would compose
it are `Session::commit_action` in `crates/viewer` and any future
cascade affordance.

The pre-click count adds, to the strand number above, the `Declare`s
**outside** `doomed` every one of whose consumers is **inside** it. A
`Declare` is its consumer's INPUT, so `cascade_delete_order` — which
walks dependents — never pulls one in on the consumer's account: a
cascade of the union leaves the declaration behind and the count is
1. A cascade of the DECLARATION is the transient: the `Declare` is
the target, so it is inside `doomed`, and the count is 0 while the
rows the run produced say 1. That difference is the whole of what the
filter cancels.

Appended from outside CHROME's fence, as the earlier sections were,
only to keep the definition true of the kernel the affordance reads.
Nothing here asks CHROME for more than the count and the list it
already owed.

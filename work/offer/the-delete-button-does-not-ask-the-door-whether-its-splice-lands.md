---
id: the-delete-button-does-not-ask-the-door-whether-its-splice-lands
kind: issue
title: viewer: the delete button is drawn live without asking the door whether the planned splice lands
status: closed
opened: 2026-11-22
priority: P4
cost: E
closed: 2026-11-22
pr: 7346
---


Found by the lane for `cascade-delete-shows-the-strand-count`.

`DocSession::delete_refusal` (`crates/viewer/src/session.rs`) answers
`None` for every node whose delete PLANS
(`editor_core::continuation::splice` answers a `Splice`): its doc says
"a held node's splice is applied at the click rather than rehearsed
every frame". So a plan one of the edit doors would refuse is drawn as
a live button, and the click answers the refusal.

The delete affordance now rehearses the plan to count the names it
strands (`session::delete::stranded_names`,
`crates/viewer/src/session/delete.rs`), once per document change and
node (`DocSession::priced`). That rehearsal holds the door's answer and
discards a refusal: a refused plan counts zero stranded names, which is
true of a click that deletes nothing, while the label still prices the
dependents the plan would have taken.

**No witness.** No document was found whose delete plans and whose
plan a door then refuses; `splice`'s own faults are typed and reach
the button already (`Refusal::Unspliced`). The row is the discarded
`Err`, not a reproduced wrong button.

What closes it: `delete_refusal` and the affordance read one rehearsal,
so a refused plan disables the button with the door's refusal — or a
statement, with its reason, that a planned splice cannot be refused.

## Closed (2026-11-22, PR 7346)

`DocSession::delete_refusal` and `DocSession::delete_affordance` read
one rehearsal (`DocSession::priced_delete`): `delete_node`'s own
action staged through `stage_run` and dropped, kept per history state
and node. A plan a door refuses now answers that door's refusal, the
button is drawn disabled with it, and its affordance prices nothing
(no dependents, no stranded names), as a splice fault's already did.
No `Err` is discarded. Still no witness document for a planned splice
a door refuses, so the new arm is reached by no row.


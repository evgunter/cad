---
id: the-delete-button-does-not-ask-the-door-whether-its-splice-lands
kind: issue
title: viewer: the delete button is drawn live without asking the door whether the planned splice lands
status: open
opened: 2026-11-22
priority: P4
cost: E
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

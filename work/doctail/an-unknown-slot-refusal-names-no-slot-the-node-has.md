---
id: an-unknown-slot-refusal-names-no-slot-the-node-has
kind: issue
title: An unknown-slot edit refusal says to edit a slot the node has without naming one
status: open
opened: 2026-11-06
priority: P4
cost: E
---


Found by the sweep of `fillet-step-discoverability-six-door-refusals`
(ROUND) for refusals that name the bad thing and not the accepted
things.

`crates/editor-core/src/edit.rs`'s `impl Display for EditError` renders
`UnknownSlot { id, slot }` as *"`id` has no slot `slot`"* with the
recourse *"edit a slot this node has"* — which names none. The node's
slots are what the reader needs, and the variant carries only the id
and the refused slot, so the `Display` has no way to list them.

The repair is not the ROUND unit's small one: it needs the node's slot
roster at refusal time (a field on the variant, or a door that renders
with the document at hand), which is a payload change on DOCTAIL's
ground.

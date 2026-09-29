---
id: a-drag-on-a-deleted-nodes-slot-says-the-slot-is-missing
kind: issue
title: A drag on a deleted node's slot is refused as a missing slot, where the pane says the node is deleted
status: open
opened: 2026-09-28
priority: P3
cost: M
refs: [three-spellings-say-a-parameter-is-not-declared]
---

Found by the sweep in `three-spellings-say-a-parameter-is-not-declared`,
which routed the undeclared-PARAMETER pair through one composition
(`Refusal::undeclared_wording`). The node-shaped sibling of that pair
was left, because its repair changes what a refusal SAYS HAPPENED and
not only its words, and it edits the session's door.

## The pair

- `crates/viewer/src/pane/properties.rs`, `standing_verdict`'s
  `Standing::Node { present: false }` arm (about `:957`): the pane
  draws the one word *deleted* beside the node's number.
- `crates/viewer/src/session.rs`, `driver_of` (about `:229`), reached
  through `guard_driven` from `DocSession::begin_gesture`: a drag on a
  slot of that same node — one the frame drew before the delete landed
  — finds no row in `props::slot_rows` and refuses
  `Refusal::NoSuchSlot { node, slot }`, whose `Display`
  (`crates/viewer/src/session/refuse.rs`, the `NoSuchSlot` arm) reads
  *node {n} has no {slot} slot*.

So the pane says the node is gone and the status line says the node is
there but lacks a slot. The second is false: `props::slot_rows` returns
nothing for a node that does not exist, and `driver_of` cannot tell
that from a node that exists without such a slot.

## What a fix has to decide

Whether a missing node is its own refusal (the pane's *deleted*, with a
composer in `refuse.rs` both surfaces read, as `undeclared_wording` is
for a parameter) or `NoSuchSlot` learns to say which half is missing.
Either way `driver_of` has to ask the document whether the node exists
before it asks for the slot. `session.rs` is the door and is edited;
`refuse.rs` holds the words.

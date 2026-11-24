---
id: a-placement-and-its-part-do-not-link-to-each-other
kind: unit
title: viewer: a placement does not link to the part it places, nor a part to its placements
status: closed
opened: 2026-11-22
priority: P1
cost: M
pr: 7356
branch: offer/a-node-shows-what-it-reads-and-what-reads-it
closed: 2026-11-23
---

**Ev reported this** (in chat, 2026-11-22): "unclear how to go from a placement to its part, or a part to its placement(s)".

In the feature tree, a body row gets a world glyph when something places it (`crates/viewer/src/pane/features.rs:73-79`). But `TreeRow.placed` is a bool (`crates/viewer/src/tree.rs:270-273`), so the row cannot say which placements those are or lead to them. In the placement editor, "copy of: {copied}" is a plain `ui.label` (`crates/viewer/src/pane/create.rs:1021-1027`) beside a repoint button, with no link to the body. The View pane lists spaces and bodies as checkboxes only (`crates/viewer/src/pane/view.rs:97-121`).

The data is there: `doc.upstream(id)` (`crates/editor-core/src/doc.rs:2495`) already serves `world.rs:769`. The link widget is there too: `widgets::message_link` (`crates/viewer/src/widgets.rs:186`) plus `SessionOp::Select(Selection::Node(..))`. What is missing is a "placed by: …" list on a body's Properties and tree row, and a link on the placement's "copy of". This is one instance of `a-node-name-the-viewer-shows-is-not-a-link-to-it`, and a taker should land it with that row.

## The unit (2026-11-22; carries `a-made-body-does-not-show-the-profile-it-was-made-from`)

This unit builds the relations half of the decided names design.

- **Reads.** A node's Properties gets a "reads" section with one row per operand seat, from `Node::operand_rows`, named by `OperandSlot::label`. Examples: `profile: Profile 1`; for a subtract, `from:` and `tool:`; for a placement, `source:` with a link to the named variable and to the body behind it, keeping today's repoint button.
- **Read by.** It also gets a "read by" section: each node that reads one of this node's outputs, with the seat it reads through. This comes from the kernel's `Doc::read_sites`, made public. A body's placements are its read-by rows at the source seat.
- **The tree glyph.** The world glyph on a tree row is derived from the same read-by list. `world::placements_of` and `copies_where` become views of it, and their kernel home sits beside `Doc::upstream`.
- **Links.** Each entry is a link that selects the node, through `widgets::message_link` until the sentence widget lands.
- **Parts.** An instance's part is another document, so its row names the file and is not a link.

This unit runs alongside the kernel sentence unit. It names nodes through whatever spelling function main has when it lands.

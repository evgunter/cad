---
id: a-placement-and-its-part-do-not-link-to-each-other
kind: issue
title: viewer: a placement does not link to the part it places, nor a part to its placements
status: open
opened: 2026-11-22
priority: P1
cost: M
---

**Ev reported this** (in chat, 2026-11-22): "unclear how to go from a placement to its part, or a part to its placement(s)".

In the feature tree, a body row gets a world glyph when something places it (`crates/viewer/src/pane/features.rs:73-79`). But `TreeRow.placed` is a bool (`crates/viewer/src/tree.rs:270-273`), so the row cannot say which placements those are or lead to them. In the placement editor, "copy of: {copied}" is a plain `ui.label` (`crates/viewer/src/pane/create.rs:1021-1027`) beside a repoint button, with no link to the body. The View pane lists spaces and bodies as checkboxes only (`crates/viewer/src/pane/view.rs:97-121`).

The data is there: `doc.upstream(id)` (`crates/editor-core/src/doc.rs:2495`) already serves `world.rs:769`. The link widget is there too: `widgets::message_link` (`crates/viewer/src/widgets.rs:186`) plus `SessionOp::Select(Selection::Node(..))`. What is missing is a "placed by: …" list on a body's Properties and tree row, and a link on the placement's "copy of". This is one instance of `a-node-name-the-viewer-shows-is-not-a-link-to-it`, and a taker should land it with that row.

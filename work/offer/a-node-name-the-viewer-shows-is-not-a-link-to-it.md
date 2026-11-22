---
id: a-node-name-the-viewer-shows-is-not-a-link-to-it
kind: issue
title: viewer: a node's name shown in a pane, a notice or a refusal is plain text rather than a link to the node
status: open
opened: 2026-11-22
priority: P1
cost: M
---

**Ev reported this** (in chat, 2026-11-22): "any time it says something like 'Revolve 3', you should be able to click it to take you to it".

The mechanism exists and some sites use it: `widgets::message_link` (`crates/viewer/src/widgets.rs:186`) with `SessionOp::Select(Selection::Node(..))` is used by the tree's repair link (`crates/viewer/src/pane/features.rs:264`, `:362-373`), the variable and source rows (`crates/viewer/src/pane/properties.rs:195-238`), and the Checks findings (`crates/viewer/src/app.rs:1806-1830`). Elsewhere a name is plain text: the Properties headings (`properties.rs:513`, `:640`), the placement editor (`crates/viewer/src/pane/create.rs:643-645`, `:1026`), the View pane (`crates/viewer/src/pane/view.rs:120`, where the name is a checkbox label), and every refusal or notice sentence that embeds a `SpokenNode`.

The uniform fix is for a node name to reach the screen as a value the renderer can draw as a link, not pre-flattened into a string, so that every naming site is a link by construction rather than one at a time. That couples with `the-viewer-spells-a-node-as-its-kind-before-its-name` (the same sites, the same spelling) and makes `a-placement-and-its-part-do-not-link-to-each-other` and `a-made-body-does-not-show-the-profile-it-was-made-from` cheap. Narrow cousin: `work/chrome/a-checks-cell-for-a-vertex-selects-its-node.md`.

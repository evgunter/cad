---
id: a-node-name-the-viewer-shows-is-not-a-link-to-it
kind: unit
title: viewer: a node's name shown in a pane, a notice or a refusal is plain text rather than a link to the node
status: closed
opened: 2026-11-22
priority: P1
cost: M
branch: offer/a-mention-is-a-link
pr: 7557
closed: 2026-11-23
---

**Ev reported this** (in chat, 2026-11-22): "any time it says something like 'Revolve 3', you should be able to click it to take you to it".

The mechanism exists and some sites use it: `widgets::message_link` (`crates/viewer/src/widgets.rs:186`) with `SessionOp::Select(Selection::Node(..))` is used by the tree's repair link (`crates/viewer/src/pane/features.rs:264`, `:362-373`), the variable and source rows (`crates/viewer/src/pane/properties.rs:195-238`), and the Checks findings (`crates/viewer/src/app.rs:1806-1830`). Elsewhere a name is plain text: the Properties headings (`properties.rs:513`, `:640`), the placement editor (`crates/viewer/src/pane/create.rs:643-645`, `:1026`), the View pane (`crates/viewer/src/pane/view.rs:120`, where the name is a checkbox label), and every refusal or notice sentence that embeds a `SpokenNode`.

The uniform fix is for a node name to reach the screen as a value the renderer can draw as a link, not pre-flattened into a string, so that every naming site is a link by construction rather than one at a time. That couples with `the-viewer-spells-a-node-as-its-kind-before-its-name` (the same sites, the same spelling) and makes `a-placement-and-its-part-do-not-link-to-each-other` and `a-made-body-does-not-show-the-profile-it-was-made-from` cheap. Narrow cousin: `work/chrome/a-checks-cell-for-a-vertex-selects-its-node.md`.

## Decided (Ev, 2026-11-22, on PR #7339)

**Mentions.** A node named to a person is a mention inside a typed sentence: its id, kind and label, and the document its id is spelled in. `SpokenNode` loses its `Display`, and every kernel refusal, finding and notice is written as a sentence of words and mentions. The viewer draws each mention as a link that selects the node, but only when the document on screen is the mention's.

**Spelling.** On every surface a node reads as its label and tag, `"Revolve 3" (5c8b603262ca)`. A node with no label reads as kind and tag, `Extrude 3fa9c1d2a0b1`. The kind is never repeated beside a label. See DESIGN.md Band 1, "Node labels".

**Relations.** Properties shows what a node reads, seat by seat from `Node::operand_rows`, and what reads it, from `Doc::read_sites` made public. Each entry is a link. This covers the profile a revolve was made from, and the link between a placement and its part in both directions.

**Tree.** The tree scrolls the selected row into view whenever the selection changes from outside the tree.

No row is filed for names running off a pane's right edge: the pane scrolls (Ev).

## The unit (2026-11-22)

This unit builds the viewer half and stacks on `the-viewer-spells-a-node-as-its-kind-before-its-name`.
- **One sentence widget.** It draws a kernel `Sentence`, wrapped the way messages wrap today, with each linkable mention underlined. A click selects the node. A mention is a link only when its document is the one on screen and that document holds the node.
- **No text-only home for a message.** `frame::Message`, `CheckRow`, the Properties headings, the placement editor's "copy of" and the View pane's names all hold a sentence.
- **The tree scrolls to the selection.** It scrolls the selected row into view whenever the selection changes from outside the tree.
- **Feasibility comes first.** Before anything else, check whether egui can hit-test a mention inside one laid-out text block (`cursor_from_pos`). Neither designer checked this.

---
id: a-frame-pickers-closed-combo-draws-a-frames-pose-in-an-extend-row
kind: issue
title: viewer: frame_picker's closed combo names the picked frame with its pose, laid out at infinite width beside the label
status: open
opened: 2026-09-24
priority: P3
cost: E
refs: [feature-tree-row-labels-draw-an-unbounded-pose-in-an-extend-row]
---


`crates/viewer/src/pane/create.rs`'s `frame_picker` draws its
`egui::ComboBox` inside a `ui.horizontal` beside the form's label, and
its `selected_text` is the picked frame's name — for the add-datum and
add-profile forms, `ViewerBehavior::frame_names`, which is
`crate::tree::node_label`: the node number and the frame's pose. egui's
`ComboBox` takes the `Ui`'s wrap mode when it is given none
(`egui::containers::combo_box`'s `combo_box_dyn`), and a horizontal
row's is `Extend`, so the closed text is laid out at infinite width.

This is the feature tree's row label again
(`feature-tree-row-labels-draw-an-unbounded-pose-in-an-extend-row`,
closed on the layout question): the pose is bounded — about ninety
characters at most — so by the NAME-vs-SENTENCE test it is a long
NAME, reached by scrolling right rather than drawn wrongly. It is
filed because the combo is a second place the same label meets the
same `Extend` row, found by `chrome/create-messages`'s sweep of
`pane/create.rs` and not a message site (a combo's closed text has no
`widgets::message` door).

## What a fix owes

Whatever the tree's answer is, applied here too: a character bound on
the closed text with the whole label on hover, or
`ComboBox::truncate` / `wrap` with a width. `profile_plane_row` is a
free function that `create::tests` already drives headlessly.

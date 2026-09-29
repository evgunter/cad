---
id: a-part-choosers-pick-button-draws-a-file-name-at-any-width
kind: issue
title: viewer: the part chooser's pick button is labelled with a file name, which nothing bounds, and a button does not wrap
status: open
opened: 2026-09-24
priority: P3
cost: E
refs: [messages-in-the-creation-and-properties-panes-still-draw-past-their-row]
---


`crates/viewer/src/pane/create.rs`'s `part_entry` draws each part the
"Add part" chooser offers as `egui::Button::new(entry.file_name())`.
A file name is the user's, so by the NAME-vs-SENTENCE test in
`messages-in-the-creation-and-properties-panes-still-draw-past-their-row`
nothing this crate controls bounds its width — and a button lays its
text out on one line, so a long name makes the button, and then the
window (`part_window`, which ratchets up to its widest content), wider
than the pane that opened it. The sentences in the window then wrap at
that wider width.

`crate::widgets::message` has a link door (`message_link`) but no
button door, which is why the `chrome/create-messages` conversion left
this one: the id that used to sit beside the button now has a line of
its own under it, and the button is the last unbounded text in the
row.

## What a fix owes

A choice between a wrapping button door beside `message_link`, a
`message_link` in place of the button (the disabled-with-a-reason
state of the open document's own entry then needs a spelling), or a
character bound on the label at its source with the whole name on
hover. `part_entry` is a free function, so a row in
`pane/create.rs`'s `layout_tests` can drive it headlessly the way
`a_parts_id_is_said_under_its_pick_button_inside_the_pane` does.

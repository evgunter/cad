---
id: a-variable-is-edited-in-a-separate-widget-not-on-its-own-row
kind: issue
title: viewer: a variable is edited in a separate panel its list line links to; each variable should be its own editable row 'x = [0.3] [length] [mm]'
status: open
opened: 2026-11-01
priority: P1
cost: M
refs: [the-profile-editor-spends-two-lines-per-driven-argument, selecting-an-index-variable-shows-no-row]
---
Reported by Ev (chat, 2026-11-01): "instead of having a separate
variable editing widget that the list of variables links to, each
variable should just display like `x = [ 0.3 ] [ length ] [ mm ]`
where the first brackets are a box you can edit and the second and
third are units dropdowns. (the 'edit l' etc should link to these)."
Filed P1 at Ev's request.

## What happens

The Properties pane's variables list
(`crates/viewer/src/pane/properties.rs:178-200`) draws each variable
as a link (`variable_line`, `properties.rs:1614`; `defined_line`,
`properties.rs:1629`). Clicking it pushes
`SessionOp::Select(Selection::Variable(..))`, and the pane then draws
a separate editor for it in the `Selection::Variable` arm
(`properties.rs:97`). That editor holds the value field, the unit
picker (`variable_unit_ui`, `properties.rs:950`) and the bounds
(`variable_bounds_ui`, `properties.rs:1372`). The add-variable form
(`add_variable_ui`, `properties.rs:240`) is a third surface. Its
already-exists notice offers an `edit <name>` door
(`exists_notice`, `properties.rs:1530`) that also routes through the
selection.

## What Ev wants

- Each variable is one editable row in the list:
  `x = [ value ] [ dimension ▾ ] [ unit ▾ ]`. The value box is the
  field (an expression box for a defined variable). The two dropdowns
  are the dimension and the written unit.
- The `edit <var>` links (profile editor, slot rows, exists notice)
  scroll to and focus that row instead of opening a separate panel.
- Adding a variable should follow
  `creating-a-thing-and-editing-it-are-two-implementations`: create a
  row and edit it in place.

Open points: where the bounds and range doors go, and whether a
dimension change on a variable already in use is an edit the document
admits. Check editor-core before drawing the dropdown live.

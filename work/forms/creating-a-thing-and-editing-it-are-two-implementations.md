---
id: creating-a-thing-and-editing-it-are-two-implementations
kind: issue
title: viewer: every node kind with a create form and an edit panel has two implementations; create should commit and hand off to the edit panel
status: open
opened: 2026-11-01
priority: P1
cost: H
refs: [the-profile-create-form-is-a-second-profile-editor, the-extrude-and-revolve-forms-take-no-variables, a-variable-is-edited-in-a-separate-widget-not-on-its-own-row]
---
Reported by Ev (chat, 2026-11-01): "if there are any other things that
have separate creating and editing widgets, those should also be
removed in favor of either (a) creating and then editing the thing or
(b) creating the thing with certain values but it uses the exact same
code as the editing code there. and the choice between (a) and (b)
should be the same everywhere. but (a) seems nice and simple and it
has the desirable property that edits to a profile while you're
originally making it still count as edits so you can undo them."
Filed P1 at Ev's request (two implementations of one logic).

## The rule

One rule for the whole viewer, with (a) preferred: a create gesture
commits the node with default values, selects it, and leaves the rest
to the node's edit surface (its Properties slot rows, or the profile
editor). Every later change is an edit in the history, so undo reaches
changes made while the thing was being created.

## Where the pairs are

Create forms and seated tools in `crates/viewer/src/pane/create.rs`,
each with its own drafts (`crates/viewer/src/drafts.rs`) and fields,
next to the Properties pane's slot rows for the node they create
(`crates/viewer/src/pane/properties.rs:671` `slot_group_ui`,
`:750` `slot_value_ui`):

- add profile (`create.rs:749`), filed on its own as
  `the-profile-create-form-is-a-second-profile-editor`;
- extrude (`create.rs:1083`) and revolve (`create.rs:1133`), whose
  literal-only fields are filed as
  `the-extrude-and-revolve-forms-take-no-variables`;
- boolean, split, mirror, transform, pattern, projection, duplicate,
  blend (`create.rs:1212-1652`);
- the mate tool (`create.rs:546`) against the placement's constraint
  rows. PLACING's `mate-tool-commits-one-mate` holds a design question
  on the same ground. Reconcile with it, don't pre-empt it;
- add variable (`properties.rs:240`) against the variable's own
  editor, filed as
  `a-variable-is-edited-in-a-separate-widget-not-on-its-own-row`.

## Open points

- Seated tools whose input is a *pick* (boolean operands, blend edges,
  mate faces) need the pick before a node can exist. Under (a) the
  gesture still collects picks, then commits with default numbers. Say
  which inputs count as "what the node is" (asked at create) and which
  are "its values" (edited after).
- A node committed with defaults may refuse to evaluate. Its row must
  read as a node awaiting values, not as a failure.
- This touches several programs' files (FORMS, DOORS, HELD, PLACING,
  VSEAM). Probably a designer pass first, then cut into per-tool
  units.

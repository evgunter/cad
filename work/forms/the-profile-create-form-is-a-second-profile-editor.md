---
id: the-profile-create-form-is-a-second-profile-editor
kind: issue
title: viewer: the add-profile form is a second profile editor; a 'new profile' button should open the one editor on an empty profile
status: closed
closed: 2026-11-02
opened: 2026-11-01
priority: P1
cost: H
refs: [the-add-profile-form-takes-numbers-and-no-formula, creating-a-thing-and-editing-it-are-two-implementations]
branch: forms/profile-unit
---
Reported by Ev (chat, 2026-11-01): "the profile editor seems to be
separately implemented twice with different features. there's the tool
for creating a profile, and the tool for editing a profile that already
exists. the tool for creating a profile should be deleted and instead
there should be a button to create a new profile, which starts out
empty, and then the already-created-profile editor takes it from
there." Filed P1 at Ev's request (two implementations of one thing).

## What happens

The add-profile form (`ViewerBehavior::add_profile_ui`,
`crates/viewer/src/pane/create.rs:749`) is its own door: a shape
radio (circle, rectangle, path, `create.rs:755-758`), per-shape fields
(circle centre/radius/bore at `create.rs:771-800`), the sketch-on-face
toggle, and a path step list drawn through `path_steps_ui` with
`WrittenIn` doors (`create.rs:840`). The edit door
(`ViewerBehavior::edit_profile_ui`,
`crates/viewer/src/pane/profile.rs:54`) opens on a committed node with
`HeldArgs` doors: slot fields, expressions, units, range, names. The
module doc (`profile.rs:1-9`) claims the two are "one editor", but they
only share the step-list renderer. Each has features the other lacks.
Expressions are the known case, filed as
`the-add-profile-form-takes-numbers-and-no-formula`.

## What Ev wants

- Delete the add-profile form.
- Put a **new profile** button in its place. It commits an empty
  profile node and opens the existing editor on it.
- From then on every change goes through the edit door, so each one
  is an ordinary edit in the history and undo reaches it.

That subsumes `the-add-profile-form-takes-numbers-and-no-formula`.
The general rule for every create/edit pair is
`creating-a-thing-and-editing-it-are-two-implementations`.

## Open points for the implementer

- Can the document hold a profile with no steps, and what does it
  evaluate to? If an empty program refuses, the new-profile node needs
  a defined "empty, not yet a profile" reading rather than a red row.
- The circle and rectangle templates and the on-face placement need a
  home in the one editor (e.g. "insert circle" as steps), or else a
  ruling that they go.

## Closed — 2026-11-02, branch `forms/profile-unit`

The add-profile form is deleted. **New profile**
(`ViewerBehavior::new_profile_ui`, `crates/viewer/src/pane/create.rs`)
inserts a profile with no loops on the profile's own plane
(`SessionOp::AddProfile` with no loops) and the creation selects it
(`SessionOp::selects_what_it_makes`, read in `ViewerApp::perform_batch`
off `OpOutcome::made`), so the one editor (`pane::profile`) is open on
it. The circle, rectangle and path templates are the editor's add-loop
row (`drafts::template_steps`).

The editor commits live (`pane::profile::edit_door_ui`): a field once
committed (Enter, focus leaving, a drag released), a row or a loop at
its click, offered whole to the edit door
(`DocSession::edit_profile_report`) and landed as one
`SessionOp::EditProfile`, one undo step. A draft the door refuses is
held with its refusal said (`ProfileEdit::refused`, `HELD_BACK`), and
the next change that makes it pass lands it whole. Apply and Revert
are gone; **Discard changes** (`DISCARD`) drops a held draft. Undo
while a draft is held steps the history and drops the held draft, which
was an edit of the program undo replaced; nothing lands on its own, so
redo still reaches what undo took back.

The traced profile keeps a create gesture under the rule: **New traced
profile…** holds the two face latches and the trace's branch (crossing,
passage, sense), and inserts the traced template
(`drafts::traced_template`), whose numbers the editor edits.

With a face picked, New profile still inserts on the profile's own
plane: no door on main inserts a profile on a face's frame. That build
is `work/vseam/on-the-picked-face-draws-the-profile-there`, which now
carries this unit's evidence.

The open points: an empty profile is a document state
(`an-empty-profile-is-a-document-state`); the templates are the
editor's add-loop choices.

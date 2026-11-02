---
id: the-profile-create-form-is-a-second-profile-editor
kind: issue
title: viewer: the add-profile form is a second profile editor; a 'new profile' button should open the one editor on an empty profile
status: open
opened: 2026-11-01
priority: P1
cost: H
refs: [the-add-profile-form-takes-numbers-and-no-formula, creating-a-thing-and-editing-it-are-two-implementations]
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

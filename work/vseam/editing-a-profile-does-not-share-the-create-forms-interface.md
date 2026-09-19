---
id: editing-a-profile-does-not-share-the-create-forms-interface
kind: issue
title: Editing an existing profile has a different, worse interface than creating one (Ev-requested, high priority)
status: review
opened: 2026-09-17
branch: vseam/profile-editor
pr: 2862
---

## Ev's note (verbatim)

> viewing/editing an existing profile has a different and worse interface than making a new one--they should share the good interface by construction

## Priority

**High priority — requested directly by Ev** (in chat, 2026-09-17, from
Ev's own list of UI nits). This row goes ahead of the rest of the
program's order; see the plan's *Ev's requests* section.

## Where it lives

- **Creating** a profile goes through the create form: `pane/create.rs`,
  `add_profile_ui` and `path_steps_ui`. It is a step list with a verb
  picker per row, legal verbs offered through `sketch::admits_at`,
  typed fields, and a live preview in the viewport (`sketch::preview`).
- **Viewing or editing** an existing profile goes through the
  properties pane (`pane/properties.rs`, `props.rs`: `slot_rows` /
  `slot_groups`), which shows the node's slots as generic fields. It
  has none of the create form's structure.

## What a fix has to be

Ev: the two should "share the good interface **by construction**". So
not a second step-list UI written for the properties pane, but one
editor for a profile's path program that both doors use:
- create: an empty program, committed by adding a node
- edit: the node's program, committed as an edit to its slots

Also: the live preview should appear while editing, as it does while
creating.

The work crosses programs:
- `pane/create.rs` is VSEAM's (and VNEWS's).
- `pane/properties.rs` and `props.rs` are VGEOM's (and VNEWS's).
- The lowering (`sketch::loop_program`, `program_step`) sits in
  `sketch.rs`, which VGEOM works for its numbers and VSEAM for its held
  state.

Filed here because the core is what the viewer authors and lowers on
the document's behalf, which is VSEAM's charter. Announce it to VGEOM
and VNEWS when it is taken.

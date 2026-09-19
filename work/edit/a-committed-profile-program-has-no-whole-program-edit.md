---
id: a-committed-profile-program-has-no-whole-program-edit
kind: issue
title: A committed profile's program can be edited only one argument at a time; no edit reshapes it or writes it whole
status: open
opened: 2026-09-18
---


## What is missing

`DocEdit` (`crates/editor-core/src/edit.rs`) writes a profile node's
program through its SLOTS: `SetParam` / `SetStructuralParam` /
`SetExpression`, one argument each, and every one of them re-runs the
whole-program check afterwards (`check_profile_after_slot_edit`). There
is no edit that replaces a live profile's program — its verbs, their
order and count, arc modes, sides, windings, target forms, a split
circle's `n`, or its loop count. `SetMembers` is the only edit that
rewrites a live node's content wholesale, and it covers list inputs
only.

## What that costs the viewer, today

The viewer's profile editor is one editor with two doors (VSEAM,
`editing-a-profile-does-not-share-the-create-forms-interface`): the
add-profile form's step list, opened either on nothing (commits
`InsertNode`) or on a committed profile (commits
`SessionOp::EditProfile`, `crates/viewer/src/session.rs`,
`DocSession::edit_profile`). Because of the gap above, the edit door:

1. **Locks the shape.** Every control that would change the program's
   structure is drawn disabled on a committed profile
   (`forms::ShapeEdits::Locked`, `pane::create::path_steps_ui`), and a
   reshaped program sent anyway refuses
   `Refusal::ProfileRestructure` (`sketch::program_edits`). Reshaping a
   committed profile — inserting a leg, turning a line into an arc —
   has no route at all short of deleting the profile, which cascades
   through everything built on it.
2. **Writes numbers one slot at a time.** A set of numbers that is a
   valid profile together can pass through an intermediate program the
   door refuses. `session::accepted_order` holds a refused write back
   until the others have made it valid (a square moved bodily lands —
   `tests/profile_edit.rs`,
   `a_move_whose_first_write_alone_crosses_still_lands`), but where no
   order works the edit refuses `Refusal::ProfileEditOrder` about a
   state nobody wrote.

## What would close it

An edit that replaces a live profile node's whole program (plane
excepted — that is an input, DM6), validated once, as ONE `DocEdit`.
That is a vocabulary change at the edit layer and a persistence one
(every `DocEdit` is persisted, GQ3), so it is a design decision for
this program, not something the viewer can add. With it, the viewer
drops the lock and `accepted_order`, and the edit door commits the
editor's program as that one edit.

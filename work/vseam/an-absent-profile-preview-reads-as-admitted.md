---
id: an-absent-profile-preview-reads-as-admitted
kind: issue
title: preview_verdict answers not-refused for a preview never taken, so Apply and Add profile go live over a program check the door runs
status: open
opened: 2026-09-28
priority: P3
cost: M
refs: [the-mirror-class-is-unswept-outside-the-properties-pane]
---


Found by the census in `the-mirror-class-is-unswept-outside-the-properties-pane`
(VNEWS), at merge base `f4e9aa68b`.

## What happens

`preview_verdict` (`crates/viewer/src/pane/profile.rs`) returns `false`
("not refused") when the preview is `None`, and both of the profile
editor's commit buttons are gated on that answer:

- **Apply** in the edit door (`apply_and_revert`, profile.rs ~:409),
  gated on `moved && !refused`, pushing `SessionOp::EditProfile`;
- **Add profile** (`add_profile_ui`, `pane/create.rs` ~:1122), gated on
  `blocked.is_none() && !refused`, pushing `SessionOp::AddProfile`.

The comment on the `None` arm excuses *"the first frame a form is on
screen"*. The preview is also `None`, for as long as the state lasts,
when `ViewerApp`'s preview pass (`crates/viewer/src/app.rs`, the
`profile_previews` computation ~:1823) finds no placement for an
`Existing` frame: `landed_pair()` is `None` (nothing has landed since
Open), or `sketch::frame_placement` answers `None` (the frame did not
evaluate, e.g. a face frame on a failed body, or the landed document
lacks it).

In that state a loop that does not validate (a zero radius, a
self-intersection) leaves the button live. The doors then refuse:

- `DocSession::edit_profile` runs `whole.check(..)` before the commit
  and refuses `Refusal::Edit(EditError::ProfileProgramRefused)` at
  admission;
- `DocSession::add_profile` has no pre-commit check, and the same
  program refusal arrives from `commit`: late, but just as
  computable.

## Why the chrome can know

`ProfileProgram::check` (`crates/editor-core/src/program.rs`) validates
on the identity plane and never reads the frame. The preview withholds
its verdict only because DRAWING needs a placement. So the fix splits
the preview's two answers: take the program's verdict without a
placement, and draw the loops only where one exists. Then `None` means
only "not drawn", never "not judged".

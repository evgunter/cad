---
id: persist-edit-replay-forwards-the-edit-doors-recourse
kind: issue
title: edit: persist's EditReplay forwards the edit door's recourse to a load that made no edit
status: open
opened: 2026-09-29
priority: P3
cost: E
---


(EDIT, found by the recourse fix pass of
`edit-refusals-short-of-the-shape-guard`, PR 3490, sweeping the doors
that forward an `EditError`.)

## What

`PersistError::EditReplay` (`crates/editor-core/src/persist/mod.rs`,
its `Display` arm, "persist: edit {index} refused on replay: {error}")
forwards the edit door's whole sentence, recourse included. Two doors
reach it:

- **Load**, replaying a file's log. Nobody is making that edit: the
  file is damaged, or the kernel that wrote it has a defect, so "aim
  the edit at a node the document holds" is a recourse for nobody.
  The viewer's own replay (`crates/viewer/src/history.rs`,
  `ReplayError`) renders `EditError::problem` and
  `KERNEL_OR_FILE_DEFECT_ENDING` for the same case.
- **Save**, verifying the caller's log. A caller that assembled the
  log itself did author those edits, so the edit door's recourse is
  arguably theirs.

The arm cannot tell the two apart, which is why this is filed rather
than fixed in PR 3490: the fix needs the door to know which it is.

## Repair shape

Split the arm by door (or carry the door), render
`EditError::problem`, and end the load arm in
`geom_core::KERNEL_OR_FILE_DEFECT_ENDING`; keep the edit door's
recourse at save only if a caller-assembled log is a supported input.

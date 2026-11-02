---
id: a-step-reading-the-pen-is-shown-but-not-edited-or-previewed
kind: issue
title: A profile step that reads the pen is shown in the editor as written, but its arguments are not editable and the held program is not previewed
status: open
opened: 2026-11-02
priority: P2
cost: M
refs: [the-profile-editor-holds-kernel-steps-not-the-authored-program]
---


Found by the lane that made the profile editor hold the authored
program (`the-profile-editor-holds-kernel-steps-not-the-authored-program`).

A step that lowers against the pen — `line_to` a trace's crossing
(`ProgramTarget::OnTrace`), `along_trace`, or a run to a written
coordinate (`ProgramTarget::AtX`/`AtY`) — has no view at plain
numbers outside an evaluation: `sketch::HeldStep::step` resolves
through `resolve_written_loops`, which refuses it
(`WrittenLoopFault::Trace`, `TraceStepFault::Unevaluated`). So the
edit door (`pane::profile::edit_door_ui`) shows such a row in words
(`HeldStep::words`, a `StepRow::Written` row): it can be removed,
moved or have a step inserted after it, but its arguments (an
`AtX`'s `x`, a centre beside an `OnTrace` target) have no fields, and
the rows after it have no tip state (`step_rows_ui`), so their verb
and arc pickers grey nothing.

And the held program draws no preview: `ProfileEdit::shapes` answers
`None`, so the viewport keeps the committed profile until Apply
(`PEN_UNDRAWN` says so). An edit to any other step of a traced
profile is therefore applied blind.

A fix wants the prefix replay the evaluation runs (`editor-core`'s
`trace::resolve_chain` reading `Traces` built from the landed
evaluation's traced faces, which the evaluation builds privately in
`eval::wire::trace::drawn`) reachable from the viewer, or the edit
door's preview read off a scratch evaluation of the held program; and
the pen step's own arguments drawn through their slot doors.

---
id: a-face-charts-backwards-refusal-names-the-orientation-to-write-only-in-prose
kind: issue
title: a face chart's backwards refusal names the orientation to write only in its message, so Python cannot read the recourse
status: open
opened: 2026-11-22
priority: P3
cost: E
---



Found by the VSEAM lane that bound `Doc.draw_on_face`
(`work/vseam/doc-sketch-on-face-retires-for-a-profile-drawn-on-the-face`).

A profile drawn on a bottom cap with `orient=Orientation.identity()`
refuses at `evaluate` as `PoseConstruction::FaceChartBackwards { orient,
write }` (`crates/editor-core/src/pose.rs`, the arm's `Display`). Python
receives it as an `EvaluationError` with `kind == "pose_degenerate"` and
`inner_kind == "face_chart_backwards"` (`crates/pncad-py/src/tags.rs`,
`node_inner_kind_tag`). The orientation to write reaches Python only
inside the message, as `"Recourse: write the orientation x: +x, z: -z"`.
`EvaluationError` carries a fixed attribute set (`kind`, `inner_kind`,
`through`, `document`; `crates/pncad-py/src/py/value.rs`, `eval_err`).

The placement's twin crosses typed. `MateFault.write` is an
`Orientation` that a caller passes straight back as `orient=`. The
retired `Doc.sketch_on_face` test read it that way
(`tests/test_face_frame.py`, the backwards row, before this unit). Its
port, `test_an_orientation_written_backwards_refuses_naming_the_one_to_write`,
can now only match the message text. A caller that wants to repair the
draw has to parse prose.

What closes it: the chart's `write` crosses as an `Orientation` the
caller can hand back to `Doc.draw_on_face(orient=...)`. That could be an
arm payload on `EvaluationError`, or a typed read of the chart's
refusal the way `solve_document(...).fault(...)` reads a mate's. The
backwards row then asserts the typed value rather than the message.

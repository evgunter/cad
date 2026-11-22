---
id: the-save-bodys-one-key-wrapper-could-flatten-to-the-bare-doc
kind: issue
title: The save body's one-key snapshot wrapper could flatten to the bare Doc
status: open
opened: 2026-11-17
priority: P4
cost: C
---

Since `the-save-format-drops-its-edit-log` (PR 7094) a save body is
`{ "snapshot": <Doc> }`: the wrapper `FileBody`
(`crates/editor-core/src/persist/mod.rs:171`) has one field left, and
the save side's `SerBody` is the same. The question is whether the body
should be the bare `Doc` instead. PR 7094 kept the wrapper because
flattening is a second format change that the row did not ask for.

What the wrapper buys today: `FileBody` carries `deny_unknown_fields`,
and that is what makes a file from before PR 7094, whose body still
holds an `"edits"` list beside `"snapshot"`, refuse `Unreadable` and
name `edits`, rather than load with its log silently dropped
(`p2_gauges::an_old_file_refuses_and_a_loose_instance_saves_and_loads`
pins that). A flattened body would refuse every pre-flattening file
on the unknown `snapshot` key instead, through `SnapshotWire`'s own
`deny_unknown_fields`. Both refuse typed; the difference is which name
the refusal carries, and that every committed `.pncad` and the golden
would be re-blessed again.

Done when the wrapper is either flattened, with the committed files and
the golden re-blessed and the old-file pin naming what the refusal now
says, or kept with the reason written beside `FileBody`.

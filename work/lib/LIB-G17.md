---
id: LIB-G17
kind: unit
title: Node::Shell, the shell recipe door
status: dispatched
opened: 2026-08-29
refs: [LIB-TUBE]
branch: lib/g17-shell
---

RECIPE-DOORS unit 3 of 3 (D5): `Node::Shell { target, thickness, open }` with
open faces resolved through the N5 ladder, replacing the teapot's
by-description plane scan. Held behind issue #1202, the kernel ask for a
`ShellNaming` birth channel — without an emitter a shell node mints no
`StableName`s, which is G16's defect one verb over. #1202 is open (kernel
work, not LIB's). Sequenced after LIB-TUBE per the log's G16 MERGED entry.

## Opened (2026-09-06)

The trigger fired: `work/shell/shell-needs-shellnaming-birth-channel.md`
(the tracker file for 1202) closed 2026-09-04 — `shell` / `shell_open`
return `Shelled<T> { body, naming }` with the `ShellNaming` record
written by the doors themselves (`crates/topo/src/shell.rs`), and
SEAT-9 (PR 1995) put `VerbRecord::Shell(ShellNaming)` and
`Verb::run_shell` on the verb seat (`crates/verbs/src/run.rs`). Nothing
gates the unit now but its own dispatch; the "after LIB-TUBE"
sequencing note is discharged (LIB-TUBE closed 2026-09-03). Full
protocol (the 08-29 ruling's substantive class); needs the LIB-13
block draw first. Spec at dispatch: `docs/LIB-G17-SPEC.md`.

## Delivered (2026-09-06, PR pending)

`Node::Shell { target, thickness, open }` with `SlotId::ShellThickness`;
`open` ORDERED, first-occurrence-deduplicated at `Node::shell`, a wire
repeat refused at load (`SnapshotError::ShellOpenRepeated`);
`wire_shell` through `Verb::Shell` behind a per-scalar `ShellLane`
door (a dual has no door and refuses typed); `NodeErrorKind::Shell`
by the total fold of `ShellError<T>` to `f64`, plus `ShellOpenResolve`
/ `ShellOpenKind` / `ShellLaneUnsupported`; `names::emit_shell` with
`RoleSeg::{Inner, Rim, HoleRim}` under `OpGroup::Shell`;
`attach_shell`; content tag 35 via `document_verb_tag`; `Node.shell`
in Python with tags `shell`, `shell_open_resolve`, `shell_open_kind`,
`shell_lane_unsupported`; `corpus/{cup,vessel}.rs` (held beside the
registry — see `shell-corpus-documents-held-out-of-the-registry`),
`lib_g17_shell_node.rs`, `tests/test_shell.py`, the golden's `open`
wire pin, the GUIDE's shell step. Filed here:
`shell-mouth-chart-designated-in-full`, `teapot-scene-through-node-shell`,
`shell-corpus-documents-held-out-of-the-registry`.


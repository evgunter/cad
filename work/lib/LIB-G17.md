---
id: LIB-G17
kind: unit
title: Node::Shell, the shell recipe door
status: open
opened: 2026-08-29
refs: [LIB-TUBE]
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

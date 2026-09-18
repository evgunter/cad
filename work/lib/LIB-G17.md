---
id: LIB-G17
kind: unit
title: Node::Shell, the shell recipe door
status: closed
opened: 2026-08-29
refs: [LIB-TUBE]
branch: lib/g17-shell
pr: 2150
closed: 2026-09-08
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

## Delivered (PR #2150)

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

## Fix pass (PR #2150, both reviews APPROVE-WITH-FIXES, 0 MAJOR)

Recorded here because the spec is deleted at merge:

- **The witness fold lives on the lane.** `ShellLane: Lane`
  (`crates/editor-core/src/lane.rs`): each scalar declares its name
  and how a bracket end reads as `f64`; the fold declares per FIELD
  which end is the honest witness (`BracketEnd`), so `verb_refused`
  carries no bracket bound, `bounds-allowlist.sh`'s `wire.rs` pin is
  back at 15, and the `geom-core` census rows are gone. Pinned on
  non-degenerate brackets (a fold unit row and a document row through
  a widened parameter); the `lo→hi` mutant reds both.
- **The `ShellLane` deviation** stands: the seat's door demands
  certification rights a dual lacks (DL3), so `Dual` refuses typed
  (`ShellLaneUnsupported`).
- **`cup`/`vessel` sit beside the corpus registry** (the `Dual64` row):
  `shell-corpus-documents-held-out-of-the-registry`.
- **The repeat check is `Node::input_fault`'s**
  (`InputFault::RepeatedDesignation`): the insert door and the load
  door refuse alike; `SnapshotError::ShellOpenRepeated` is gone. The
  blends' load-only asymmetry: `blend-selection-canonical-check-load-only`.
- **The GUIDE has no chamfer or tube step**:
  `guide-has-no-chamfer-or-tube-step`.
- **The correspondence generic waits for a third instance**:
  `correspondence-structs-coincide`.
- The reviewers' rows are in the suite as assertions
  (`lib_g17_r1_probes.rs`, `lib_g17_r2_probes.rs`): order swap changes
  only the rim name (both orders, both files; the `sort_unstable`
  mutant reds), rebind shrinks keeping the earlier position, the
  thickness-only edit moves the key and defeats the memo, the thick
  wall refuses `WallClearance` with its numbers, a holed designated
  face mints `HoleRim`, a side wall opens the cup on its side.


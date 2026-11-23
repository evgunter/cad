---
id: profile-readers-read-kernel-steps-through-the-written-address-map
kind: unit
title: Every reader of a profile's kernel loop or step index reads it through one map to the written address, identity today
status: open
opened: 2026-11-22
priority: P1
cost: H
parent: a-loop-cannot-repeat-inside-a-profile
---


Unit 1 of `a-loop-cannot-repeat-inside-a-profile` (Ev, PR 7263). It
adds no vocabulary and changes no behaviour. It exists so that the
block units after it change what the map says, and not the readers.

## Why first

The ruling lowers `each k { … }` to a flat program "where the
document's program becomes the kernel's" (`docs/PATHS-DESIGN.md` §2
*Composition*), so the `profile` crate sees nothing new. Today the
document side reads every index the kernel hands back as if it were the
written one: kernel loop `li` is program loop `li`, and kernel step `i`
is written step `i`. Once a block exists the two differ. This unit
names each such reader, routes it through one map, and keeps the map
the identity. If one reader cannot be routed, that is the riskiest
unknown of the build, and this unit finds it before any vocabulary
lands.

## The address the map keeps

The **written address** keeps the meaning it has today: a loop's index
in the written loop list, and a step's index in its loop's written
steps. A step inside braces has one written address however many
copies it draws. So `SlotId::Profile { loop_, step, arg }`
(`crates/editor-core/src/node.rs`, 92 sites across 31 files, 34 of them
in the viewer), `ProfileProgram::ids` (`Vec<Vec<StepId>>`, one id per
written step), `RecordedNotation`'s `(u32, StepArg)` key,
`LoopProgram::authored_steps`, `step_rows`, `step_args` and
`step_radii` all keep their meaning and their shape. That is the cut's
answer to "the flat `ids`, the authored address and `SlotId::Profile`
against a nested program": none of the three has to change, because
each one addresses what was written, and only a reader of what the
kernel drew needs the map. The block units supply the blocks' half of
that claim: how the written index counts a block's steps.

The **map** is produced by resolution beside the flat steps. For each
kernel loop it gives the written loop and the member (`MemberAt`,
`crates/editor-core/src/names/role.rs`, empty here). For each kernel
step it gives the written step. Its name and type are the implementer's
choice. It is not persisted and not a cache. It is rebuilt with the
resolution, as `profile_edges_of`'s doc says of the records.

## The readers to route (found in the tree on 2026-11-22)

In `crates/editor-core/src/program.rs`:
- `resolve_loops`, `ProfileProgram::resolve` and `resolve_with`, and
  `LoopProgram::resolve_with`. They return the map beside
  `Vec<Vec<Step<T>>>`, or a type holding both.
- `ProfileProgram::drawn`: `ids[p.step]`, where `p.step` is
  `profile::ChainShape`'s kernel step. Also `shape` and `loop_shape`,
  and `structure` and `partition` (and through them
  `ProfilePartition::add_loop`).
- `checked_records`, `CheckedRecords::new`, `span_of` and
  `edges_of_step`, `profile_edges_of`, and `segment_radii`
  (`program_index(emission.step)` treated as written).
- `replay_records`, and the step its kernel faults report:
  `ProgramRefusal::Transition { loop_, step }`, `ProgramRefusal::Order`,
  and `StepSegmentsError`'s `loop_` and `step` fields.
- `pieces`, which hands `&self.ids` to `eval::ProfilePieces::publish`.

Elsewhere:
- `crates/editor-core/src/eval/anchor.rs`: `ProfilePieces::publish`
  (`ids.get(program_loop)`), and the `program_loop` the canonical-loop
  ordering carries.
- `crates/editor-core/src/eval/wire.rs`: `prepare_profile(resolved, ids)`
  and `edge_radii` (`segment_radii(.., anchor.program_loop)`).
- `crates/editor-core/src/eval/mod.rs`: the profile arm's two
  `resolve_with` calls, the per-loop id hash, and the `step_radii` walk
  over `program.loops`.
- `crates/editor-core/src/trace.rs`: `resolve_chain` and `pen_fault`,
  which report a written slot from a kernel index.
- `crates/editor-core/src/coincide.rs`: the reader that takes
  `program.loops.get(loop_)` then `steps[at-2..at]` (around line 1091),
  and the walk over validated loops by index (around line 1740).

## Done when

- Every reader above takes its kernel-to-written answer from the map. A
  grep for `program_index(` applied to a kernel record's index finds
  none outside the map's own constructor. The PR lists every hit and
  its disposition (`docs/prompts/implementer-discipline.md` §5).
- A test hands one reader (`ProfilePieces::publish`, or
  `profile_edges_of`) a non-identity map built in the test: two kernel
  loops from one written loop, and kernel steps 3..6 mapped to written
  steps 0..2. The test asserts the names and spans it answers come from
  the written address. A reader still indexing by kernel position fails
  it.
- Every existing suite is green unchanged, with no golden re-blessed
  and no frame moved. The map is the identity on every program that
  exists.

## Leaves for later

- The member key on a piece's name: `a-profile-piece-name-carries-its-member`.
- Any block: `a-profiles-loop-list-repeats-a-loop-with-each` and after.
- The viewer's own flat reads (`crates/viewer/src/drafts.rs`
  `DoorLoops`, `ProfileEdit`, `session.rs` `carry_unmoved`). They read
  written addresses, so they need nothing until a block exists. Their
  row is on VSEAM.

## Check first

Whether `eval/anchor.rs`'s canonical-loop ordering (`derive_naming`,
`LoopAnchor::program_loop`) can carry a written loop plus member rather
than a kernel loop index, without changing what a loft's section
correspondence pairs. `profile_edges_of`'s doc says the loft pairs
canonical segment k of every section. If the anchor's `program_loop` is
read as a kernel index by the loft or the emitters in a way the map
cannot stand in for, stop and report the reader. Do not give the
anchor a second index.

## Rows and ground

- Closes nothing on its own. It changes no other program's row.
- It crosses DOCTAIL's and SELFREF's `program.rs`, BORDERS' and
  TALLY's `eval/anchor.rs` and `eval/mod.rs`, the five programs that
  claim `eval/wire.rs`, and SELFREF's and TRACE's `trace.rs`. SELFREF
  U1 (`a-profile-point-argument-holds-one-point2-variable`) reshapes
  the same resolve functions; whichever lands second merges the other.
  Announce on selfref's log.
- Review: **single FULL**. It is behaviour-preserving by claim, across
  naming, and only a full read can check that no reader was missed.
  Difficulty **M**: mechanical once found, with the anchor the one hard
  read.

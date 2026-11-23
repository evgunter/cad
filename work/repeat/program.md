---
id: repeat
kind: program
title: REPEAT — a profile repeats part of itself with each k, lowered to a flat program
status: ready
opened: 2026-11-22
group: kernel/sketch
prefix: repeat/
tag: (REPEAT orchestrator)
paths: [crates/editor-core/src/program.rs, crates/editor-core/src/repetition.rs, crates/editor-core/src/step_handle.rs, crates/editor-core/src/eval/anchor.rs, crates/editor-core/src/names/role.rs]
keep_out: [opened 2026-11-22 by PROFILE's cut of the repetition build Ev ruled on PR 7263 - the parent row came from profile by git mv with its id kept, every path above is shared - program.rs is DOCTAIL's and SELFREF's (SELFREF U1 reshapes ProgramStep's point arguments in the same file and is coupled with every unit here - announce on selfref's log), eval/anchor.rs and eval/mod.rs are BORDERS' and TALLY's, names/role.rs is DOCTAIL's and REFS', edit.rs mint.rs and persist/* are DOCTAIL's - run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled, crates/profile/* is PROFILE's and nothing here changes it (the lowering hands the kernel a flat program), the viewer half and the Python half are their owners' rows (vseam and pydoor) parked on the units here, docs/PATHS-DESIGN.md and docs/DESIGN.md are ratified design - a re-word forced by an approved change lands with it and a change to what a clause decides is an [ev] PR]
---

The build Ev ruled on PR 7263 (2026-11-21: "(a) is good! this plan
sounds good"): a profile's program repeats a loop, or a run of one
chain's steps, with `each k { … }` over an ordinary document index, and
the block lowers to a flat program where the document's program becomes
the kernel's. The ruled text is `docs/DESIGN.md` `[repetition]` (one
sentence) and `docs/PATHS-DESIGN.md` §2 *Composition*.

Cut out of PROFILE, which was over its 30-point budget before this
build was priced, along the seam between the lattice (PROFILE: the
kernel `profile` crate, validation, the fillet family) and the document
layer's program representation (this track: `program.rs`'s vocabulary,
its addresses and names, and the lowering). Nothing here changes the
`profile` crate.

Nobody holds it. It opens `ready` because its first two units are
dispatchable.

## Charter

- **The address map** (first): every reader of a kernel loop or step
  index reads it through one map from kernel steps to written ones,
  identity until a block exists.
- **The piece's member**: a profile piece's name gains its member.
- **The loop block, the run block, the ring**, each a complete
  vocabulary when it lands.
- The parent row, `a-loop-cannot-repeat-inside-a-profile`, closes with
  the last of them and the two consumer halves (the viewer's on VSEAM,
  the Python door's on PYDOOR).

## Ratified ground (cited, not re-litigated)

- `docs/DESIGN.md` `[repetition]` and `docs/PATHS-DESIGN.md` §2
  *Composition*, as PR 7263 wrote them.
- PROFILE's list (`work/profile/program.md`) applies unchanged.

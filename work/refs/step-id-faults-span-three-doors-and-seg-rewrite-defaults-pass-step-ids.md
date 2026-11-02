---
id: step-id-faults-span-three-doors-and-seg-rewrite-defaults-pass-step-ids
kind: issue
title: StepIdFault spans three doors with arms only one can raise, and SegRewrite's edge and vertex default to the identity
status: open
opened: 2026-11-02
---


Found by the review of PR #6341 (a loft section named by a minted
`SectionId`), which fixed the same two shapes for section ids and left
the step-id ones they were copied from.

**One fault enum for three doors.** `program::StepIdFault`
(`crates/editor-core/src/program.rs:3047`) is raised by `InsertNode`
(`Preminted` only), by `SetProgram` (`LoopCount`, `Shape`,
`NotThisProfiles`, `Repeated`) and by the load door (`NotMinted`, and
the shape and repeat arms against a save). `EditError::StepIdsRefused`'s
recourse therefore has to answer `NotMinted`, which no edit raises, and
ends it with the kernel-defect ending
(`crates/editor-core/src/edit.rs:3474`). #6341 split the section ids'
faults instead: `node::SectionIdFault` holds only what the edit doors
raise, and the load door has `persist::SectionIdBreak`, whose missing-id
arm is a format break carrying `REGENERATE_RECOURSE`. Doing the same for
step ids moves a published tag (`step_id_fault_tag`, pncad-py), so it is
its own change.

**Identity defaults where an id is carried.** `names::SegRewrite`
(`crates/editor-core/src/names/role.rs:1938`) gives `edge` and `vertex` an
identity default, and a profile edge or vertex locator carries a step id.
A rewriter that moves a name between documents and forgets to override
them passes step ids through unmapped, silently. #6341 made `section`
required for that reason; `edge` and `vertex` have the same hazard. Today
only `Remapping` (`refactor.rs`) and `StepPieces` (`role.rs`) override
them, and both should; making them required costs each of the other
rewriters (`WholeMemberEdges`, `CiteParents`, `Recarry`) two one-line
identities.

**What closing it takes.** Split `StepIdFault` per door the way
`SectionIdFault` / `SectionIdBreak` are, with a tag migration for the
Python façade; and make `SegRewrite::edge` and `vertex` required.

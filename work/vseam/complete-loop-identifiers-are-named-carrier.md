---
id: complete-loop-identifiers-are-named-carrier
kind: issue
title: The complete-loop forms' identifiers say carrier, which a rectangle is not
status: open
opened: 2026-11-02
priority: P3
cost: M
---


## What happens

The complete-loop program forms are `circle`, `circle_split` and, since
PR 6340, `rectangle`. Their shared identifiers and prose still call
them *carrier* forms, which was true while both were arcs of one
carrier and is false of a rectangle's four lines:

- `profile`: `RoleList::Carrier` (`crates/profile/src/structure.rs:420`),
  `ReplayStructure::carrier`, and the prose around them;
- `editor-core`: `RecordedProgramError::CarrierInChain` and the carrier
  prose in `program.rs` (lines 1096, 1185, 1525–1554);
- `pncad-py`: `CarrierPieces` and the role-list prose
  (`src/py/step.rs:13, 50, 193, 221, 291–294`), and the stub
  (`pncad.pyi:1910`).

## Why it matters

The soundness bug PR 6340 fixed came from this conflation: the
coincidence door read "a `Piece(k)` role" as "one arc carrier", which
held only while every form drawing `Piece(k)` was a circle. A
rectangle drawing `Piece(k)` made two walls of a box read as one
carrier and one axis. The repair moved the question onto the loop
(`LoopProgram::carrier_radius`), but the names still invite the next
reader to make the same inference.

## What would close it

A rename across `profile`, `editor-core` and `pncad-py` to a word for
"one step draws the whole loop" (for example `RoleList::Whole`,
`CompleteLoopInChain`, `LoopPieces`), with the prose re-read for the
one-carrier claim, keeping "carrier" only where the claim is about arcs
of one circle (`carrier_radius`, `CIRCLE_PIECES`). The Python names are
public, so the rename is a binding change.

## Price

M: mechanical across three crates, but it touches a public Python name
and the censuses that list it.

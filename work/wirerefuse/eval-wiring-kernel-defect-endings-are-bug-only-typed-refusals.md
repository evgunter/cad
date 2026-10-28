---
id: eval-wiring-kernel-defect-endings-are-bug-only-typed-refusals
kind: issue
title: editor-core: seven eval wiring and profile-anchor refusals end in KERNEL_DEFECT_ENDING for states only a kernel bug reaches
status: open
opened: 2026-10-20
parent: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
priority: P3
cost: M
---

A child of the census in `work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md` (as of main 5df6e6c854); why a typed `KERNEL_DEFECT_ENDING` is wrong in either D2 class is that row's "What" and "Shape of the work".

## Sites

- `crates/editor-core/src/eval/mod.rs` `NodeErrorKind` `Display`,
  `ProfileAnchor` (~2280). Door: evaluating a profile node. **Class 2,
  likely**: validate's canonical form is an exact reindexing of its
  input.
- `eval/mod.rs` `NodeErrorKind` `Display`, `MissingSlot` (~2480).
  Door: every verb node's slot reads (`eval/wire.rs` `need_scalar` and
  its kin). **Class 2, likely**: `Node::slots` and the wire agree.
- `eval/mod.rs` `NodeErrorKind` `Display`, `VerbArity` (~2496). Door:
  the run doors. **Class 2, likely**: the per-verb correspondences and
  the run doors agree.
- `crates/editor-core/src/eval/anchor.rs` `PiecesFault` `Display`,
  `NoRecord` (~190), `NoIds` (~195), `Length` (~205) and `NoStepId`
  (~211). Door: evaluating a profile node (`NodeErrorKind::ProfilePieces`).
  **Class 2, likely**, each: one evaluation of one program builds the
  naming anchor, the replay record and the minted step ids.
- `crates/editor-core/src/eval/wire.rs` `place` (~1038), rendering
  `topo::TransformError::NullScaffold` through `NodeErrorKind::Transform`
  (`eval/mod.rs` `Display`, ~2281, a build). Door: placing a copy's
  bodies. **Class 2, likely**: it places finished feature bodies, so only
  a kernel bug reaches the arm here. This caller is the bug-only reading
  of an arm that PR 5528 made end in the scaffolding lever ("finish that
  edit first") at a build, which is true for a caller's mid-edit body at
  the public `transform_rigid` and blames the user here.

`ProfileRecordMissing` (`eval/mod.rs` ~2289) is HELD (parameters: the
guided lift over a parameter box). It is class 2, likely, and is not
this row's. Holding it is conservative: the record it misses is a replay
record of this program's profile lift, not a variable.

The variants' docs give the reason they are typed: "no panic paths in
this crate". `[failure]` says otherwise, and so does the crate itself:
`editor-core/src` already holds `unreachable!`/`panic!` sites
(`program.rs`, `parse.rs`, `mint.rs`). The sentence goes with the
repair. `work/wire/eval-kernel-bug-refusals-end-in-a-tag` (closed) moved
`MissingSlot` and `VerbArity` onto the constant. That row's direction
is the one the parent row retires.

## Repair shape

- **Class 2**: `unreachable!` or `debug_assert` per `[failure.taxonomy]` rows 4–5, answering row 0 first; the tests that pin the text move to `#[should_panic]`.

Re-check each `likely` or `unsure` argument before acting on it; one that fails moves the site to class 1.

## Moved to WIREREST (2026-10-20)

WIRE measured 69 budget points against 30. This row is P3 or P4, and WIRETAIL already stood at 25 of 30, so it moved by `git mv` (id and body unchanged) to WIRE's second tail. Signed (WIRE orchestrator).

## Re-checked (2026-10-26, tracker sweep)

Five of the seven sites are gone from main: `crates/editor-core/src`
holds no `ProfileAnchor`, no `VerbArity` and no `PiecesFault` (so no
`NoRecord`, `NoIds`, `Length` or `NoStepId`); `Verb<T>` and `Arity`
went with PR 5457's option (a). Two remain:
- `NodeErrorKind::MissingSlot` (`eval/mod.rs:2329`), still ending in
  `geom_core::KERNEL_DEFECT_ENDING`;
- `eval/wire.rs`'s `place` (`:927`), which still maps
  `transform_rigid`'s refusal to `NodeErrorKind::Transform`
  (`eval/mod.rs:2186`, "the transform op refused: …"). It now places a
  copy by the copy's `RigidMotion` (stage 3's placement), so re-read
  which ending a bug-only `NullScaffold` takes there before acting.
`ProfileRecordMissing` is no longer in `eval/mod.rs`. Re-priced in
place: two sites, still M for the `#[should_panic]` moves.

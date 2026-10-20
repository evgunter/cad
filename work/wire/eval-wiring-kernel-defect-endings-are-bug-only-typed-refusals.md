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

(ENCL implementer, from the census in
`work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md`,
on main 5df6e6c854.)

`[failure]` (D2) splits every state in two: one input reaches, which
is a typed refusal whose ending must name the decision's lever, and one
only a kernel bug reaches, which MUST panic ("downgrading one … to a
typed error launders a bug into a supported outcome"). A typed refusal
ending in `geom_core::KERNEL_DEFECT_ENDING` is therefore wrong in either
class. The one exception is a kernel approximation limit, which takes
`[tolerance.recourse]`'s last resort (`KERNEL_LIMIT_RECOURSE` /
`KERNEL_LIMIT_LAST_RESORT`), not this ending.

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

`ProfileRecordMissing` (`eval/mod.rs` ~2289) is HELD (parameters: the
guided lift over a parameter box). It is class 2, likely, and is not
this row's.

The variants' docs give the reason they are typed: "no panic paths in
this crate". `[failure]` says otherwise, and so does the crate itself:
`editor-core/src` already holds `unreachable!`/`panic!` sites
(`program.rs`, `parse.rs`, `mint.rs`). The sentence goes with the
repair. `work/wire/eval-kernel-bug-refusals-end-in-a-tag` (closed) moved
`MissingSlot` and `VerbArity` onto the constant. That row's direction
is the one the parent row retires.

## Repair shape

- **Class 2** (only a kernel bug reaches it): `unreachable!` or
  `debug_assert` per `[failure.taxonomy]` rows 4–5, answering row 0
  first (a type that cannot hold the state is preferred). The tests
  that pin the refusal's text move to `#[should_panic]`, or are deleted
  where they only built the error value by hand.
- **Class 1** (input reaches it): the decision's own lever, written in
  its own words (`[tolerance.recourse]` (i)), and a test that reaches it
  through the door.
- **Class 3** (an approximation limit): the last-resort recourse.

Re-check each `likely` or `unsure` argument before acting on it: one
that fails moves the site to class 1.

---
id: a-profile-piece-name-carries-its-member
kind: unit
title: A profile piece's name carries its member, empty outside a block
status: open
opened: 2026-11-22
priority: P1
cost: M
parent: a-loop-cannot-repeat-inside-a-profile
---


Unit 2 of `a-loop-cannot-repeat-inside-a-profile` (Ev, PR 7263). The
ruling: "A piece's name gains its member." This unit adds the member,
empty everywhere, so the block units fill it in and never reshape the
name.

## What changes

- `ProfileEdgeRef::Piece { step, role }` and its twin
  `ProfileVertexRef::Piece` (`crates/editor-core/src/names/role.rs`)
  gain the member, keyed as `RoleSeg::Member { at: MemberAt, .. }` is
  keyed (`MemberAt = Vec<(VarId, u32)>`, outermost first). It is empty
  outside any block.
- `ProfilePieces::publish` (`crates/editor-core/src/eval/anchor.rs`)
  writes it from the map that
  `profile-readers-read-kernel-steps-through-the-written-address-map`
  produces. If that unit has not landed, it writes empty.
- The wire: serde skips an empty member (`skip_serializing_if`), so no
  saved document changes bytes. The goldens under
  `crates/editor-core/tests/golden/` and `bool13_goldens/`, the
  `tests/corpus` documents and `tests/wire_rv_bytes.rs` stay as they are.
- Every renderer of a piece (`names/words.rs`, through `spoken::step` /
  `StepAt`) says nothing extra for an empty member.

## Size

`ProfileEdgeRef::Piece` is spelled at 70 sites in 35 files (counted
2026-11-22: editor-core `names/role.rs`, `names/merged.rs`,
`names/words.rs`, `names/nest.rs`, `names/emit_topo.rs`,
`eval/anchor.rs`, `coincide/stated.rs`, `refactor.rs`, `step_handle.rs`,
`crates/pncad-py/src/py/step.rs`, and tests). `ProfileEdgeRef` appears
199 times in 51 files and `PieceRun` 82 times. The change is atomic:
every consumer sees the new field in one PR.

## Done when

- A piece name round-trips through the wire with an empty member and
  with a non-empty one, the latter built in a test. An old document's
  piece name loads equal to the new form with an empty member.
- Names, selections, `ProfilePartition::pieces` and the maintenance
  checks in `edit.rs` (around lines 5918 and 6243) compare the member.
  A test shows two pieces that differ only in member are two names.
- Every existing suite is green with no golden re-blessed.

## Check first: `ProfileEdgeRef` is `Copy`

`ProfileEdgeRef` derives `Copy`, and so does `ProfileVertexRef`. A
`MemberAt` is a `Vec`, so adding it as designed removes `Copy` from both
names and from every type that holds one by value. The designers did
not see this. Before writing anything, count the sites that rely on
`Copy` (remove the derive in a scratch build, or have CI count the
errors). If the change grows past this unit's size, or a fix needs a
member key of a different shape (a fixed-size key, or an interned one),
**stop and report**. The key's shape is a design choice that also
decides whether blocks nest, and it is asked as a question in the cut,
not settled here.

## Leaves for later

- Filling the member: `a-profiles-loop-list-repeats-a-loop-with-each`
  and after.
- A selection whose member key reads an index (a bolt mated to hole
  `k`'s wall): `a-selection-cannot-name-a-repeated-piece-by-an-index-expression`.

## Rows and ground

- Changes no other row.
- It crosses DOCTAIL's and REFS' `names/role.rs`, BORDERS' and TALLY's
  `eval/anchor.rs`, and BIND's and PYDOOR's `crates/pncad-py/src/py/step.rs`.
  A piece name is persisted inside selections, so this is persist
  ground: announce on the away channel before landing (PROFILE's
  `keep_out`).
- It can run beside unit 1. They meet at `ProfilePieces::publish`;
  whichever lands second merges.
- Review: **single STYLE**. The change is mechanical once the `Copy`
  question is answered, and the wire test is the substance. Difficulty
  **M**.

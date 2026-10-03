---
id: an-insert-has-a-typed-door-its-minted-id-refusals-predate
kind: issue
title: An insert's id now has a typed door; SceneDocError::NoNodeMinted and the demos' minted-id expects predate it
status: open
opened: 2026-10-03
priority: P4
cost: M
refs: [three-loops-apply-several-edits-and-net-their-maintenance]
---

Found by the sweep for
`three-loops-apply-several-edits-and-net-their-maintenance`.

## What

That unit gave an insert's minted id a type: `apply_insert`
(`crates/editor-core/src/edit.rs`, crate-private) answers the id beside
the `Applied`, and `Recording::insert` is its public face. Every
`unreachable!`/`unwrap_or_else` on "an accepted insert mints its node"
in `regauge_then_mate`, `refactor.rs` and the viewer's
`DocSession::commit_run` callers, and pncad-py's `Doc.insert`
`no_minted_id` raise (`BoundaryEdit::NoMintedId`), went with it. The
declare sugar's own `DeclareError::NoMintedId` went with the
declaration node (declared pairs are a boolean's or union's payload
now, so `declare_all` writes a `SetDeclare` and mints nothing).

Two shapes still read `EditRecord::minted`'s `Option` after an insert
and so carry an arm for a contract violation no caller can reach:

- `crates/viewer/src/scene.rs`'s scene-document insert,
  `SceneDocError::NoNodeMinted` (and its prose rows in
  `crates/viewer/tests/error_display.rs`).
- The single-insert helpers that `.expect` the id: eleven in
  `demos/tour/` (`src/`'s `checks.rs`, `plate.rs`, `chain.rs`,
  `ring.rs`, `diefillet.rs`, `bracket.rs`, `heatsink.rs`, `teapot.rs`,
  `assembly.rs` and `impeller.rs`, and `tests/teapot_document.rs`) plus
  test fixtures in editor-core, pncad and the viewer. The demos are
  the evidence: a user inserting one node has no typed door short of
  a one-edit `Recording`.

## Shape of a fix

A public single-insert door answering `(Applied, RecipeNodeId)` (or
`apply_insert` made public), the two shapes moved onto it, and the
dead refusal arms removed with their words.


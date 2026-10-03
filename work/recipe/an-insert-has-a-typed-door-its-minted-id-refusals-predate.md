---
id: an-insert-has-a-typed-door-its-minted-id-refusals-predate
kind: issue
title: An insert's id now has a typed door; DeclareError::NoMintedId, SceneDocError::NoNodeMinted and the demos' minted-id expects predate it
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
in `regauge_then_mate` and `refactor.rs`, and pncad-py's `Doc.insert`
`no_minted_id` raise (with `BoundaryEdit::Declare`), went with it.

Three shapes still read `EditRecord::minted`'s `Option` after an
insert and so carry an arm for a contract violation no caller can
reach:

- `editor_core::declare_all` (`crates/editor-core/src/names/flush.rs`,
  `DeclareError::NoMintedId`), published to Python as
  `no_minted_id` (`crates/pncad-py/src/tags.rs`'s
  `declare_error_tag`); retiring the arm is a vocabulary change.
- `crates/viewer/src/scene.rs`'s scene-document insert,
  `SceneDocError::NoNodeMinted`.
- The single-insert helpers that `.expect` the id: eleven in
  `demos/tour/` (`src/`'s `checks.rs`, `plate.rs`, `chain.rs`,
  `ring.rs`, `diefillet.rs`, `bracket.rs`, `heatsink.rs`, `teapot.rs`,
  `assembly.rs` and `impeller.rs`, and `tests/teapot_document.rs`) plus
  test fixtures in editor-core, pncad and the viewer. The demos are
  the evidence: a user inserting one node has no typed door short of
  a one-edit `Recording`.

## Shape of a fix

A public single-insert door answering `(Applied, RecipeNodeId)` (or
`apply_insert` made public), the three shapes moved onto it, and the
dead refusal arms removed with their words.


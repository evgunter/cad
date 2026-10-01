---
id: mate-refusals-name-documents-by-hex-id
kind: issue
title: editor-core: MateFault::PosesOfAnotherDocument and LeverRefusal's part arms name documents by hex id in text the viewer draws
status: open
opened: 2026-09-29
priority: P3
cost: E
parent: MSOLVE-11
---


## What

Two mate refusals the feature tree draws verbatim (a `NodeErrorKind::Mate`
row) name a document by its hex id, `DocumentId`'s or `DocRef`'s
`Display` (`crates/editor-core/src/ident.rs`), which reads to a person
the way an arena key does:

- `MateFault::PosesOfAnotherDocument` (`crates/editor-core/src/mate.rs`,
  `impl core::fmt::Display for MateFault`): "the solve is of document
  <32 hex>, not of document <32 hex>".
- `LeverRefusal`'s four part arms, `FaceUnbounded`, `MalformedBody`,
  `NoExtent` and `NoFiniteBound` (`impl core::fmt::Display for
  LeverRefusal`): "instance 6's part <32 hex>@<12 hex> has no faces …".
  The instance already names which part; the id adds nothing a reader
  can use. (`FaceUnbounded` and `MalformedBody` also print the face as
  `{face:?}`, an arena key.)

`test_utils::refusal::arena_key` flags a hex document id; the rows
are admitted by exact row and exact span, with this file named in
each, in `crates/editor-core/tests/refusal_concision_chains.rs`
(`ADMISSIONS`: `Mate/PosesOfAnotherDocument`, `Mate/Unleverable`) and,
where an edit refusal forwards them to the status line,
`crates/viewer/tests/refusal_concision_edits.rs` (`ADMISSIONS`). An
admission whose span its row no longer holds is red.

`lever-refusal-respells-reach-refusal` restructures `LeverRefusal`; the
two want doing together.

## What would close it

The text names the instance (and, for the mispairing, the role of each
document) and leaves the ids to the typed payload. Remove the two
admissions when it lands.

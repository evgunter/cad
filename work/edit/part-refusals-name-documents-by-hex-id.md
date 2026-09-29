---
id: part-refusals-name-documents-by-hex-id
kind: issue
title: editor-core: the reference cycle, the at-rest carried route and two EditError arms name documents and versions by hex id in text the viewer draws
status: open
opened: 2026-09-29
priority: P3
cost: E
---


## What

Refusals the viewer draws verbatim name a document, or a version, by
its hex id (`DocumentId`'s, `DocRef`'s or `ContentPin`'s `Display`,
`crates/editor-core/src/ident.rs`), which reads to a person the way an
arena key does. On EDIT's ground:

- `PartFault::ReferenceCycle` (`crates/editor-core/src/eval/parts.rs`,
  `impl core::fmt::Display for PartFault`) prints the loop as
  `<32 hex>@<12 hex> -> …`. The loop is the diagnosis, so the text
  cannot simply drop it; the viewer knows each id's file name
  (`crates/viewer/src/parts.rs`, `PartFiles`) and the kernel does not.
- `Route` (`crates/editor-core/src/assembly.rs`,
  `impl core::fmt::Display for Route`), which an at-rest finding's
  carried attribution renders as "(carried from document <32 hex>
  through instance 4)".
- `EditError::EvaluationOfAnotherDocument` (`crates/editor-core/src/edit.rs`,
  "the supplied evaluation is of document <32 hex>, not of document
  <32 hex>").
- `EditError::PinUnchanged` (`crates/editor-core/src/edit.rs`, "node 6
  already pins <64 hex>"), the whole pin.
- Not on any roster, found by the same sweep: `crates/editor-core/src/update.rs`'s
  "update: every reference to {id} already pins {pin}" (hex twice, and
  a stage prefix).

`test_utils::refusal::arena_key` flags a hex id since
`edit/part-root-carried-refusal`, and the roster rows are admitted by
exact id with this file named beside them:
`crates/editor-core/tests/refusal_concision_chains.rs` (`FILED_DEBUG`,
`Part/ReferenceCycle`), `crates/editor-core/tests/refusal_concision_at_rest.rs`
(`FILED_HEX_ROUTES`) and `crates/viewer/tests/refusal_concision_edits.rs`
(`FILED_HEX`).

## What would close it

The document named in words a reader has: the typed value keeps the
id, and the drawn line names the file, as the feature tree does for a
part root's failure (`crates/viewer/src/tree.rs`, `carried_lines`).
Either the sentence stops naming the document and the surface adds
the file name, or the kernel sentence takes a namer. A version has no
file name; "the version this reference already pins" is the sentence.
Remove the admissions when it lands.

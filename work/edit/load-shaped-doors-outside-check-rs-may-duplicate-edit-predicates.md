---
id: load-shaped-doors-outside-check-rs-may-duplicate-edit-predicates
kind: issue
title: Load-shaped doors outside persist/check.rs were never swept for edit-door predicates spelled twice
status: open
opened: 2026-09-16
---


The sweep behind `blend-selection-canonical-check-load-only` matched
every `return Err(SnapshotError::…)` in
`crates/editor-core/src/persist/check.rs`'s `validate_document` and
asked, per site, whether the predicate is shared with the edit doors or
hand-written at the load door. It found three hand-copied pairs
(`three-door-predicates-are-hand-copied-not-shared`), but the pattern
sees ONE door: a predicate duplicated between an edit door and a
load-shaped door that does not spell `SnapshotError` inline is
invisible to it. Two such doors exist and were never looked at:

- **`crates/editor-core/src/persist/wire.rs`** — the deserialization
  seat. Its refusals are `serde::de::Error::custom` and its own error
  type, not `SnapshotError`, so nothing it checks was matched. Whether
  it re-decides anything an edit door decides is unmeasured.
- **`crates/editor-core/src/program.rs`'s replay** — a snapshot's edit
  log is replayed through `apply`, so by construction it inherits the
  edit doors rather than mirroring them; what is unmeasured is whether
  replay does any pre- or post-check of its own beside that.

The pattern that would find them, which this row exists to run: for
each door that admits a document or a node from OUTSIDE the edit log
(`persist/wire.rs`, `persist/mod.rs`'s `load`/`save` seats,
`program.rs`'s replay), list every refusal it can produce and ask
whether an edit door produces a refusal over the same predicate. A hit
is a predicate with two homes; the fix each time is one home with the
doors naming the answer in their own vocabulary.

Recorded as a row rather than a sentence in a PR body, because a PR
body is not a slate: the blind spot outlives the unit that disclosed it.

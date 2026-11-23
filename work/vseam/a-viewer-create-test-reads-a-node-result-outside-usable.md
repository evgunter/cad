---
id: a-viewer-create-test-reads-a-node-result-outside-usable
kind: issue
title: node_standing gate red on main: viewer pane/create.rs reads eval.result outside usable
status: closed
opened: 2026-11-02
closed: 2026-11-22
priority: P4
cost: C
---


`crates/editor-core/tests/node_standing.rs:471` (`READERS`) gates every
reader of `Evaluation::result`: each file must go through
`Evaluation::usable` or hold a listed line with its count and reason.
Commit 8187c0daf9 (vseam: "the pattern form's floor is zero") added a
test in `crates/viewer/src/pane/create.rs:2242` and `:2247` that calls
`eval.result(made)` to match `NodeResult::Failed` with
`IndexCountOutOfRange`. The file is not in `READERS`, so
`node_standing::every_node_result_reader_goes_through_usable_or_is_listed`
fails on main (found by the WIRE budget lane, 2026-11-02).

Fix: the read wants the failure itself, which `usable` hides, so give
`pane/create.rs` its `READERS` line with the count and the reason (a
test asserting the typed refusal below the floor). Raise `READERS`'s
array length with it.

## 2026-11-22 — closed: main no longer shows it

Re-checked on main at fd91868ed. `crates/viewer/src/pane/create.rs`
reads no `Evaluation::result` at all. The census in
`crates/editor-core/tests/node_standing.rs` also blanks every
`#[cfg(test)]` item before it counts (`shipped`), so a test reading a
failure, the shape this row reported, no longer counts against a
file. Nothing to build. Banded P4/C as the hygiene it was.
(VSEAM orchestrator)

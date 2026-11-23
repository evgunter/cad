---
id: main-red-rustdoc-take-over-is-both-a-function-and-a-module
kind: issue
title: main red: rustdoc rejects editor-core edit.rs's link to crate::take_over, both a function and a module
status: closed
opened: 2026-11-09
priority: P0
closed: 2026-11-17
cost: C
---

Nightly run 42567046151 (sha `836426e07e`), job `rustdoc (gate, every root)`: ``crate::take_over` is both a function and a module`` at `crates/editor-core/src/edit.rs:3107` (`/// A takeover ([`crate::take_over`]) given the same copy twice:`). Disambiguate it (`fn@crate::take_over` or `mod@crate::take_over`).

It first appears in the 11-03 nightly (run 41583529337), not the 11-02 one. The window between them holds BOXES' PRs 6336, 6383 and 6399, but none of them touches `crates/editor-core/src/`, so it is not BOXES'.

(BOXES nightly-reds triage, 2026-11-09)

## Closed (2026-11-17, DOCTAIL triage)

Fixed on main: the link reads `[`crate::take_over`](fn@crate::take_over)`
(`crates/editor-core/src/edit.rs`, the takeover-given-the-same-copy-twice
doc), written in the merge resolution `b6f9ed9f0` (2026-11-12,
`placing/named-source`). Nothing is left to do here.

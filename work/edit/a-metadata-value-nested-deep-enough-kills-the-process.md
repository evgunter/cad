---
id: a-metadata-value-nested-deep-enough-kills-the-process
kind: issue
title: editor-core: a MetaValue nests without bound through the Rust API, and the edit door's walk over one recurses per level, so a deep one kills the process
status: open
opened: 2026-09-30
priority: P2
cost: E
---

(EDIT, found by the recursion sweep of `edit/expr-nesting-bound`.)

## What

`MetaValue` (`crates/editor-core/src/meta/mod.rs`) is a public tree:
`List(Vec<MetaValue>)` and `Map(BTreeMap<String, MetaValue>)` nest with
no bound, and a caller builds one directly or through `meta::to_value`.
Every walk over it recurses once per level: `MetaValue::first_non_finite`
(the edit door's D2 check on `DocEdit::SetAppearanceMeta`, and
`persist/check.rs`'s `first_non_finite`), the hand-written `PartialEq`,
the derived `Clone`, `Debug`, `Drop` and serde impls, and the content
pin's `serde_json::Value` round.

## Evidence

A scratch probe (dev profile, a thread with the wasm32 build's 1 MiB
stack), applying `DocEdit::SetAppearanceMeta` whose value is a `List`
nested `n` deep inside the `{"v": 1, ...}` map the D7 convention asks
for, to a one-extrude document:

| `n` | outcome |
|---|---|
| 1 000 | applies |
| 10 000 | stack overflow, SIGABRT |

Not reachable from Python (the binding builds no `MetaValue`), so the
door is the Rust API and a hand-edited file. The load door bounds a
file's nesting (`persist::nesting::BODY_NESTING`, 289 levels today),
and a `MetaValue` nested to that limit (about 140 levels, two JSON
levels each) loads on a 1 MiB stack (measured, 339 KiB).

## What would close it

A nesting bound on `MetaValue` checked at the doors that take one
(`SetAppearanceMeta`, `to_value`), refused typed and read by the load
door's limit as the expression bound is; or walks that do not recurse,
`Drop` included.

---
id: a-metadata-value-nested-deep-enough-kills-the-process
kind: issue
title: editor-core: a MetaValue nests without bound through the Rust API, and the edit door's walk over one recurses per level, so a deep one kills the process
status: review
opened: 2026-09-30
priority: P2
cost: E
branch: recipe/meta-bound-and-rule-recourse
pr: 3909
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
file's nesting (`persist::nesting::BODY_NESTING`, 271 JSON brackets
today), and a `MetaValue` nested to that limit (about 131 levels, two
JSON brackets each) loads on a 1 MiB stack (measured at the earlier
limit of 289, about 140 levels: 339 KiB).

The scan is type-agnostic (the review of PR 3510, its Q7): one limit
for every value in a body, so the deepest type's budget is every
type's. Raising `BODY_NESTING` for another type (names nested deeper,
`a-stable-name-nests-one-level-per-copy-and-every-walk-over-it-recurses`)
raises how deep a `MetaValue` this recursive load reads, so a bound
here, or a load that counts its own levels as the expression reader
does, should land before any such raise.

## What would close it

A nesting bound on `MetaValue` checked at the doors that take one
(`SetAppearanceMeta`, `to_value`), refused typed and read by the load
door's limit as the expression bound is; or walks that do not recurse,
`Drop` included.

## Built (2026-10-02, PR 3909)

The type holds the bound: a value nests at most `meta::MAX_NESTING` (128) levels, and a list or a map past it cannot be built (`MetaValue::list`/`map`, `to_value` and the deserializer refuse typed). `to_value` also bounds how deep it reads a producer (`meta::MAX_PRODUCER_NESTING`, options and newtypes counted), so a deep or self-referential producer is refused before the stack runs out; the deserializer refuses a list or map past the bound before reading into it. The load door's limit reads the bound (`persist::nesting::META_BODY_NESTING`). Pinned in `crates/editor-core/tests/meta_nesting_bound.rs` on the 1 MiB stack. The design, the sweep and the mutants are in the PR body.

Filed: `the-load-doors-refusals-are-held-to-no-shape-guard`.

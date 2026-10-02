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

The structural closure: **a value nests at most `meta::MAX_NESTING` (128) levels, and the type holds it.**
- `MetaValue::List` and `Map` carry a `meta::Nested<C>` (`MetaList`, `MetaMap`) whose fields are private and which caches its nesting in a byte. A list or a map is built only by `MetaValue::list` / `MetaValue::map`, which refuse past the bound with `MetaError::NestedTooDeep { bound }` ("… Recourse: store it flatter, keeping a deep part as a string or as bytes"), by `to_value`, or by the deserializer. Children are read through `Deref`; there is no `DerefMut`, so a held value cannot be deepened in place.
- `to_value` counts levels top-down (`ValueSer { level }`) and refuses the child past the bound before reading it, so a producer ten thousand levels deep is stopped at level 129, not walked.
- The deserializer refuses with the problem alone; the load door states its own recourse.
- The load door reads the bound: `persist::nesting::META_BODY_NESTING` (6 brackets of envelope + 2 per level = 262) is folded into `BODY_NESTING` as a max with the expression's 270, so the limit is unchanged today and follows a raise of either bound.
- So no door needs a runtime check: `SetAppearanceMeta` cannot be handed a value deeper than the bound, and every recursive walk (`first_non_finite`, `PartialEq`, the derived `Clone`, `Debug`, `Drop`, serde, the content pin's round) is bounded by it.

Pinned in `crates/editor-core/tests/meta_nesting_bound.rs`, every row on the 1 MiB stack: the row's evidence shape (a `List` folded 10 000 deep, and a `to_value` producer 10 000 deep) refuses typed at the bound; a value at the bound walks, clones, compares, prints, round-trips as a producer, applies through `SetAppearanceMeta`, pins, saves both ways, loads back equal, and its deepest save measures `META_BODY_NESTING`; a hand-edited file one past refuses `Unreadable` naming the bound, and 10 000 past refuses at the scan. A planted mutant (the bound check removed from `Nested::over`) reds two of the three rows.

Filed: `the-load-doors-refusals-are-held-to-no-shape-guard`.

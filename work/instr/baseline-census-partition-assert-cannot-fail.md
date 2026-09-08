---
id: baseline-census-partition-assert-cannot-fail
kind: issue
title: baseline_census's constant-plus-discriminating sum is a fact about the two filters, not about the corpus
status: open
opened: 2026-09-08
---


## Finding

`tools/tess-lint/tests/baseline_census.rs:228-233`:

```rust
assert_eq!(
    constant.len() + discriminating.len(),
    IDENTITY_COLUMNS.len(),
    "every identity entry is either constant or discriminating"
);
```

`constant` is `IDENTITY_COLUMNS` filtered on `distinct == 1` and
`discriminating` on `distinct > 1` (`:202-213`). `distinct` is a
`dedup`ed length over the sized rows, and `:187` already refuses an
empty `sized`, so every entry is at least 1 and the two filters
partition the list **by construction**. The sum is
`IDENTITY_COLUMNS.len()` for every possible corpus, so the assertion
cannot fail for the reason it states.

It is subsumed twice over besides: `:216` and `:223` assert `constant`
and `discriminating` against their exact five- and two-element lists,
so a column changing side, or an eighth column, reds there first with
a message naming the column.

## Class

Same shape as `tools/tess-lint/tests/cut_line_pin.rs`'s
`TABLE.iter().any(|r| r.1) && TABLE.iter().any(|r| !r.1)`, deleted in
PR 2151, and as the `found.len() >= 20` that
`crates/test-utils/tests/reader_census.rs:548-556` records deleting:
a predicate over things that cannot vary at runtime, standing beside
the assertions that already subsume it.

The sweep behind this item covered every `assert` in `tools/*/tests`
and `tools/*/src` whose operands are `const` items or are derived from
them alone. **What it could not match**: assertions whose operands are
derived through a helper the grep did not follow, and the same shape
written as an `expect`/`unwrap` message rather than an assert.

## Fix

Delete it, or replace it with the thing it was reaching for — that
`distinct` has no zero entry — which is a statement about the corpus
and can fail. The comment above it ("so the prose cannot drift from
it") names a real want; the assertion as written does not serve it,
because the prose it guards is the five/two split and that is what the
two literal lists above already pin.

## Was

Found by the METER unit-4 fix pass (`meter/cut-prefix-pin`, 2026-09-08)
sweeping `tools/` for the class after the reviewer named it on that
unit's own instance. Not fixed there: `baseline_census.rs` is outside
unit 4's diff and the deletion is its owner's judgement to make, the
file having stated an argument for the assertion.

## Moved to INSTR (2026-09-08)

Moved from `work/meter/` to `work/instr/` by `git mv` when METER's exit
walk opened the successor (`docs/METER-EXIT-WALK.md` §4, ratified by Ev on
2026-09-08 at PR #2212). Id, header and body unchanged; the directory is
the claim. This row is one of the twenty on INSTR's opening slate.

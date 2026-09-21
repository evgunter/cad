---
id: baseline-census-partition-assert-cannot-fail
kind: issue
title: baseline_census's constant-plus-discriminating sum is a fact about the two filters, not about the corpus
status: closed
opened: 2026-09-08
closed: 2026-09-16
---


## Finding

`tools/tess-lint/tests/baseline_census.rs`, in
`five_of_the_seven_identity_entries_discriminate_nothing_among_the_sized_rows`:

```rust
assert_eq!(
    constant.len() + discriminating.len(),
    IDENTITY_COLUMNS.len(),
    "every identity entry is either constant or discriminating"
);
```

`constant` is `IDENTITY_COLUMNS` filtered on `**d == 1` and
`discriminating` on `**d > 1`, over the same `distinct` vector.
`distinct` is a `dedup`ed length over the sized rows, and the same
test's `assert!(!sized.is_empty(), "the census needs sized rows to be
over")` already refuses an empty `sized`, so every entry is at least 1
and the two filters partition the list **by construction**. The sum is
`IDENTITY_COLUMNS.len()` for every possible corpus, so the assertion
cannot fail for the reason it states.

It is subsumed twice over besides: the two assertions above it pin
`constant` against `["chart", "u0", "u1", "v0", "v1"]` and
`discriminating` against `["nu", "nv"]`, so a column changing side, or
an eighth column, reds there first with a message naming the column.

## Class

Same shape as `tools/tess-lint/tests/cut_line_pin.rs`'s
`TABLE.iter().any(|r| r.1) && TABLE.iter().any(|r| !r.1)`, deleted in
PR 2151, and as the `found.len() >= 20` that
`crates/test-utils/tests/reader_census.rs` records deleting in
`every_site_that_reads_rust_source_is_in_the_ledger`'s doc — *"which
set equality had already subsumed and which could not fail for the
reason it stated"*: a predicate over things that cannot vary at
runtime, standing beside the assertions that already subsume it.

**The precedent is that PAIR, not every `TABLE.iter().any` in
`cut_line_pin.rs`.** A different one survives there today, in
`the_committed_baselines_own_cut_line_is_a_row_of_the_table`, and is
not a residue: it reads `BASELINE`'s first line at runtime and asks
whether the table admits it, so a re-cut of the committed baseline
reds it. It is a claim about the baseline, not about this file's own
source text.

The sweep behind this item covered every `assert` in `tools/*/tests`
and `tools/*/src` whose operands are `const` items or are derived from
them alone. **What it could not match**: assertions whose operands are
derived through a helper the grep did not follow, and the same shape
written as an `expect`/`unwrap` message rather than an assert.

**And it could not match this item's own instance.** `constant` and
`discriminating` descend from `parse(BASELINE)`, so they are
runtime-derived; the defect is that the predicate is forced by how
its own operands were built, not that its operands are fixed at
compile time. The pattern that finds it is the construction-forced
one — a sum of complementary filters over one source — which is what
the re-sweep on INSTR unit 4 used. The stated pattern above is a real
pattern with real hits; it is simply not the one this row is an
instance of.

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

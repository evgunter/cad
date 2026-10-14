---
id: a-names-alike-bound-sits-at-the-debug-builds-own-time
kind: issue
title: a_large_table_of_names_alike_at_no_citation_is_said_in_bounded_time's 10 s bound sits at a debug build's own time
status: open
opened: 2026-10-08
priority: P4
cost: E
---


## What

`crates/editor-core/tests/name_words_rows.rs`,
`a_large_table_of_names_alike_at_no_citation_is_said_in_bounded_time`,
asserts `first_took.as_secs() < 10`. Its own comment, and the recipe
row that landed the fix, calibrate that bound against a release build:
1.3 s after the fix, 15.5 s before it. The test is in the `ci`
profile's slow set, so `cargo nextest run --workspace` runs it in a
debug build. On a 4-core cloud box (JOIN lane, 2026-10-08, main at
047d10d5 and a JOIN branch that leaves editor-core alone), the first
saying read:
- main: 9.30 s and 9.79 s, and a pass at 10.00 s total;
- branch: 9.61 s, 9.99 s, 11.02 s and 11.56 s.

Two of those six runs failed. A debug build spends its whole budget,
so the bound tells the regression it guards (15 s in release) apart
from ordinary debug cost by chance alone.

## Fix

Give the bound a margin on the build actually running it. For example,
scale the bound under `cfg(debug_assertions)`, or gate the timing
assertion to release, so that it still separates 1.3 s from 15.5 s.

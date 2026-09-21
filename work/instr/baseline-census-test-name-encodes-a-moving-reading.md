---
id: baseline-census-test-name-encodes-a-moving-reading
kind: issue
title: A #[test] name states a census reading a re-cut moves, and two sites quote the name
status: open
opened: 2026-09-16
priority: P3
cost: D
---



## Finding

`tools/tess-lint/tests/baseline_census.rs`'s test is named

```
five_of_the_seven_identity_entries_discriminate_nothing_among_the_sized_rows
```

**The name states a reading of the committed baseline**, which is the
one thing this file's own module docs say must not be transcribed:
*"a number transcribed into prose is a number nothing can check, so
both are counted here rather than written down"*. A re-cut that moved
a column from the constant list to the discriminating one would red
the test's two assertions — correctly, and with a message naming the
column — and leave the name saying five.

The file elsewhere prefers names that state the SHAPE rather than the
reading: `the_committed_baseline_gates_a_re_key_in_exactly_these_scenes`,
`the_committed_baseline_carries_this_many_indistinguishable_pairs`.
Either spelling would do here.

## Why this is not a drive-by rename

The name is quoted outside the file, so renaming is a coupled change:

- `tools/tess-lint/src/lib.rs`'s module docs point at the split this
  test derives, in the paragraph beginning *"The SHAPE of the hole"*.
- `work/instr/C15.md` restates the same split in `S73`.

Both want re-checking in the same diff as the rename, which is why
this is a row rather than a fix inside unit 4's fence.

## Was

Disclosed by INSTR unit 4's style review (`instr/u4-census-partition-assert`).

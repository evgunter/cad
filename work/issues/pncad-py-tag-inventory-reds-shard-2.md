---
id: pncad-py-tag-inventory-reds-shard-2
kind: issue
title: pncad-py tag-inventory guard reds shard 2/2 on every code-tier branch
status: open
opened: 2026-09-04
---

Filed by the CIW `render-lanes-red-at-missing-merge-ref` lane, from its own
gate run (PR 1724). Not that unit's subject; recorded so it has a durable
home. Sibling lanes are hitting the same red right now, so a duplicate can
be closed against this file rather than the other way round.

## The fact

`pncad-py tests::the_whole_tag_table_matches_its_committed_inventory` FAILS
in the `test (…, 2/2)` shard on the current tree. `nextest` exits 100 and
the job reds; every other test in the shard passes.

Observed on three unrelated branches within one hour on 2026-09-04, each
having drawn a different configuration point:

| run | branch | job |
| --- | --- | --- |
| 33822327502 | `ciw/render-lane-merge-ref` | `test (interval, eps = 1e-12, 2/2)` |
| 33822152938 | `ciw/one-pin-reader` | `test (interval, eps = 1e-6, 2/2)` |
| 33822129170 | `ciw/perf-host-identity` | `test (eps = 1e-12, 2/2)` |

Earlier the same evening, on other programs' branches: 33820183319
(`test (interval, eps = 1e-6, 2/2)`), 33814096354 (`test (eps = 1e-6, 2/2)`),
33812074765 (`test (interval, eps = default, 2/2)`), 33804838111 and
33803928081 (BOTH `2/2` jobs of the run, default and interval).

## What that pattern says

**Neither axis selects it.** It fires at `default` and at `interval`, and at
all three tolerance rows. It fires only in shard `2/2` — `1/2` is green in
every one of those runs — which is where this test lands, not a property of
the shard.

**It is not about any of those branches' diffs.** The three CIW branches
above change `.github/workflows/`, `scripts/` and the perf emitters between
them; none touches `crates/`.

## Why it should not be eps- or lane-sensitive at all

The guard is a source reader, not a geometry test
(`crates/pncad-py/src/tests.rs:2516`): it reads this crate's own
`src/tags.rs` through `test_utils::source::crate_dir`, parses the tag table
out of it, and compares that against an inventory committed beside it.
`crates/pncad-py/src/tags.rs:42` states the contract — "a rename, an
addition, a deletion or a new tag function reds on the default
no-interpreter row".

## What is NOT established

This lane did not read the assertion text: the failure output sits mid-log
in a 3,500-line job and the lane's own subject was elsewhere. So the issue
asserts the failure and its independence from the sampled axes, and asserts
nothing about the cause. Any of the job ids above reproduces the message.

Two candidates a reader should separate rather than assume:

- the inventory in `src/tests.rs` and the table in `src/tags.rs` genuinely
  parted on a recent merge (the last commits to touch either are `b48e0c1d`,
  "the tag guard reads through the shared lexer", and `434964df`), or
- the reader's *input* moved rather than its subject — `test_utils::source`
  gained a new home for the aggregation guard at `eb93731b`, and this test
  reaches the tree through that module.

## Why it is not visible on `main`

`main`'s push runs classify against the previous main tip, so a merge whose
diff is docs- or tracker-only never runs this shard. The recent green main
runs are not evidence that the tree is green here.

---
id: main-red-reader-census-lists-the-deleted-given-pair-test
kind: issue
title: Main red: reader_census still lists crates/topo/tests/a_given_pair_is_made_from_a_kernel_verdict.rs, which PR 5772 deleted
status: closed
closed: 2026-10-27
opened: 2026-10-27
priority: P1
cost: E
refs: [5772]
---

Filed by QUAD's main-red triage lane (PR 5824).

## What

`test-utils` `reader_census` (`crates/test-utils/tests/reader_census.rs`) is red on main `3a0cd80f`+ at the default ε:
`cargo nextest run -p test-utils -E 'test(reader_census::)'`. The ledger still carries an
`Entry { path: "crates/topo/tests/a_given_pair_is_made_from_a_kernel_verdict.rs", disposition: Shared }`
(`reader_census.rs` :432), and reading it fails:

    reading crates/topo/tests/a_given_pair_is_made_from_a_kernel_verdict.rs: No such file or directory (os error 2)

**Cause:** `3b0a7a6e`, the merge of **PR 5772** (`flush/nary-union-door`), deleted that file (in `666693e2`). Its first parent `3b0a7a6e^1` holds the file; the merge does not. The census reads source at run time, so the file's presence decides the row.

The same census is also red on a second, unrelated entry, `crates/topo/src/link.rs` (ATREST's PR 5773): `work/atrest/main-red-reader-census-misses-topo-link-rs.md`.

## Done when

The stale entry is dropped (or moved to wherever the given pair's rows now live, if they moved with a `Shared` read), and `reader_census` is green on main.

## Closed (2026-10-27, by the ATREST orchestrator)

The same `reader_census` row was red on ATREST's `link.rs` too. Both were fixed in one PR: the dead entry for the deleted `crates/topo/tests/a_given_pair_is_made_from_a_kernel_verdict.rs` is removed from the ledger, and `reader_census::` is 7/7.

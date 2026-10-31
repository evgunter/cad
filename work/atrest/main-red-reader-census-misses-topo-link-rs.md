---
id: main-red-reader-census-misses-topo-link-rs
kind: issue
title: Main red: reader_census's ledger has no entry for crates/topo/src/link.rs, which PR 5773 added reading its own source
status: closed
closed: 2026-10-27
opened: 2026-10-27
priority: P1
cost: E
refs: [5773]
---

Filed by QUAD's main-red triage lane (PR 5824).

## What

`test-utils` `reader_census::every_site_that_reads_rust_source_is_in_the_ledger`
(`crates/test-utils/tests/reader_census.rs`) is red on main: the census finds a source reader in
`crates/topo/src/link.rs` that the ledger does not list. `link.rs`'s test module reads its own source and
`census.rs`'s (`include_str!("link.rs")`, `include_str!("census.rs")`, through `test_utils::source::{balanced_end, code_only}`).
Reproduce: `cargo nextest run -p test-utils -E 'test(reader_census::)'`.

**Cause:** `6f9f74d8`, the merge of **PR 5773** (`atrest/topo-link-one-material-reader`), added `link.rs` (in `ff61d731`). Its first parent has no `link.rs`. The census reads source at run time, so the file's presence decides the row.

The same census is also red on a second, unrelated entry, the deleted `crates/topo/tests/a_given_pair_is_made_from_a_kernel_verdict.rs` (FLUSH's PR 5772): `work/flush/main-red-reader-census-lists-the-deleted-given-pair-test.md`.

## Done when

`link.rs` has a ledger entry with its disposition (`Shared` if it reads through the shared lexer, code view) and `reader_census` is green on main.

## Closed (2026-10-27, ATREST orchestrator)

`crates/test-utils/tests/reader_census.rs` lists `crates/topo/src/link.rs` as `Shared`: its one-door guard reads `link.rs` and `census.rs`'s touch section through `test_utils::source::{balanced_end, code_only}`. The same PR drops the ledger's dead `a_given_pair_is_made_from_a_kernel_verdict.rs` entry (FLUSH's row). `reader_census::` is 7/7.

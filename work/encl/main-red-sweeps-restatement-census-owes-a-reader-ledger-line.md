---
id: main-red-sweeps-restatement-census-owes-a-reader-ledger-line
kind: issue
title: Main red: sweep's restatement census reads source and owes a reader-ledger line
status: closed
closed: 2026-10-21
pr: 5533
branch: encl/main-red-restate
opened: 2026-10-20
priority: P1
---


**Main red.** Filed by CONTACTHOLD's nightly-reds lane from the 2026-10-20 nightly (run 39375288176, head `37d8dfdecf`), red at every eps row; the 2026-10-19 nightly (head `07a57312bf`) was green on it.

`crates/test-utils/tests/reader_census.rs` `every_site_that_reads_rust_source_is_in_the_ledger`:

```
the source-reader ledger no longer matches the tree.
  arrived or moved (owe a ledger line): [
    "crates/sweep/src/lib.rs",
]
  listed but no longer reading source (delete the line): []
```

**Culprit.** `302b6705e8` ("sweep restates stored edges through topo's public restatement door"), merged with ENCL's PR 5418 (`encl/sweep-restate`) — the same commit `main-red-restate-stored-edge-is-off-the-tier1-mutation-roster` names. It adds `sweep`'s `restatement_census` test module to `crates/sweep/src/lib.rs`, which reads every source file of the crate with `std::fs::read_to_string` and blanks it through `test_utils::source::code_only`. The reader is the shared lexer, so the honest disposition is a ledger line saying so; no reader needs rewriting. Still red on `cd4243bcfd`.

**Done when** the ledger carries `crates/sweep/src/lib.rs` with its view and the row is green on main.

## Closed

2026-10-21. PR 5533. The ledger in `crates/test-utils/tests/reader_census.rs` now carries `crates/sweep/src/lib.rs` as `Shared` ("caller's re-description door call census, code view"). `restatement_census::callers_doors_certify_only_computed_carriers` reads `sweep/src` through `test_utils::source::code_only` and the shared boundary helpers, so no reader needed rewriting. `every_shared_entry_actually_reaches_the_shared_lexer` confirms the disposition, and the census is green at default eps. `main-red-reader-census-lacks-sweep-lib-after-restate-review` was a duplicate of this row and is closed as one.

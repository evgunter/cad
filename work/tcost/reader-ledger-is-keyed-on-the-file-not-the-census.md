---
id: reader-ledger-is-keyed-on-the-file-not-the-census
kind: issue
title: every_site_that_reads_rust_source_is_in_the_ledger keys on the reader FILE, so a second census inside an already-listed file arrives invisibly
status: open
opened: 2026-09-12
priority: P3
cost: D
---



## Finding

Found by the delta review of PR 2480 (NOTE 6), and filed here because
`crates/test-utils/tests/reader_census.rs` is TCOST's and TINT's ground.
PR 2480 is the first instance.

`every_site_that_reads_rust_source_is_in_the_ledger` compares the set of
FILES that read Rust source against `LEDGER`'s paths. That catches a new
reader in a new file, which is the case it was built for. It cannot see
a **second** source census added inside a file the ledger already lists:
the file is present, the set comparison is satisfied, and the new reader
arrives with no ledger line, no disposition and no review of whether it
should be using the shared lexer.

PR 2480 hit this exactly. Its operand-door census was first written
inside `crates/editor-core/src/eval/mod.rs` — already listed, disposition
`Shared`, comment *"node-kind vocabulary census, code view"* — and the
ledger row stayed green over a second census the comment did not
describe. (That PR then moved the census to
`crates/editor-core/tests/wire_operand_door.rs`, a new file, which DID
red the row until its entry was added. The hole is still there; the PR
only stopped standing in it.)

## Why the disposition enum does not cover it

`Shared` / `Home` / `Unconverted` / `NotRust` are per-FILE answers. A
file holding one shared-lexer census and one hand-rolled reader has no
honest disposition in that vocabulary, and the ledger would carry
whichever the first census earned.

## What a taker owes

A decision about the ledger's key. Options the reviewer named or that
follow from it: key on the reading SITE (a function, or a `#[test]`)
rather than the file; or keep the file key and add a per-file COUNT of
readers, so a second one has to be declared. Either way the entry's
trailing comment stops being the only record of what a listed file
actually reads.

The neighbouring rows in that file — `every_ledger_entry_is_honest`, the
outstanding-`Unconverted` floor — are keyed the same way and inherit the
same blind spot.

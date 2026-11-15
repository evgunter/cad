---
id: the-ring-promotion-posture-row-reads-euler-kill-rs-by-hand
kind: issue
title: staleness_posture.rs's ring-promotion posture row reads euler_kill.rs as raw lines with a hand-rolled comment skip, outside the shared lexer
status: open
opened: 2026-11-15
priority: P4
cost: E
---


Filed by GREEN's `green/nightly-1115` lane while greening
`test-utils::all reader_census::every_site_that_reads_rust_source_is_in_the_ledger`
(red on the nightlies of 2026-11-14 and 11-15).

`crates/topo/src/pcurves/staleness_posture.rs`,
`every_ring_promotion_door_declares_its_posture`: it takes
`include_str!("../euler_kill.rs")`, walks it line by line, and drops a
line whose trimmed text `starts_with("//")`. That is a hand-rolled Rust
reader: a `//` inside a string literal, a block comment, or a door named
in a doc comment's code span is read as the wrong thing, which is the
class `test_utils::source` exists to close
(`crates/test-utils/tests/reader_census.rs`, the module header).

The row was written in `pcurves.rs` (PR 6900) and moved to its own file
by PR 6939 (`onedoor/pcurves-split`, merge `2627b5737`), where the
reader census first names it. The ledger now lists it `Unconverted`,
citing this row, and `UNCONVERTED_TODAY` stays at 4 only because
`crates/topo/src/boxes.rs` stopped reading source in the same two days
(PR 6957). So the count did not fall when it should have.

**Owed.** Read `euler_kill.rs` through `test_utils::source::code_only`
(or `fn_items`, which the sibling `lift_agreement_rows.rs` already uses),
change the ledger line to `Shared`, and lower `UNCONVERTED_TODAY` to 3.

Beside it, not the same defect: `lift_agreement_rows.rs` walks the
kernel crates with its own `read_dir` recursion where
`test_utils::source::rust_sources` is the shared walk. It lexes through
the shared reader, so its ledger line is `Shared`.

Reproduce: `cargo nextest run -p test-utils -E 'test(/reader_census/)'`.

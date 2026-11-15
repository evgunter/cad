---
id: the-ring-promotion-posture-row-reads-euler-kill-rs-by-hand
kind: issue
title: Two pcurves test files read Rust source as raw lines with a hand-rolled comment skip - staleness_posture.rs over euler_kill.rs, lift_agreement_rows.rs's maker census
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
reader census first names it.

**A second one beside it** (found by PR 6985's review):
`crates/topo/src/pcurves/lift_agreement_rows.rs`, the maker census,
counts `JointElement(` over
`text.lines().filter(|l| !l.trim_start().starts_with("//"))`, the same
prefix skip, in the same test that reads `pcurves.rs` and
`pcurves/decision.rs` through `test_utils::source::code_only` and
`fn_items`. It also walks the kernel crates with its own `read_dir`
recursion where `test_utils::source::rust_sources` is the shared walk.

The ledger lists both files `Unconverted`, citing this row, and
`UNCONVERTED_TODAY` is 5: `crates/topo/src/boxes.rs` stopped reading
source (PR 6957) and these two arrived. This row is what licenses the
count going up.

**Owed.** Read `euler_kill.rs` and count the makers through
`test_utils::source::code_only` (or `fn_items`), change both ledger lines
to `Shared`, and lower `UNCONVERTED_TODAY` to 3.

Reproduce: `cargo nextest run -p test-utils -E 'test(/reader_census/)'`.

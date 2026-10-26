---
id: compensated-blocks-oracle-shifts-an-i128-past-its-width
kind: issue
title: compensated's blocks row overflows its own exact-integer oracle (an i128 shifted past 127 bits) at a seed the fuzz reaches
status: closed
closed: 2026-10-26
opened: 2026-10-17
priority: P3
cost: E
---

Found by PCERT while validating PR 5225 (`pcert/azimuth-placement`); the
row and its code are untouched by that PR.

## What fails

`crates/geom-brep/src/ssi/compensated.rs`,
`tests::the_blocks_enclose_the_exact_results`, at fuzz seed
`0x1d82f6b2fbe0d4e6` (`CAD_FUZZ_SEED=0x1d82f6b2fbe0d4e6 cargo nextest
run -p geom-brep -E 'test(the_blocks_enclose_the_exact_results)'`,
reproduced):

    panicked at crates/geom-brep/src/ssi/compensated.rs:294:42:
    attempt to shift left with overflow

The panic is in the row's exact-arithmetic oracle, not in the code it
checks: the division check's `check` closure writes an enclosure's end
as `m·2^e` and compares `end·b` with `a` in `i128`, shifting `a` left by
`−e` (`ai << -e`) or `m·b` left by `e`. A quotient end small enough has
`−e ≥ 128` (or a large one `e` past the headroom `m·b` leaves), and the
shift overflows: a debug panic, and in release a wrapped shift the
oracle would read as a verdict. The fuzz draws such an end at that seed.

## The fix

The oracle compares in a wider exact type (a big integer, or an
exponent-aligned comparison that never shifts past the width), or
bounds the draw so every end's exponent fits; and a row pins the seed.

## Closed (2026-10-26, tracker sweep)

**Fixed on main, and a duplicate** of
`compensated-blocks-check-shifts-past-i128` (the same test, the same
`ai << -e` panic in the oracle). The oracle now compares in
`num_bigint::BigInt` (`crates/geom-brep/src/ssi/compensated.rs` ~:296-309
at `bb25fa4f`), the first fix this row offered. Evidence is on that row's
Closed section.

---
id: f64-exact-decomposition-has-four-test-homes
kind: issue
title: geom-core tests hand-roll the f64-to-exact decomposition four times, with no shared home
status: open
opened: 2026-09-29
priority: P3
cost: E
---

Found by the CERT-DIFF fix pass (PR #3451, review of `99bc87d9c6`).

**Where** — each reads a finite `f64`'s bit fields into an exact value
(`±m·2^e`, or a count of `2^-1074` units), the same field arithmetic
(`(bits >> 52) & 0x7ff`, the subnormal arm at `-1074`, the hidden bit
`1 << 52`), written out by hand:

| Site | Shape |
|---|---|
| `crates/geom-core/tests/interval_exact_fuzz.rs`, `decomp` | `(negative, odd mantissa as u128, exponent)` |
| `crates/geom-core/tests/review_m5_pr2_scratch.rs`, `split` | the same triple, zero as `m = 0` |
| `crates/geom-core/tests/review_m5_pr2_scratch_hull.rs`, inline in `cmp_bound_vs_ratio` | an `i128` mantissa and exponent, signed |
| `crates/geom-core/tests/certification_door_differential.rs`, `exact` | a `BigInt` count of `2^-1074` units |

There is no shared home: no helper module under `crates/geom-core/tests/` holds one, and
the kernel's own exact reading, `sym::rational::Rat::of_f64`
(`crates/geom-core/src/sym/rational.rs`), is `pub(super)`.

**Fix** — one decomposition in the tests' shared helpers (or a public
exact reading, if one earns its place in the API), and the four read
through it. The four differ in the integer they return (`u128`, `i128`,
`BigInt`), so the shared shape is the bit-field split itself; each
caller keeps its own widening.

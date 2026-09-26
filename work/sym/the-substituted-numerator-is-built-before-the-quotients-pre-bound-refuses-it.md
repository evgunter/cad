---
id: the-substituted-numerator-is-built-before-the-quotients-pre-bound-refuses-it
kind: issue
title: a per-node reduction builds the substituted numerator before the quotient's term pre-bound refuses it: 100 first-time refusals, 33.7 s of the pad's dev leaf
status: open
opened: 2026-09-25
priority: P3
refs: [rule-g-is-the-link-and-pads-leaf-cost, DECIDE-7]
---


## What was measured (DECIDE-7)

`geom_core::sym::algebra`'s `apply` substitutes the square in the
numerator (`poly_subst_square`), then in the denominator, and only then
forms the quotient's product (`num.mul(&den.recip()?)`). `Poly::mul`'s
term pre-bound refuses that product when `|A|·|D^h|` is past
`max_terms`, where `A` is the substituted numerator. Every term of `A`
has been multiplied out and summed by then.

This was measured on the pad's dev leaf (`2.4990e3·ε`) with the
per-session reduction memo in place, using
`decide_7_rule_g_cost_interval::decide_7_where_rule_gs_time_goes`
profiled under the shipped set (the `refused:` rows of
`SymProfile::reduce_refusals`). 100 first-time refusals at the
quotient's product on the term pre-bound take 33.73 s of a 149.35 s
profiled replay, at a mean of 2.3 steps. With rule G shut there are
32, in 5.31 s.

## Why it is not simply moved earlier

The pre-bound needs `|A|`. `A` is accumulated by `Poly::add`, whose
terms can cancel, so a running count past the bound does not prove the
final count is past it. An early refusal on the running count would
refuse some reductions the current code completes, and that moves
forms. What would keep every form: an upper bound on `|A|` that is
exact where it refuses (none is known), or a test on the denominator
side, `|D^h1|·|B|`, which does not need `A` but is not the product
that refuses here.

## Home

`crates/geom-core/src/sym/algebra.rs` (`apply`, `poly_subst_square`).

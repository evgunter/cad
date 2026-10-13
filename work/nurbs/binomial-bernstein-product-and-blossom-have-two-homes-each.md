---
id: binomial-bernstein-product-and-blossom-have-two-homes-each
kind: issue
title: Binomials, the Bernstein product and the span blossom each have two homes in geom_core::spline
status: open
opened: 2026-10-13
priority: P3
cost: M
refs: [4954]
---

## What

PR 4954 (`nurbs/quotient-bound`) gave the quotient bound its own
binomials, Bernstein product and blossom. Each sits beside an older
home in `crates/geom-core/src/spline`.

**Pair 1: binomials and the Bernstein product.**
- `pieces::binomial` is exact in integers to `n = 56`.
  `derivative`'s private `product` and `Grid::raised_to` form the
  product weights `C(m,i)·C(n,j)/C(m+n,k)` at any `Real`.
- `compose.rs` has `binom_row`/`binom_table`, capped at
  `BINOM_EXACT_MAX = 54` because its multiplicative recurrence rounds
  at 55. Its `BernWeights` Bernstein product runs at the certification
  scalar.
- A third copy, `binom` (multiplicative `f64`), sits in
  `crates/geom/tests/curves/piece_coverage_meter.rs`.

**Pair 2: the span blossom and restriction.**
- `pieces::blossom` / `span_row` / `restrict` are Real-generic de Boor
  steps with exact dyadic piece weights.
- `blossom.rs` has `CoeffWindow::blossom` and `SpanRow::restrict`.
  Every step goes through `Certification::between`, under the
  argument contract in that module's docs. `ssi/enclose.rs`'s
  `bezier_on` cuts still read it.

## Why it is not in PR 4954

Neither fold is mechanical. `compose`'s weights are pinned bit for bit
by the rehearsal's ratified bit-identity rows, and its docs reject ring
quotients for exactly that reason. `blossom.rs`'s steps are
certification arithmetic with an argument contract that the
Real-generic one does not carry.

## Shape

One home per primitive.

- **Binomials.** Integer-formed, which also lifts `compose`'s cap
  from 54 to 56.
- **Bernstein product.** Either one Real-generic product that
  `compose` instantiates at its certification scalar, or a stated
  reason the two must differ.
- **Blossom.** `CoeffWindow::blossom` as the Interval instance of
  `pieces::blossom`, if `between`'s clamp can ride as a scalar hook.

Each fold has to show its pinned bits unmoved, or re-baseline them
with a cause.

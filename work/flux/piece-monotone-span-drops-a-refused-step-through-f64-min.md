---
id: piece-monotone-span-drops-a-refused-step-through-f64-min
kind: issue
title: props/quad.rs piece_monotone: the span fold's f64::min drops a refused step's NaN, so the span is the other steps' minimum
status: open
opened: 2026-10-09
---


Found by the NURBS span-locator unit's fix pass (PR 4442) while
building a row that drives a `Trv` bracket through `piece_monotone`.

## The finding

`crates/geom-brep/src/props/quad.rs`, `piece_monotone`: the chord's
per-step advance is folded as

```rust
span = span.min(lo_or_refuse(step));
```

`lo_or_refuse` answers NaN for a step that does not certify, and
`f64::min` returns its non-NaN operand. So a refused step drops out of
the fold, and `span` is the minimum over the other steps. That is an
UPPER estimate of the true least advance, while `Margin::metered`
reads it as the margin of a "definitely apart" claim. Measured: a
block whose last point carries a `Trv` coordinate leaves `span` at the
first step's certified advance.

## Why it is latent

The rate side refuses the same block: its box is a hull, which refuses
an uncertified operand, so the rate is the refused `0` and the claim
does not certify. So no wrong answer is known to come out. The fold is
still unsound on its own terms, and it has the class's shape: a refusal
laundered by `f64::min`.

## Repair

Fold with a NaN-propagating minimum, as `Real::min` does at `f64` (or
`geom_core::interval::min_bound`, if its contract matches), so a
refused step refuses the span.

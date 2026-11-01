---
id: try-line-max-drops-a-refused-locus-certificate
kind: issue
title: recognize_curve's try_line folds a refused INV-C3 sup through f64::max, so a line promotes with no locus certificate
status: open
opened: 2026-09-29
priority: P3
cost: E
---

## Finding

`crates/step-import/src/recognize_curve.rs`, `try_line` (~:307-362):
the INV-C3 locus certificate is `sup = composite_sup(curve, Cylinder
{ radius: 0.0, .. })`, and `composite_sup` answers `NaN` "on every
structural refusal and every refused hull". The comment above it says
that NaN "fails the budget comparison below (D4 ¶2)". It does not reach
that comparison: `let residual = delta_line.max(hull);` is `f64::max`,
which returns the other operand when one is NaN, so a refused
`delta_line` is dropped and `!(residual <= eps_in)` reads the INV-C5
control hull alone.

Measured (CERT-NAMES fix pass, a throwaway unit test, not committed):
a degree-30 Bézier with 31 collinear control points on the x axis
(unit weights) gives `composite_sup = NaN` — the cylinder composite's
degree 60 is past `compose`'s `BINOM_EXACT_MAX = 54`, so the binomial
row is all-NaN and the composite is refused — and `recognize(&c, 1e-6)`
answers `Promoted { Line, residual: 0.0 }`.

On that fixture the promotion is geometrically right (the control
hull does bound the map deviation on unit weights), so the harm is
that the locus certificate the module docs name as INV-C3 is not
load-bearing whenever it refuses, and the comment claims it is. The
sibling folds were fixed once as a class (`nan_propagating_max` in
`geom-brep`'s `ssi.rs`).

## What would close it

Either refuse on a NaN `delta_line` before the fold (a NaN-propagating
max, as `ssi.rs` does), or, if INV-C5's hull is the whole certificate
for unit weights, say so in the module docs and drop the claim that
INV-C3 gates promotion. A row pinning the degree-30 fixture either way.

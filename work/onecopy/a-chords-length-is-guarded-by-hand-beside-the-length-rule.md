---
id: a-chords-length-is-guarded-by-hand-beside-the-length-rule
kind: issue
title: fit's chord_params and clearance's candidate axes guard a vector's norm by hand beside checked_nonzero_length
status: closed
opened: 2026-11-09
priority: P3
cost: E
closed: 2026-11-09
pr: 6647
---


Found by PR 6628's delta review (NOTE-A), and by this row's sweep of the
same shape.

## What

- `geom` `curves/fit.rs` `chord_params`: `d = (pair[1] - pair[0]).norm();
  if !(d > 0.0)` refuses a zero, NaN or underflowed chord, but admits an
  overflowed `d = ∞`, which then makes `total` infinite and every
  parameter `0` or NaN.
- `topo` `clearance.rs`, the candidate separating axes: `components
  finite && n.norm() > 0.0` admits an `n` whose norm overflows.

Both restate a partial copy of `geom_core`'s length rule plus its zero
question, which `Vec3::checked_nonzero_length` answers.

## What else moved (6647's review, NOTE-1)

Beyond the overflow refusal, an *accepted* chord's value moves in the
prescale band: a chord whose largest component lies in about
`(1e-162, 1.5e-154)` now reads the prescaled length, where the plain
norm lost bits to subnormal squares (the same class PR 6628 disclosed
for its plain-norm sites). PR 6647's body said only the overflow side
changed; this is the correction.

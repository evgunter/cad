---
id: a-chords-length-is-guarded-by-hand-beside-the-length-rule
kind: issue
title: fit's chord_params and clearance's candidate axes guard a vector's norm by hand beside checked_nonzero_length
status: open
opened: 2026-11-09
priority: P3
cost: E
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

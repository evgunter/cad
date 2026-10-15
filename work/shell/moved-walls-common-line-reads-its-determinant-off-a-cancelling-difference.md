---
id: moved-walls-common-line-reads-its-determinant-off-a-cancelling-difference
kind: issue
title: walls_cross reads the moved walls' common-line determinant as 1 − k², which cancels for near-parallel walls, while ‖n_a × n_b‖² is in hand
status: open
opened: 2026-10-13
priority: P2
cost: E
---


Found by GERM's sweep for the `TiltedEllipse` cancellation
(`work/germ/flat-cone-tilted-ellipse-stands-off-the-cone.md`). The
shape is a difference of squares of near-unit cosines standing in for
the square of a small sine that is already computed.

`crates/topo/src/shell.rs`, `walls_cross` (~4241): `let det = T::one() -
k.powi(2)` with `k = a.normal.dot(b.normal)`. For walls near parallel,
`k ≈ 1` carries their angle `s` only to `u/s`, so `det` carries a
relative error of about `u/s²`. `p0` divides both of its terms by
`det`. The function computed `cross = a.normal.cross(b.normal)` a
few lines up, and `‖cross‖² = s²` is the same determinant read off
the small quantity. `carrier_cross::common_line`
(`crates/topo/src/boolean/carrier_cross.rs`, gone since PR 5190) used that shape:
`let k = s.powi(2)`.

**What is owed:** `det = cross.norm_squared()`, and a measurement of
whether `p0`'s numerators (`r_a − k·r_b`) still dominate at the
transversal trilean's floor. Not measured: this was found by reading
the code, and no pose has been shown to move.

2026-10-15 (CLEAVE, `cleave/circle-pair-touch-on-both`): measured on
the twin this row cites. `carrier_cross::common_line` already used
`k = s²`, and its `p0` still sat up to `u·|h|/s²` off both planes
(`1e-6` at `s = 1e-5` with `|h| ≈ 4`, ε = 1e-9; metres at
`s = 1e-8`): the numerators `h₁ − h₂·c` cancel as the determinant
does. So the determinant swap alone does not settle this row; reading
`r_a − k·r_b` at the transversal floor is the measurement owed.
`carrier_cross` replaced the line by meeting each conic with the other
plane in its own angle.

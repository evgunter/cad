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
(`crates/topo/src/boolean/carrier_cross.rs`) already uses that shape:
`let k = s.powi(2)`.

**What is owed:** `det = cross.norm_squared()`, and a measurement of
whether `p0`'s numerators (`r_a − k·r_b`) still dominate at the
transversal trilean's floor. Not measured: this was found by reading
the code, and no pose has been shown to move.

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
the small quantity, the shape `carrier_cross::common_line` used
(`let k = s.powi(2)`) before it was retired with
`carrier_cross::meetings`.

**What is owed:** `det = cross.norm_squared()`, and a measurement of
whether `p0`'s numerators (`r_a − k·r_b`) still dominate at the
transversal trilean's floor. Not measured: this was found by reading
the code, and no pose has been shown to move.

## Note (2026-10-26, easy-fixes lane)

The first half is done: `crates/topo/src/shell.rs` `walls_cross` takes `det = cross.norm_squared()`, the `‖n_a × n_b‖²` already in hand. Both normals are `Surface::Plane` normals, unit by the chart's contract, so this is the same determinant `1 − k²` stood for. Verified: `cargo nextest run -p topo --tests -E 'kind(test) & test(/shell|moved_wall|thin|split|merge_face|section/)'`, 168 passed, and the topo lib rows, 367 passed. Still open: the measurement of whether `p0`'s numerators `r_a − k·r_b` and `r_b − k·r_a` dominate at the transversal trilean's floor. At `k ≈ 1` with `r_a ≈ r_b` they are differences of near-equal terms too, and no pose has been measured.

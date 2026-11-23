---
id: a-curved-covers-far-distance-carries-a-nan-derivative-on-the-dual-lane
kind: issue
title: geom::cover's gram_top takes a square root at zero on every orthonormal frame, so a curved cover's far distance carries a NaN derivative on Dual
status: open
opened: 2026-11-23
priority: P2
cost: E
---


Found by the turn-lever measurement lane (2026-11-23), by a code read and a probe (`crates/geom/tests/tl_dual_cover_probe.rs` in its measurement tree, `analysis/oneround/turn-lever/g2-measure.patch` on `oneround/turn-lever`); I checked that `gram_top` reads as quoted.

`gram_top` (`crates/geom/src/cover.rs`) is `√((A + B)/2 + √(((A − B)/2)² + C²))`. On every unit orthogonal frame `A = B = 1` and `C = 0`, so the inner root is `√0`, and `Dual::sqrt` (`crates/geom-core/src/dual.rs`) forms its derivative as `deriv / (r + r)`, which is `0/0`. `OffAxis::of` reads `gram_top` for its `top` and `off`, so the far distance of every curved piece (a tube, a torus, a sphere, a patch, a wall, a cone, and a hull or meet of them) comes back with a finite value and a NaN derivative; `tilt`'s root does the same where the line is parallel to the axis, and a point exactly on the line through the same `√0`. With every input a `Dual::constant` the probe still reads NaN, so it is the root's kink at zero and not a sensitivity.

**No verdict reads it.** `impl Decide for Dual` classifies on the value channel alone, and `is_poison` and `Bounds` are value-only, so a levered margin built on such a bound serves exactly as f64 does: in the lane's run with every levered decision read against both bounds, 116,359 `Dual` verdicts at ε 1e-9 with none refused and none moved. What it costs is the other half: any consumer that reads the derivative of a quantity derived from a cover's far distance (a sensitivity, a stackup) reads NaN on every curved face.

Owed: the root posed so that an orthonormal pair has a derivative (the inner root is the half-difference of the two eigenvalues: where it is zero the top eigenvalue is `(A + B)/2` and its derivative is that of the trace's half, or the pair is read as orthonormal where the frame's own type says so), with a row that reads a curved cover's far distance on `Dual` and holds a finite derivative.

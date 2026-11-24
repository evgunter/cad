---
id: spline-control-rounding-is-a-parallel-array-read-twice
kind: unit
title: A spline control point carries its rounding as a parallel f64 array, and its out-of-homogeneous read is written twice
status: review
opened: 2026-11-20
priority: P1
cost: M
branch: oneround/spline-control-rounding
pr: 7652
---


Found by the blind-spot sweep of `oneround/rounded-pairs` (value/error
pairs read by hand beside `geom_core::running::Rounded`), second pass
(tuple destructurings holding a `rounding` name).

A spline control point is carried as `Control = ([T; N], [f64; N])`: its
homogeneous coordinates and, beside them, an `f64` bound on each one's
rounding. `blend` propagates the bounds by hand, which is `Rounded`'s
arithmetic written out (`geom_brep::pcurve_cache::spline_incidence`,
`blend`; `topo::spline_trim`, `blend`). The point out of homogeneous
form is then read by a hand-guarded division by the weight's lower end:
`let lead = w - e[3]; if !(lead > 0) { INFINITY }`, then
`(e_c + |P_c|·e_w)/lead + u·|P_c|`. That is `Rounded::divisor` and
`Rounded::quotient` spelled on the fields.

The read is also written twice. `spline_incidence::point_rounding` and
`spline_trim::point_rounding` restate the same formula at `N = 4` and at
general `N`, each with its own mutant hook. Their doc comments carry the
same formula.

The value lane is `T` and the bound lane `f64` (`middle(h[3])`), so a
`Rounded<T>` does not hold it as it stands. Either the bound becomes a
`T` (zero at an enclosure scalar, as the doc already says it reads
there), or the pair gets a one-home reader in `geom_core::running` for an
`f64` bound beside an enclosure.

Sound today: weights are positive, and a weight whose bound reaches it
reads infinite and refuses.

P1 by `work/README.md` Priority (two implementations of one underlying
logic).

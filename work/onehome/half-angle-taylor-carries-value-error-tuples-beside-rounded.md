---
id: half-angle-taylor-carries-value-error-tuples-beside-rounded
kind: issue
title: half_angle's taylor carries (value, error) tuples and spells room by hand beside running::Rounded
status: open
priority: P1
cost: M
opened: 2026-11-02
---


Found in the review of PR 6376 (`onehome/vertex-section-room`), which
gave `geom_core::running::Rounded` its `room()`, `divisor()` and
`toward_zero()` and routed the tree's spellings of those through them.

`geom_brep::half_angle` (the certified subdivision's bisection step)
reads `F`'s Taylor terms as `[(f0, e0), (f1, e1), (f2, e2), (f3, e3)] =
taylor(m)`: `(value, error)` tuples, a parallel vocabulary to `Rounded`.
On them it spells by hand:

- the room, `f1.abs() - e1` (the `d1_lo` beside `d1_hi`) and, in the
  slack meter, `near = g1.abs() - e1`;
- the magnitude, `f2.abs() + e2`, `f3.abs() + e3` (`Rounded::magnitude`);
- a division by that room unguarded, `w = (two * reach / near)`, whose
  room may be zero or negative (the `.min(half).max(T::zero())` clamp
  hides a negative `near` rather than refusing on it; read whether a
  negative `near` can reach `lever`).

Fix: have `taylor` return `Rounded<T>` terms and read them through
`room()`, `magnitude()` and `divisor()`, and refuse (or show unreachable
with a cited argument) where `near` has no room. The unguarded division
is the same defect class PR 6376 closed for `Rounded::quotient`.

P1 by `work/README.md` Priority (two implementations of one underlying
logic). Open PR 6329 (`[ev]`) may retire `Rounded` as an f64-lane
charge; this row then becomes whatever that ruling makes of
`half_angle`'s bounds.

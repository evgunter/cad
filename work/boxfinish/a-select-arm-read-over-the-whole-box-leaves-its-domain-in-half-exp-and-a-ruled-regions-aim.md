---
id: a-select-arm-read-over-the-whole-box-leaves-its-domain-in-half-exp-and-a-ruled-regions-aim
kind: issue
title: Two value selections read an arm over the whole box that is bounded only on its own side of the selector: a hyperbola's half_exp and a ruled region's aim
status: open
opened: 2026-11-24
---


Found by `boxfinish/wall-roots`'s sweep (the class of
`geom_brep::quartic::quadratic_roots`, fixed there). Read from the
code, not run. `Real::select_le_zero` at `Interval` hulls both arms
where the selector straddles zero, and each arm is evaluated over the
whole box, not over its own side.

- `geom_brep::intersect::conic_arc::ConicBranch::half_exp`:
  `s.select_le_zero(1/(c − s), s + c)` with `c = √(1 + s²)`. The first
  arm is bounded only for `s ≤ 0`: over `s ∈ [−a, b]` with `b ≥ 1`,
  `c − s` contains zero and the root is the whole line. The second arm's
  `√(s + c)` leaves its domain the same way for `a ≥ 1`. The cure is
  `quadratic_roots`': each arm reads `s` held to its own side
  (`s.min(0)`, `s.max(0)`).
- `topo::ruled_region` (the aim, `on_axis` and `dir`):
  `radial / rho.select_le_zero(1, rho)`, `rho = |radial| ≥ 0`. An
  enclosure `rho = [0, ε]` straddles by the door's rule (`lo ≤ 0 <
  hi`), so the divisor is `[0, 1]` and `dir` the whole line. Holding
  `rho` to its side does not cure it (the divisor is still `[0, ε]`):
  it needs a floor, or a hull of directions.

Not established: that any fixture reaches either with a box this wide.

---
id: root-slack-meter-reads-an-interval-root-at-its-whole-width
kind: issue
title: The subdivision's root-slack meter reads an Interval root over its whole enclosure, so the dependency problem refuses about a quarter of the Interval lane's correct shallow-crossing answers
status: open
opened: 2026-10-08
priority: P3
cost: M
refs: [circle-torus-certifies-shallow-roots-off-by-more-than-the-band, root-slack-meters-roots-outside-the-arc]
---



Found by the lane that gave the circle × torus door its root-slack
meter (`circle-torus-certifies-shallow-roots-off-by-more-than-the-band`).

## What

`circle_roots::certified_subdivision` meters a located root's slack
from `RootSlack::residual` read AT the root. On the `Interval` lane the
root is the bisection's enclosure, and the meter's reading
(`geom_brep::conic_torus_implicit`, `conic_quadric_residual`,
`conic_torus_residual`, `conic_cone_residual`) is evaluated over that
whole enclosure: `sin`/`cos` of an interval, then the point, the axial
split and the squares, each widened by the dependency problem. The
reading's width is then about `|∇F|·|C′|·w` for an enclosure `w`
radians wide, not the slope ALONG the carrier times `w`; at a shallow
crossing the two differ by the ratio the meter divides by, and the
slack reads past `ε`.

## Measured

On the circle × torus measurement set (9,000 poses × 3 ε, exact
oracle), the meter newly refuses on the `Interval` lane 1,193 of 4,804
correct answers at `ε = 1e-12`, 1,134 of 5,130 at `1e-9` and 11 of
2,655 at `1e-6`. None of them was wrong: the lane's enclosures held
the truth on every pose before the meter.

## What would close it

Read the meter's residual in a centred form: at a point of the
enclosure, plus the slope times the enclosure's half-width — which
needs a point of an enclosure in generic code, a seam `Bounds` does
not offer the funnel (`geom_core::real::bounds_allowlist`). Or
measure that the `Interval` lane's enclosure of the root already IS
the slack the caller decides on, and say where the meter is owed only
the `f64` lane's. Either is a design question on the scalar seam, not
a local fix.

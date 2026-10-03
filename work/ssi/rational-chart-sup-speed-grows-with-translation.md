---
id: rational-chart-sup-speed-grows-with-translation
kind: issue
title: ssi/enclose: a rational wall's chart sup speed (cell_homogeneous_deriv) grows with the wall's absolute coordinates (822 at 100 m, 8.1e5 at 1e5 m against a true ~1.3), so floors and the settling readout over-refuse translated rational walls
status: closed
opened: 2026-10-01
priority: P2
cost: M
closed: 2026-10-02
pr: 3863
branch: ssi/rational-chart-speed
---


(SSI orchestrator, from the delta review of PR 3707, 2026-10-01. The
numbers are measured.)

`crates/geom-brep/src/ssi/enclose.rs` `cell_homogeneous_deriv` (now
`cell_quotient_numerator`) encloses
a rational wall's derivative from its homogeneous coordinates. The
enclosure's width scales with the absolute coordinates, so the minted
chart sup speed is roughly the true speed times the distance from the
origin. On a strongly rational wall (weights 1.8 / 0.7), with a true
speed of about 1.3, the sup speed reads 822 at 100 m and 8.1e5 at 1e5 m.

Effect: every consumer of the chart speed over-refuses a translated
rational wall. That covers the accounting and seeding floors
(`CellBudget` on the merge base, `FloorUnresolvable`/`SettlingUnresolvable`
on the Chart lane after PR 3707) and the tube pad. A wall at
the origin is unaffected.

Likely fix shapes, to weigh:
- enclose the quotient-rule derivative about a cell-local origin, so the
  translation cancels before the interval arithmetic sees it;
- or bound the speed from the derivative's own Bernstein form (cf. the
  coefficient-norm reading PR 3685 gave limb 2).

The first step is to measure which term carries the growth.

## Closed (2026-10-02, PR 3863)

Measured: both `hull(A_d)` and `sbox·hull(w_d)` carry `|t|·hull(w_d)`,
and their decorrelated subtraction cannot cancel it (at 100 m,
`A_d.x = [−330, 242]` against `(S·w_d).x = [−333, 243]`). Neither term
alone carries the growth; the subtraction does.

`cell_quotient_numerator` (renamed from `cell_homogeneous_deriv`) now
hulls the numerator per coefficient pair
as `k·(w₁·(P₁ − P₀) + (w₁ − w₀)·(P₀ − S))`, with `S` in the span cell's
own control box. Every point enters as a difference, so a translation
moves `deriv_box` and `chart_speeds` by their rounding width. Each
term lies inside the old decorrelated hull in real arithmetic, so the
box does not widen beyond ulps: the PR's reviewer measured about 4% of
components wider, by at most 4e-16 relative. A non-rational net on
uniform knots reads the same bits; on non-uniform knots 81 of 3,318
components moved by an ulp, all from the knot divisor now being
rounded outward. On a cubic × linear wall weighted
1.8 / 0.7 the `u` chart speed reads 5.858 at 0, 100 m and 1e5 m
(before: 10.35, 1420, 1.41e6; true 2.036). Pinned by
`a_translated_rational_wall_reads_the_origins_chart_speed_and_floors`,
`the_derivative_box_dominates_the_dense_sampled_true_derivative`
and `the_quotient_numerator_box_holds_the_sampled_numerator`
(`enclose.rs` tests); the last goes red on the `w₀`/`w₁` swap in the
pair term. No golden moved.

Rotation invariance stays with
`the-chart-speed-and-tube-pads-read-a-derivative-norm-off-a-per-coordinate-box`.
The evaluation lane's wide-parameter twin is filed as
`work/props/nurbs-interval-ders-at-a-wide-parameter-grows-with-translation.md`,
and the quadrature lane's rational cross as
`work/quad/quad-rational-patch-cross-grows-with-translation.md`.

---
id: the-periodic-chart-closure-decides-are-four-spellings-of-one-question
kind: issue
title: Whether a loop closes on a periodic chart is decided four ways, with four meters and three refusals
status: review
pr: 4938
opened: 2026-10-12
---



## What

Each lane that reads a face's loops on a periodic chart decides that
they close, and wind as a face's boundary does, in its own way:

- `geom_brep::props::quad::loop_windings` (`props_quad_junction_closed`):
  per junction, the chart gap less whole azimuth turns, its excess over
  the ends' certified slack and `3ε`; the turns are then integers, held
  to `|k| ≤ 1` and `Σk = 0`. Refuses `ChartLoopOpen` or `LoopsWindChart`.
- `props::curved::cylinder_chart` (`props_loop_closed` in space per
  junction, `props_chart_loops_closed` on the face's summed rim `Δu`,
  levered at the radius, decided Zero; `props_ring_contractible` per
  ring). Refuses `NotIsoRectangle`. It does not hold a loop to one turn.
- `props::curved::torus_chart` (`props_torus_lift_closed_u` / `_v`, the
  lift's summed `Δu`, `Δv`, levered at the major and minor radii).
  Refuses `NotIsoRectangle`, and refuses any loop that winds.
- `props::spiric` (`props_spiric_lift_closed`, the mignitude of the
  lifted `Δu·R`, `Δv·r`). Refuses `NotIsoRectangle`.

One stored defect — a loop that does not close, or winds as no boundary
does — reads as a defect in one lane and as "not a shape the closed
forms fold" in three. Only the quadrature decides per junction against
a stated slack, and only it holds a loop to one turn.
Found by PR 4839's dual review.

## The shape of a fix

One decision for the class: per junction, a gap less whole periods, in
excess of a stated slack, giving integer windings per periodic
direction, with the face-boundary facts (`|k| ≤ 1`, `Σk = 0`) as
integer checks, and one refusal for each of "does not close" and
"winds as no boundary does". The torus's real question — which lift a
band's second rim takes — stays its own.

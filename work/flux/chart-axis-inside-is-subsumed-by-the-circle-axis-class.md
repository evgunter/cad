---
id: chart-axis-inside-is-subsumed-by-the-circle-axis-class
kind: issue
title: props_chart_axis_inside decides nothing the parse's axis class has not, so no row can red its removal
status: open
opened: 2026-10-19
priority: P4
cost: E
---

Found by the FLUX cone-closure unit (`flux/cone-closure-and-per-term-whole`,
PR 5333), asked for a mutant that drops the per-rim
`props_chart_axis_inside`.

`closure::require_axis_inside` (`crates/geom-brep/src/props/closure.rs`)
decides a closed form's circle radius definite at `Kε` before
`curved::about_axis` carries its stored turn to the axis. Its own doc
says why it never refuses: `Kε` is also the least radius
`props_circle_axis_class` admits a rim at, so every circle that reaches
it has already been decided past it. Dropping the torus's per-parallel
call (`torus_chart_loop`) leaves every row of geom-brep's closure, cone,
iso-parse, s58 and rim-dimension suites green at ε = 1e-9; the
cylinder's call is in the same position.

The cone carries no such decide: its rims' ends are decided off the
axis by `props_cone_loop_off_apex` / `props_cone_ring_off_apex` before
`cone_loop_winding` carries their turns, which places the axis inside
each rim the same way.

Either the decide guards a case no row reaches (a circle admitted some
way other than the axis class), and wants a row that reds without it,
or it is redundant with the axis class and the repair is to delete it
and say in `about_axis`'s doc which decide places the axis inside.


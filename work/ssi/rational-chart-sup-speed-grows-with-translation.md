---
id: rational-chart-sup-speed-grows-with-translation
kind: issue
title: ssi/enclose: a rational wall's chart sup speed (cell_homogeneous_deriv) grows with the wall's absolute coordinates (822 at 100 m, 8.1e5 at 1e5 m against a true ~1.3), so floors and the settling readout over-refuse translated rational walls
status: open
opened: 2026-10-01
priority: P2
cost: M
---


(SSI orchestrator, from the delta review of PR 3707, 2026-10-01. The
numbers are measured.)

`crates/geom-brep/src/ssi/enclose.rs` `cell_homogeneous_deriv` encloses
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

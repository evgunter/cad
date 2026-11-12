---
id: offset-corner-on-surface-escalates-at-interval-on-an-arc-necked-extrude
kind: issue
title: The general offset door refuses offset_corner_on_surface at Interval on an extruded profile with arc necks (bulge), so a curved twin of the OFF-D dumbbell cannot be shelled at certifying scalars
status: open
opened: 2026-10-30
priority: P2
cost: M
---


## Finding

Found by CLEAR's SHELL-4 lane (`clear/shell4` at `623daf636b`, WIP test
module `shell_cavity_embedded`). Filed by the CLEAR orchestrator.

SHELL-4 needs a curved twin of the OFF-D dumbbell: an extruded profile
whose neck is arcs (`bulge_loop`), shelled at a `t` whose walls cross.
At `Interval`, every variant tried refuses in the general offset door
before the cavity gate is reached: six bulge/`t` pairs, including a thin
wall at bulge 0.2, `t = 0.05`. Each fails as
`Face { Escalated offset_corner_on_surface }`, with enclosures of about
4–5e-6. At `f64`, bulge 0.2 / `t = 0.15` shells Ok.

So at certifying scalars an arc-necked extrude cannot be shelled at all.
The decision `offset_corner_on_surface` does not settle at `Interval`
widths that are far from in-band. Either its margin is too wide at the
arc–line corner, or the corner point is enclosed by a wide evaluation.
The site is `crates/topo/src/offset_derive.rs`.

What it blocks: SHELL-4's curved-twin row, a refusal with a witness at
`Interval`. SHELL-4 can land without it by pinning the twin at `f64`
and saying so. The certifying-scalar row then waits on this one.

- 2026-11-02 — moved from OFFSET to SHELL by `git mv` when OFFSET merged into SHELL, id and body unchanged (the track budget now counts only P0, P1 and unbanded rows, and OFFSET had fallen far under it).

## Measured (CLEAR's verdict-door lane, 2026-11-12)

The fixture this row was found on did not have an arc neck. A bulge in
`bulge_loop` rides the segment its vertex starts, and the SHELL-4
dumbbell put its bulges on `(3, 0.8)` and `(2, 1.2)`. Those start the
blob walls `x = 3` and `x = 2`, so its neck stayed straight and 0.4
wide, and at `f64` its cavity certifies embedded (bulge 0.2,
`t = 0.15`: 56 candidates, nothing refused). With the bulges moved to
`(2, 0.8)` and `(3, 1.2)`, the neck walls are arcs. At `f64` the cavity
gate refuses `CavityCrosses` at bulge −0.2 and at +0.2 (`t = 0.15`),
and shells Ok at −0.2, `t = 0.05`. At `Interval` all three refuse here,
`Face { Escalated offset_corner_on_surface }`, with enclosures of
1.3–1.6e-6. The row's finding holds for a real arc neck. Its exhibit,
`shell_cavity_embedded`'s `the_curved_dumbbell_refuses_at_f64_and_waits_on_the_offset_door_at_interval`,
now builds the arc neck.

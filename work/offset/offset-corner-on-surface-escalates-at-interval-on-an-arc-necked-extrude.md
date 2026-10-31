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

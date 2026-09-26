---
id: half-revolve-caps-are-never-an-operand
kind: issue
title: A pi revolve of an axis-touching profile returns coplanar co-oriented Start and End caps on different keys, so it can never be a boolean operand
status: open
opened: 2026-09-25
priority: P2
cost: D
refs: [full-revolve-emits-split-planar-walls]
---


## What

`Revolution::Partial(π)` of an axis-touching profile returns Start and End
caps that are coplanar, co-oriented and adjacent across the axis edge, on
DIFFERENT surface keys. `merge_coplanar_faces` finds no group, and the
boolean's F7 gate refuses `UndeclaredCoincidence`. Their coplanarity rests
on θ = π, which is decided numerically. Measured by GERM's dumbbell lane
(2026-09-25) with a rectangle; a 3π/2 revolve is maximal and unions fine.

## Parked by Ev

Split off the full-revolve fork. Ev, on its `[ev]` PR (2026-09-25): park it,
"file it as P2". If it is ever wanted, both designers pointed the same way:
recognise an exact half turn as a recorded, flagged verdict, the way the
recipe layer recognises `Full` from |θ| − τ at the band, and emit one cap
spanning both sides of the axis. Their reports are in the `[ev]` PR that
ruled `full-revolve-emits-split-planar-walls`.

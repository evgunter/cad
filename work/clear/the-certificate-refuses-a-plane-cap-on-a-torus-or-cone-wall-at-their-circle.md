---
id: the-certificate-refuses-a-plane-cap-on-a-torus-or-cone-wall-at-their-circle
kind: issue
title: The injectivity check refuses Uncertified, departure arm at the depth floor, where a plane cap meets a torus or cone wall at their shared circle: seven finished tour bodies would not finish at the at-rest gate
status: open
opened: 2026-11-12
priority: P0
cost: M
---


## Finding

Measured by CLEAR's lane clear-gate (`the-at-rest-gate-certifies-embedding`,
its Measured section). Main at `348ec09727`; the body-level question
the gate would ask (`StrictlyPositive`, one body against itself,
`DEFAULT_MAX_CELL_DEPTH`, 65,536 pairs), at `f64`, on bodies the tour
builds and tier 3 passes.

Seven bodies refuse `EngineRefusal::Uncertified { arm: Departure }` at
depth 40, each on a plane face against a curved wall across their
shared circle:

| body | pair | refuses after | pairs |
|---|---|---:|---:|
| `lily_stem`, `lily_arch`, `lily_pedicel` | plane × torus, at a `Circle` | 0.27–0.62 s | 1,718–3,462 |
| `tube_along_arc` | plane × torus, at a `Circle` | 0.20 s | 1,028 |
| `teapothandle` | plane × torus, at a `Circle` | 0.27 s | 1,598 |
| `hollowelbow` | plane × torus, at a `Circle` | 1.23 s | 18,798 |
| `lily_lantern` | plane × cone, at a `Circle` | 1.79 s | 23,283 |

The circle is a chart iso of the torus (a meridian) and of the cone (a
parallel), and the pair still splits to the floor, so the obstacle is
local to some cells along it, not the edge's presentation at the root.
Where along the circle was not traced. A candidate is where the circle
meets the wall's seam line (a corner of a curved edge and a line):
`departure-reads-no-curved-corner-off-its-shared-edge`.

## Why P0

`the-at-rest-gate-certifies-embedding` (P0) cannot land its gate while
these bodies refuse: the gate would refuse them at their doors.

## Fix shape

Trace one refusing cell (`tube_along_arc` is the smallest), then fix the
reading it lacks; the six other bodies are the rows that prove it.

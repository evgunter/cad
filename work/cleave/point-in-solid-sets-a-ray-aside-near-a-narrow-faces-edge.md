---
id: point-in-solid-sets-a-ray-aside-near-a-narrow-faces-edge
kind: issue
title: Point containment's ray walk sets a crossing within Kε of a face boundary aside, so a needle operand thinner than 2Kε loses its rays
status: open
opened: 2026-10-14
priority: P2
cost: M
---

Found by FLUSH's needle-class design fork (fork-log row 142, PR 5119;
the FLUSH row `the-side-reader-refuses-a-body-thin-in-two-directions-whose-pairs-are-definite`).

## What

`boolean::solid_contain`'s in-face walk (`point_in_loop`) refuses a
ray crossing that lands within Kε of a loop boundary. A face narrower
than 2Kε has no point more than Kε from its boundary, so every crossing
of it is in band. The shell-role reader on PR 5119 sets such rays aside
(measured: `brick (0,1)×(0,w)×(0,w)` at w 10.5–21ε, ε 1e-9 and 1e-6).
`point_in_solid`, the boolean's containment fallback, walks the same
in-face test, so a needle operand loses its rays there too (read, not
measured).

## Owed

The fork's converged design decides each edge a ray passes once, for
both faces the edge bounds, with one tie-break (a watertight cast), so
no crossing is in band of a boundary. A designer measured it reading
all 20 needle cases on planar faces (`flush/needle-design-x` at
`feb5b660e`, `ndx_side`). If Ev rules it on PR 5119, the needle unit's
ray-walk change serves `point_in_solid` as well; measure a needle
operand through `point_in_solid` there.

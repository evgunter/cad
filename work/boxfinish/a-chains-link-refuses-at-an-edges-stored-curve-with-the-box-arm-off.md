---
id: a-chains-link-refuses-at-an-edges-stored-curve-with-the-box-arm-off
kind: issue
title: The tour's one-link chain refuses at Interval with the box arm off: an edge's stored curve does not certify against its faces
status: open
opened: 2026-11-24
priority: P1
cost: M
---


Found by M11 (the probe lane, 2026-11-24; `probe/M11-REPORT.md` on
`boxfinish/probe-switch-off` at `e37099a49a`), on one tree with the
switch's box arm off and on.

The tour command's chain study, one link, at `Interval`:

| | box arm on | box arm off |
|---|---|---|
| the row | "CERTIFIES 0.02 s, 0 refused" | "refuses 0.05 s, node 111e1e667832 ... an edge's stored curve does not certify against its faces" |
| the command | runs to its end, 109 s | panics at `chaintol.rs:297` (710 s at M10) |

So the exit M10 could not attribute is the box arm's: with the leaf
read at the box scalar, the link's finish refuses at an edge's stored
curve.

**Not measured:** which edge and which rung of the curve's certificate
refuses, and whether PR 7370 (the radial readers, which re-posed the
pcurve cache's wall distances) changes it: the probe tree predates it.
That measurement, on main with PRs 7370 and 7480 merged, is the first
step (M12).

It holds the switch's deletion: the tour does not run to its end
without it. (BOXFINISH orchestrator)

---
id: main-red-projectbox-wall-1-no-longer-refuses-at-eps-1e-6
kind: issue
title: Main red: the tour's eps_regression::tour_runs_green_at_eps_1e_6 panics on projectbox wall 1 (the round-wire spring's volume) NO LONGER REFUSES
status: open
opened: 2026-10-21
priority: P1
cost: E
---


## What

`demos/tour` `eps_regression::tour_runs_green_at_eps_1e_6` is red on main
8c7db2ae (found by NURBS's FULL review of PR 5542, which reproduced it on main).
`demos/tour/src/projectbox.rs:198` pins wall 1. The round-wire
(`circle_split`) spring's `mass_properties` refuses
`PropsError::QuadratureBudget`, which its doc says is past even ε = 1e-6's
target (width 3.59e-3 m). At ε 1e-6 the call now succeeds, so `walls::wall`
panics "NO LONGER REFUSES", by design.

This is the class #5499 → #5519 hit on the Klein bottle's wall 8. A kernel
change moved a wall's verdict, and no per-PR row walks the tour at 1e-6
(`work/ciw/demos-row-skips-kernel-changes-to-doors-the-tour-calls`, P2).

## To do

- Find the change that let the volume decide at 1e-6 (bisect the tour row).
  Confirm the new volume is right (against `coil_volume(π·WIRE_H²)`), not
  merely accepted.
- If it's right, retire wall 1 at that ε the way Klein wall 8 was retired
  (kept live, asserting it measures), and update the gap comment and
  findings. The wall pins `work/quad/a-swept-circle-section-loop-decides-its-volume-sign-only-at-the-origin`,
  so check whether that item moved too.
- If the volume is wrong, that is a QUAD soundness defect, and it ranks above
  the wall.

---
id: rocker-crease-radius-stays-at-the-eye-after-wall-1-retired
kind: issue
title: tour: the rocker's R_CREASE stays at R_EYE although sided headroom (PR 4092) lets the crease carve at R_BLEND
status: closed
opened: 2026-10-07
priority: P3
cost: E
pr: 4287
closed: 2026-10-08
---


Left by PR 4092's fix pass. With sided headroom, the rocker's wall 1 retired:
`R_BLEND = 0.5` carves the crease tier-3 at `crease_cut`'s closed form, and
`r = 0.6` refuses `FaceClearanceUncertified` (`demos/tour/src/rocker.rs`).
The retired wall's note asked for `R_CREASE` to rise to `R_BLEND`. The fix
pass kept it at `R_EYE`, because raising it moves the rendered frame and
`TestRocker`'s Python mirror (`CREASE = 0.25`).

**What the taker owes:**
- raise `R_CREASE` to `R_BLEND`;
- re-baseline the frame and say what moved;
- update the Python mirror and the README row.

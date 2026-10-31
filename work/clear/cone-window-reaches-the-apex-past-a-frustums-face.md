---
id: cone-window-reaches-the-apex-past-a-frustums-face
kind: issue
title: window_of stretches a cone face's window to the apex though the face stops short of it, so the injectivity walk spends its pair budget on apex cells it can never clear (the cone frustum's cavity, a 3/64 frustum operand)
status: open
opened: 2026-10-31
priority: P1
cost: M
---


## Finding

Found by CAVITYDIAG (#6105); the full analysis is in
`shell-cavity-gate-over-refuses-three-valid-bodies`.

- `window_of` (`crates/topo/src/clearance.rs`) always widens a cone's `v`
  range to `min(…, 0)`, which reaches the apex. A frustum face stops
  short of the apex, at `v = −0.1037` for the cavity.
- `cell_region` can never clear the apex cells, and the frustum's two
  halves meet in space there.
- So the walk spends about 179k exhibit-arm splits on apex cells and
  exhausts `Budget(Pairs 65536)` at both scalars, in 8.5 s.
- A frustum operand with top radius 3/64 fails the same way. The 2/64
  operand holds only by luck of the bisection.
- Pinned: `a_cone_frustums_cavity_exhausts_the_budget_at_its_apex`.

## Decided shape (CLEAR orchestrator)

End `window_of` at the boundary's `v` range when the apex is not on the
face. The reviewer and the lane both found this the simpler of the two
options; the other was discharging cells outside `Boundary::extent`.

It is decided at a funnelled site, or structurally from the face's
loop. A face that does reach the apex keeps today's window.

Blocks SHELL-4.

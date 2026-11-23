---
id: a-near-right-angle-cylinder-pair-stops-at-a-first-order-sector-side
kind: issue
title: A cylinder pair a hair short of perpendicular stops at CurvedSectorSideUnsupported: the pierce's sector side is decided Zero at 7.5e-17 without the second-order trilean
status: closed
branch: germ/sector-side-order2
pr: 7174
opened: 2026-10-31
priority: P1
cost: M
closed: 2026-11-20
---


Filed by the SECTARMS orchestrator (2026-10-31) from its survey lane's
re-run of `work/sectarms/skew-cylinder-germ-pair-has-no-section-frame.md`'s
poses on main at b357165a. Filed on APEX beside
`vertex-vertex-side-codes-take-no-curvature-charge-on-curved-sector-faces`;
`sectors.rs` is shared by many programs, so re-home it if another
program's row already owns the second-order sector side.

## What

The drum `cyl(0.5, 1.0)` against itself, B turned 1.5707963 rad about `x`
(a hair short of a right angle) and moved by `(0.3, 0.2, 0.1)`, ∪ in both
member orders, refuses `CurvedSectorSideUnsupported` before it reaches
the join. The sector side is decided Zero at a margin of 7.538e-17
(`bool_pierce_sector_side_curved`, raised in `boolean/sectors.rs` and
`vtxfac.rs`). The second-order sector trilean
(`geom_brep::enters_material_order2`) is not wired into that verdict.
At exactly π/2 the same pose passes this gate and stops at the join's
frame (the SECTARMS row), so this gate refuses at a pose next to one it
passes, and SECTARMS's fix will not move it.

## Reproduce

The survey's probe is uncommitted. It is a `crates/sweep/tests/` row
on `common::germ_pair`'s drum, with the placement above.

## Done when

The sector side at this pose is decided by the second-order reading, or
refuses on a margin it can name, and the pose is pinned at whatever it
reaches next.

Moved from APEX to GERM on 2026-11-02, by `git mv`, when APEX merged into GERM: the track budget now counts only the P0, P1 and unbanded rows a sitting dispatches (Ev, in chat, 2026-11-02), so GERM, GERMCERT, ROOTS and APEX came in under one budget.

## Closed (2026-11-20, PR 7174)

A pierce's sector-side charge decided Zero now ties the bound (`TiedBy::Charge`). The tie is read by its jet through the existing second-order descent, and only on a parting decided off the zero band. A charge tie whose descent reads no side refuses `CurvedSectorSideUnsupported` on the charge, so it never reads `On`. vtxfac counts only `Band | Slack` ties as first-order (`TiedBy::at_first_order`).

At the filed pose (turn 1.5707963), both member orders behave the same. The default ε escalates honestly on `enters_material` (rise 8.4e-9, in band). 1e-6 builds at the closed form, with its pad over the 1e-7 budget (SPIRICARM's pad row). 1e-12 builds SOUND. π/2 − 1e-6 builds SOUND at default and 1e-12. The rows are `crates/sweep/tests/a_near_right_angle_drum_pair.rs`.

The full review raised one MAJOR and three MINORs, all fixed; the delta review approved. The remaining notes are filed as `a-charge-ties-jet-at-an-arm-near-r-reads-past-a-barely-decided-parting` and `one-sector-side-refusal-has-three-constructions`.

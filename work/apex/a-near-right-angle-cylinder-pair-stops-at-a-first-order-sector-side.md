---
id: a-near-right-angle-cylinder-pair-stops-at-a-first-order-sector-side
kind: issue
title: A cylinder pair a hair short of perpendicular stops at CurvedSectorSideUnsupported: the pierce's sector side is decided Zero at 7.5e-17 without the second-order trilean
status: open
opened: 2026-10-31
priority: P1
cost: M
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

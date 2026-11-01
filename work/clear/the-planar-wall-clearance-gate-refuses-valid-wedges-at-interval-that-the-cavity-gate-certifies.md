---
id: the-planar-wall-clearance-gate-refuses-valid-wedges-at-interval-that-the-cavity-gate-certifies
kind: issue
title: The planar wall-clearance gate refuses valid wedges at Interval that the cavity gate certifies
status: open
opened: 2026-11-01
priority: P2
cost: M
---


## Finding

Found by SHELL-4's review (PR #6183), measured on `clear/shell4`.

**Reproducer.** A wedge prism whose profile is `(0, 0) (4, 0) (4, 0.5)
(0, 0.42)`, extruded 2, shelled at `t = 0.2` at `Interval`. Its
material is thinnest at `x = 0`, where it is 0.42 thick, which is more
than `2t = 0.4`. So the offsets of its top and bottom faces do not
cross.

- **Today:** the shell refuses
  `WallClearance { gap 0.46, needed 0.4 }`.
- **With the planar gates skipped where a clearance lane answers:**
  the shell builds at `Interval`, the cavity gate (`cavity_embedded`)
  certifies the cavity embedded, and tier 3 passes.

**Cause.** `wall_clearance` in `crates/topo/src/shell.rs` (the drift
lever, `shell_walls_extent` and `let drift = (a.normal +
b.normal).norm() * lever`). The two faces are tilted off antiparallel,
so the gate:
- reads the pair as facing;
- takes the gap at the origins short by the full drift across the
  pair's extent (`gap − drift − 2t`);
- refuses, although the wall is at least 0.42 everywhere.

The gate's own docs call it conservative ("may refuse a body that would
have shelled"), so the refusal is typed and never a wrong body. It
predates SHELL-4.

## The trade-off

SHELL-4 measured both ways (PR #6183, item 2) and kept the planar
gates at every scalar.
- Skipping them where a lane answers lets this wedge, and the box with
  a dilated cylindrical void, reach the cavity gate. The box then
  refuses with a witness.
- But `lib_g17_r2_probes::p5_the_interval_witness_reports_the_declared_end_of_a_widened_parameter`
  loses its typed `WallClearance` and escalates in the offset door
  instead (`offset_together_concurrence`, `[-0.03125, 0.03125]`).

So a fix has to keep that typed refusal while letting the certified
cavity decide the tilted pair. For example:
- read the drift lever's refusal at certifying scalars as a deferral
  to the cavity gate;
- or tighten the lever to the pair's true separation.


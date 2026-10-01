---
id: ssi-march-newton-tolerance-has-no-resolvability-door
kind: issue
title: ssi/march: the march's Newton tolerance is ε-derived with no resolvability door, so a slab far from the origin fails as StepRefinementFailed rather than by its scale
status: closed
opened: 2026-10-01
priority: P1
cost: M
branch: ssi/march-endings
closed: 2026-10-01
pr: 3707
---


Found by the `ssi/chart-floor` lane, measuring its own ℝ³ floor row.

`ssi/exhaust.rs`'s floor door (`SweepFloor::mint`) refuses an
accounting or seeding floor narrower than the finest cell bisection can
cut at the slab's largest coordinate. The march has the same
precondition and no door for it: `ssi/march.rs`'s `newton_refine`
settles to `SSI_NEWTON_TOL`·ε (`MarchTol::from_band`), which no state
can reach once ε is below the spacing of floats at the slab's
coordinates.

Measured on the threaded cylinder × unit sphere of
`tests/m5_pr7_ssi.rs`, both translated to `x = 1e8` m (float spacing
1.49e-8 m), at ε = 1e-9, through `cylinder_sphere_ssi`:

- with the default floor (ε), the floor door refuses
  `FloorUnresolvable` by name (the row
  `a_floor_no_bisection_reaches_refuses_by_name_on_both_lanes`);
- with `floor_scale` naming a floor of `1e-7` or `1e-6` m, the floor
  door passes, and the march refuses
  `StepRefinementFailed { step_meters: 7.04e-4 }`: "the march lost the
  branch it was tracing". The cause is the scale, not the branch.

Before the chart-floor unit the same failure was
`SeedRefinementFailed`, which `cylinder_sphere_ssi` swallowed, so the
accounting pass answered in its place.

Fix shape: the march's tolerance gets the floor door's precondition at
the door that mints `MarchTol` (or the SSI doors check it against the
slab before marching), refusing by the slab's reach and the spacing
there.

## Closed (2026-10-01, PR 3707)

Not by a door on the tolerance: the march settles to what its coordinates resolve, `max(SSI_NEWTON_TOL·ε, c·gap(reach))`, with `c` measured per lane, and the reach cut to where the states can lie (the operands' boxes ∩ the domain). It refuses `SettlingUnresolvable` (naming the geometry or the domain) only when that target exceeds `SSI_SETTLE_MAX`·ε. A first door, which refused models that certified at base, was blocked by review and replaced. Measured base vs head: "lost the branch" is gone from every sweep, and nothing that certified at base regresses.

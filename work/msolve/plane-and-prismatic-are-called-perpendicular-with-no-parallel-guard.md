---
id: plane-and-prismatic-are-called-perpendicular-with-no-parallel-guard
kind: issue
title: The coset table calls a planar and a prismatic perpendicular with no parallel guard, so below the zero band a direction along the normal reads as a slide in the plane
status: open
opened: 2026-10-01
priority: P3
cost: E
---



## What

Found by MSOLVE-12's sweep of the coset fold's divisions.
`intersect_subgroups` (`crates/editor-core/src/mate/coset.rs`, entry
3, planar ∩ prismatic) answers `Prismatic { direction: d }` on
`perpendicular(d, n)` alone. Its sibling entry 2 (planar ∩
cylindrical) asks `parallel(u, n)` first, which is what keeps a
direction along the normal from reading as perpendicular. Without that
guard, an arm below `band.zero()` levers every cosine into the zero
band, so `d ∥ n` is called perpendicular. The translation stage then
slides across `d` within the plane along `b = d × n`, whose length is
zero there, and `b·Δ / |b|²` is `0/0`. The candidate refuses
`PoseOutOfRange` for a pair whose true answer is that the table cannot
decide it at that lever.

## Reach

No row reaches it. It needs a fold arm below the zero band (1e-9 m at
the witness ε), and a fold's arm is at least the two mated parts'
reach. A prismatic coset is minted only by a coaxial mate with a
clocking rider. The public `coset::intersect` takes any `Arm`,
`Arm::of(0.0, 0.0)` included.

## What it wants

Entry 3 asks `parallel(d, n)` first, as entry 2 does (`Trivial` when
parallel: a translation along the normal is never in the plane). Or
the lever gets a floor, which ERROR-DESIGN E3 declined.

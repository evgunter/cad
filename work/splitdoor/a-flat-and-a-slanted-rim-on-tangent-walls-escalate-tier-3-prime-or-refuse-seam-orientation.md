---
id: a-flat-and-a-slanted-rim-on-tangent-walls-escalate-tier-3-prime-or-refuse-seam-orientation
kind: issue
title: A flat and a slanted rim crossing on internally tangent walls: tier 3′ escalates on two ops, and the other slant refuses SeamOrientation
status: open
opened: 2026-10-30
priority: P1
cost: M
---


Found by `splitdoor/sheets-part-conic`'s fixture for
`a-boundary-crossing-tangent-reads-lines-and-circles-only-in-sheets-part`
(`crates/sweep/tests/sheets_part_on_slanted_rims.rs`). Measured at
ε = 1e-6, 1e-9 and 1e-12, on main and on that branch alike: the
`sheets_part` change moves neither finding.

## The fixture

An inner rod of radius 1 about `(1, 0)` is cut below `z = 0`. An outer
rod of radius 2 about the origin is cut above `z = y·tan t`. Each is a
two-arc extrude over `z ∈ [−3, 3]` whose vertices sit at `±y`. The
walls are tangent inside one another along `x = 2, y = 0`, and the
rims (a circle and an ellipse) cross at `(2, 0, 0)`, where the walls
part. Closed forms: `A = 3π`, `B = 12π`, and they share
`(2/3)|tan t|`.

## 1. Tier 3′ escalates on `A ∪ B` and `B ∖ A` (t = 0.3 and 1.0)

Every op builds at its closed form within its pad and passes tiers 1–3.
`A ∖ B` and `A ∩ B` pass tier 3′. On `A ∪ B`, `B ∪ A` and `B ∖ A`,
`validate_pseudomanifold` returns `CensusEscalated` with
`predicate: "curve_pair_coplanar_sub_side"`
(`geom_brep::curve_pair::pose`), at margins of about −2.04ε, 1.47ε and
1.78ε, at every ε. Those three bodies keep `B`'s slanted cap with the
pinch at `(2, 0, 0)`. Which pair of curved edges the census reads
there is not yet measured. Pinned in
`sheets_part_on_slanted_rims::a_flat_rim_crossing_a_slanted_rim_on_tangent_walls_builds_every_op`
(slow set).

## 2. `SeamOrientation` at t = −0.5

Every op (`A ∪ B`, `A ∖ B`, `B ∖ A`, `A ∩ B`, `B ∪ A`) refuses
`SeamOrientation`, the orientation chain's kernel-bug invariant. Its
raising site, measured, is `crates/topo/src/boolean/zip.rs`'s
ring-match arm (`None if leaves`): a seam half-edge that leaves the
face has no ring half-edge matching it. It is not the one-vertex sense
check below it. With the plane slanted the other way (t = 0.3, 1.0)
every op builds. Pinned in
`sheets_part_on_slanted_rims::a_flat_rim_crossing_a_rim_slanted_the_other_way_refuses_seam_orientation`.
The fix updates that pin.

**Why P1.** A boolean that refuses as a kernel bug on a legal input
(`work/README.md` Priority).

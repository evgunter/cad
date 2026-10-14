---
id: mass-properties-measures-a-flipped-seam-region-without-tier-3
kind: issue
title: mass_properties alone measures a flipped seam region's complement: the sphere flux arm reads each face's side alone
status: open
opened: 2026-10-11
priority: P3
cost: M
refs: [a-pair-of-seam-siblings-flipped-together-is-silent]
---

## Measured

Tier 3's check 6 names a sphere face whose own loop holds both poles
when its bit contradicts its seam region's side
(`crates/topo/src/validate.rs`, `seam_region`). `topo::mass_properties`
does not run tier 3 and accepts any `Body`. Its sphere arm
(`crates/geom-brep/src/props/curved.rs`, `sphere_face`) reads only the
face's own side (`side_of_sphere_loops`), which for such a face is
`Unencoded`, and so it measures under the bit alone. On the PR 4573
review's case #1 (`crates/sweep/tests/seam_siblings_flipped_together.rs`,
`PAIRS[0]`), with both siblings flipped, `mass_properties` returns
`0.3323929191408849` against a true `7.19366090655474`, with no
refusal. The same body is refused by name at tier 3.

An encoded face's contradicted bit refuses in the flux arm itself
(`SenseContradicted`, `SenseInverted`), so the door is consistent with
tier 3 everywhere except the seam region.

## Owner question

The flux arm is per-face and holds no body. The region read needs the
body walk in `topo::props` (`sign_walk`), so a fix is a region pass in
the walk refusing `SenseContradicted` for a member whose bit
contradicts its region, or a documented statement that the measurement
door trusts the bit where only a region encodes the side.


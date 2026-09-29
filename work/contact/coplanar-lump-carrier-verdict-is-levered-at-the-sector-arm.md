---
id: coplanar-lump-carrier-verdict-is-levered-at-the-sector-arm
kind: issue
title: The coplanar lump's carrier verdict (vtxfac carrier_eq, recl require_same) is levered at the sector's shorter arm, so a face whose far vertices stand 2500 bands off is called one carrier
status: open
opened: 2026-09-29
priority: P3
cost: M
---


Filed by CONTACT-9 (review MINOR-4). Pre-existing: main and CONTACT-9's
head refuse alike.

When a sector reads coplanar, the carrier identity it is lumped with is
decided at the sector's shorter arm:
- `vtxfac.rs` Delta 2 calls `carrier_eq(&sector_carrier,
  &pierced_carrier, id, s.arm, band)`;
- `recl.rs` `require_same(..., arm)` passes the pair's shorter arm.

`oriented_plane_eq`'s parallelism rung (`plane_eq.rs`) levers
`|n₁ × n₂|` at that arm. A face whose far vertices stand thousands of
bands off the other plane is then "the same plane" to the ladder.
Undeclared it refuses `UndeclaredCoincidence`, whose text says the
geometry coincides. Declared, it would be lumped.

**Witness P4 (the review's).**
- A prism of profile `(0,0), (1e-3,0), (10,5), (5,10), (0,1e-3)`,
  height 1, mapped by `(x+5, y+5, z + θ(x − y)/√2)` with
  `θ = off·ε·√2/1e-3`.
- Its bottom has 1 mm edges on the block's top at the pierce corner,
  and far vertices standing about ±2500·ε off.
- `intersect` and `subtract` refuse `UndeclaredCoincidence` at head and
  at main.

**The fix's shape.** Lever the carrier verdict at the face's extent, or
decide it from the face's own vertices in metres, as CONTACT-9 did for
the side codes. This touches the same arm as the `tang` row's door lever
(`work/tang/torus-carrier-axis-margin-is-levered-by-one-not-the-ring`).

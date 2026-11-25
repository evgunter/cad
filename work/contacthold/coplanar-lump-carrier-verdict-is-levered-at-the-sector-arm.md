---
id: coplanar-lump-carrier-verdict-is-levered-at-the-sector-arm
kind: issue
title: The coplanar lump's carrier verdict (vtxfac carrier_eq, recl require_same) is levered at the sector's shorter arm, so a face whose far vertices stand 2500 bands off is called one carrier
status: closed
opened: 2026-09-29
priority: P3
cost: M
closed: 2026-11-25
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

Moved from SECTOR to CONTACTHOLD on 2026-11-20, by `git mv`, when SECTOR merged into CONTACTHOLD: SECTOR held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one CONTACTHOLD carries.

## Closed

Closed 2026-11-25 as **obsolete** (its premise is gone), in the CONTACTHOLD cleanup (`docs/doc-ledger/contacthold-leaves-the-tracker.md`), against main at `6003877ad`. The sector-arm carrier ladder is gone. `recl::require_same` takes no `arm`: it reads the glue door's recorded relation or refuses `unglued_corner` (`crates/topo/src/boolean/recl.rs:59-112`). The vtxfac Delta-2 site reads `declared.glued_relation(…)` with no `carrier_eq` (`boolean/vtxfac.rs:1085-1095`). This is PR #6079: "consumers (recl, vtxfac, …) read the glue table's relation and run no ladder" Carrier identity is now decided once, by the glue door, at the faces' consumed extent. If the P4 witness still lumps wrongly, that is a new row about the glue door or `unglued_corner`, not this one.

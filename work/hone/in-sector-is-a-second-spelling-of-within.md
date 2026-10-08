---
id: in-sector-is-a-second-spelling-of-within
kind: issue
title: sectors::in_sector and sectors::within read a direction's membership of a sector twice, at different levers
status: open
opened: 2026-10-08
priority: P3
cost: M
refs: [pierce-germ-direction-within-is-levered-at-the-sector-arm]
---



Filed by PR 4289's second review (S1). Filed on TANG and moved here by
its third (m4): `sectors.rs` is CLEAVE, GERM, HONE and REACH ground, and
one reading with two spellings is HONE's "one home per rule".

## What

Two readers in `crates/topo/src/boolean/sectors.rs` decide whether a
direction lies within a face's sector at a vertex:

- `within`: the sine of the direction past each bound plane, levered
  at the sector's arm (`bool_sector_within`), with a strict mode, and a
  decided zero counted within. It is read by `pair_search`,
  `sector_overlap`, `runs_into`, and `vtxfac`'s pierce germ.
- `in_sector`: the same two sines plus the cosine to the sector's
  middle (`bool_cone_within`, `bool_cone_facing`), a decided zero
  counted within as `within` counts it, each levered at the least
  deviation of the points it reads (`least_lever`): the direction at
  the caller's lever, and each bound at its own reach. It is read by
  `cone_side`.

`in_sector` exists because `within` reads a direction opposite a thin
sector as inside it. Its lever differs too. By D4 a reading past a
bound flips at the least move of the direction's far point or of that
bound's far point, so it belongs at `min(direction's reach, L_u)`, per
bound. `within` reads both bounds at the sector's arm, the shorter
chord, which can be the other bound's. PR 4289's first fix pass read
the direction's reach alone, which dropped the bound's term: that was
the defect its third review found, not a fix. CONTACT's
`pierce-germ-direction-within-is-levered-at-the-sector-arm` is the
same lever, seen from the pierce germ.

## The shape to give

Make one reader with the direction's lever as a parameter, the bound
terms and the facing reading included. Then make `within` its sugar. Each caller
must be shown bit-identical, or moved with its rows. It is not done in
PR 4289 because `within`'s callers decide under `bool_sector_within`,
whose name the K telemetry and `offer_rows` key on, and the facing
reading would change what a thin sector reads for them.

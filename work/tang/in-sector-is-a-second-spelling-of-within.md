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



Filed by PR 4289's second review (S1).

## What

Two readers in `crates/topo/src/boolean/sectors.rs` decide whether a
direction lies within a face's sector at a vertex:

- `within`: the sine of the direction past each bound plane, levered
  at the sector's arm (`bool_sector_within`), with a strict mode, and a
  decided zero counted within. It is read by `pair_search`,
  `sector_overlap`, `runs_into`, and `vtxfac`'s pierce germ.
- `in_sector`: the same two sines plus the cosine to the sector's
  middle, all levered at the caller's lever (`bool_cone_within`,
  `bool_cone_facing`), a decided zero counted within as `within`
  counts it. It is read by `cone_side`.

`in_sector` exists because `within` reads a direction opposite a thin
sector as inside it, and its arm lever reads a direction off a
short-armed sector as on its bound. CONTACT's
`pierce-germ-direction-within-is-levered-at-the-sector-arm` is the same
lever, seen from the pierce germ.

## The shape to give

Make one reader with the lever as a parameter and the facing reading
included. Then make `within` its sugar at the sector's arm. Each caller
must be shown bit-identical, or moved with its rows. It is not done in
PR 4289 because `within`'s callers decide under `bool_sector_within`,
whose name the K telemetry and `offer_rows` key on, and the facing
reading would change what a thin sector reads for them.

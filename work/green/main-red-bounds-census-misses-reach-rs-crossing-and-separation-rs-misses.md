---
id: main-red-bounds-census-misses-reach-rs-crossing-and-separation-rs-misses
kind: issue
title: Main red: bounds_census reports blend/reach.rs's crossing and topo separation.rs's misses as sole-bracket-bound doors missing from its roster
status: open
opened: 2026-11-25
---


Found by the `sectarms/one-image-function` lane (PR 7537), whose diff
edits `crates/geom-core/tests/bounds_census.rs` and so runs it per PR.

`geom-core::all bounds_census::every_sole_bracket_bound_door_is_in_the_roster`
is red on main at `8174716d36`'s merge base and at `origin/main` on
2026-11-25: "arrived or moved" names

- `crates/sweep/src/blend/reach.rs`: `crossing`
- `crates/topo/src/separation.rs`: `misses`

Neither has a roster line in `bounds_census.rs` on `origin/main`.
`work/doctail/main-red-bounds-census-misses-reach-rs-crossing-and-sure-below.md`
is closed and named `crossing`; the test still reports it. Hosted runs:
PR 7537's `test (kernel)` jobs on heads `68e8d3c249` and `8174716d36`.

## Done when

The roster holds both doors with their dispositions, or the doors are
no longer sole-bracket bounds.

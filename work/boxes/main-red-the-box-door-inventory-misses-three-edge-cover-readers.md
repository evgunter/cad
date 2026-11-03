---
id: main-red-the-box-door-inventory-misses-three-edge-cover-readers
kind: issue
title: Main red: the box-door inventory pin misses three edge-cover readers since 928e6189
status: open
opened: 2026-11-03
priority: P0
cost: E
---


Found by RESTREAD's self-touch second-order lane (PR 6422) on topo's
slow set at the default ε, and confirmed on main at `cba0a8ed6e` by
counting the door calls the pin counts.

## What

`boolean::boxes::tests::every_door_that_reads_a_box_is_inventoried`
(`crates/topo/src/boolean/boxes.rs:3513`) is red on main. Its `PINNED`
roster has no entry for three files that now call the edge-cover doors:

- `shell.rs` — 2 calls of `edge_cover_linked` (`shell.rs:4490`,
  `shell.rs:5109`);
- `splitting/containment.rs` — 1 call of `edge_cover`
  (`splitting/containment.rs:1410`);
- `boolean/ruled_region.rs` — 1 call of `edge_cover`
  (`boolean/ruled_region.rs:377`).

All three arrived with `5a031b3c94` ("topo reads one edge cover"),
merged in `928e6189d9`. The pin is in the slow set, so the per-PR gate
did not run it.

## Fix

Add the three entries to `PINNED` and, as the assertion asks, give
each door's direction (pruning or refusing) in the module docs' DOOR
list; read S234 first. RESTREAD touches none of these files.

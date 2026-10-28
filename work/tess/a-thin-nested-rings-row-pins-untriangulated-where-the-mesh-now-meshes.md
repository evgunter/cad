---
id: a-thin-nested-rings-row-pins-untriangulated-where-the-mesh-now-meshes
kind: issue
title: thin_nested_rings_build pins every op untriangulated, but the mesh now meshes them
status: closed
opened: 2026-10-26
priority: P2
cost: E
closed: 2026-10-28
---


Filed by RESTREAD (PR 5704's local sweep run), outside its fence.

## What

`an_annular_tube_through_a_plate::thin_nested_rings_build`
(`crates/sweep/tests/an_annular_tube_through_a_plate.rs:230`) pins
`untriangulated: &ALL_OPS` at `:241-243`. At the default ε every op at
`w = 0.001` now tessellates, so `every_op`
(`crates/sweep/tests/an_annular_tube_through_a_plate.rs:125-131`) pushes
`the mesh Ok("meshed")` for all six runs and the row panics at `:146`.
The volume and `outcome` lines pass. Only the mesh pin moved.

## Why it is main's

- The row is in the sweep slow set (`.config/nextest.toml`), so a PR
  run does not reach it; only the nightly does.
- It passed on PR 5704's head 94bbbd9. Between that and the merge of
  main at 173271c, main brought `crates/mesh/src` changes: PR 5717
  (cf0c114, "mesh sizes each lane at its own worst-triangle theorem")
  and its fix passes. PR 5704 touches only the census, which
  tessellation does not call.
- Confirmed on origin/main `ea88d599ea` by PR 5704's second verifier: red at both ε, `the mesh Ok("meshed")`.

## Fix

Check that the thin rings mesh soundly (`check_mesh` passes at
`w = 1e-3`, `9e-4`, `1e-4`), then narrow or empty the row's
`untriangulated` list, re-baselining what moved.

## Closed (2026-10-28, duplicate)

The same finding as `main-red-thin-nested-rings-build-meshes-the-rings-its-pin-expects-refused` (P1), filed the same day by another lane. That row carries the fix; this row's evidence (the PR 5717 bisection pointer) is read there.

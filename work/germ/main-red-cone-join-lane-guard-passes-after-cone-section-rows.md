---
id: main-red-cone-join-lane-guard-passes-after-cone-section-rows
kind: issue
title: Main is red: cone_join_lane's three refusal rows fail because the cone section rows (PR 4352) satisfied the guard the cone join door (PR 4375) pins as refusing
status: open
opened: 2026-10-09
priority: P0
cost: E
---


Filed by JOIN's orchestrator, which found it while CI on PR 4399 was red.

## What

On main (b1a15ad0 and later), `sweep::all cone_join_lane` fails three of its four rows, in every ε leg:
- `the_tilted_planes_chords_are_its_ellipse_in_both_member_orders`
- `the_rings_island_on_the_cone_face_winds_by_the_sections_curve`
- `the_slabs_chords_are_its_faces_circles_on_both_sides`

Each panics at `crates/sweep/tests/cone_join_lane.rs:114` for B4, C1 and T1 (Union, true): "the interior-loop guard refuses the cone pair until its certificate has cone rows, got Ok(())".

## Why (likely; not bisected)

Two GERM PRs crossed:
- PR 4375 (`germ/cone-sector-join`) landed the suite and the guard pin; its head `a4769e8cf7` carries `b4370dbbb2`.
- PR 4352 (`germ/cone-section-rows`) merged 17 minutes earlier and adds cone rows to the certificate.

Neither branch saw the other, so on main the guard's condition is met, the union builds, and the pin to "refuses" is stale.

## Owed

Decide whether the cone unions now build soundly: run the rows through `differential::outcome` and `check_mesh`. If they do, re-pin the three rows to build, against their closed forms. If they don't, restore the refusal.

Every open PR that merges main inherits the red until this lands; JOIN's 4399 is one.
